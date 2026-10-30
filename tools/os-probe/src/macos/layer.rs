//! The button pass's pixel work: a capture decoded to 8-bit RGBA, a
//! button's own layer (colour and alpha) solved from two captures over
//! known opaque backdrops and checked against a third, and what is read off a
//! solved layer: its fill, its bezel's bounds and corner, its ink and what
//! it draws outside the button's frame.
//!
//! A capture over an opaque backdrop `B` holds `C = P + (1 - a) B` per
//! channel, `P` the layer's premultiplied colour and `a` its alpha; two
//! backdrops give `a = 1 - (C_white - C_black) / (B_white - B_black)` and
//! `P = C_black - (1 - a) B_black`. The backdrops are the bytes the captures
//! read at a point of the backdrop, not the colours the probe asked for.

use std::collections::HashMap;

use objc2_app_kit::{NSBitmapFormat, NSBitmapImageRep};
use objc2_foundation::NSRect;
use toml::{Table, Value};

use super::ffi;
use crate::out;

/// The alpha below which a solved pixel counts as empty, in 0.0..=1.0.
const EMPTY_ALPHA: f64 = 8.0 / 255.0;

/// How far a pixel's colour (any channel, straight alpha, 0..=255) or alpha
/// (times 255) must differ from the fill for the pixel to count as ink.
const INK_DELTA: f64 = 48.0;

/// The most rows a corner profile reads.
const CORNER_ROWS: usize = 40;

/// The coverage at which a corner profile's row reaches the bezel's body.
const BODY_COVERAGE: f64 = 0.98;

/// How far from the body's alpha a pixel's alpha may be for the fill.
const BODY_ALPHA_TOLERANCE: f64 = 8.0 / 255.0;

/// A capture as 8-bit RGBA, rows top to bottom.
pub(super) struct Rgba {
    pub(super) width: usize,
    pub(super) height: usize,
    pixels: Vec<[u8; 4]>,
}

impl Rgba {
    /// The capture's pixels as RGBA bytes, alpha first or last as its format
    /// says, `ff` where it has no alpha.
    pub(super) fn from_rep(rep: &NSBitmapImageRep) -> Result<Self, String> {
        let raw = ffi::bitmap_buffer(rep)?;
        let to_usize =
            |v: isize, what: &str| usize::try_from(v).map_err(|_| format!("{what} is {v}"));
        let width = to_usize(rep.pixelsWide(), "pixelsWide")?;
        let height = to_usize(rep.pixelsHigh(), "pixelsHigh")?;
        let row = to_usize(rep.bytesPerRow(), "bytesPerRow")?;
        let pixel = to_usize(rep.bitsPerPixel(), "bitsPerPixel")?
            .checked_div(8)
            .filter(|p| *p > 0)
            .ok_or_else(|| "no pixel size".to_owned())?;
        let samples = to_usize(rep.samplesPerPixel(), "samplesPerPixel")?;
        let alpha_first = rep.bitmapFormat().contains(NSBitmapFormat::AlphaFirst);
        let mut pixels = Vec::with_capacity(width.saturating_mul(height));
        for line in raw.chunks_exact(row.max(1)).take(height) {
            for px in line.chunks_exact(pixel).take(width) {
                let rgba = match (px.get(..samples), alpha_first) {
                    (Some(&[a, r, g, b]), true) | (Some(&[r, g, b, a]), false) => [r, g, b, a],
                    (Some(&[r, g, b]), _) => [r, g, b, 0xff],
                    _ => return Err(format!("{samples} samples in {pixel} bytes a pixel")),
                };
                pixels.push(rgba);
            }
        }
        if pixels.len() != width.saturating_mul(height) {
            return Err(format!(
                "the buffer holds {} of {width} x {height} px",
                pixels.len()
            ));
        }
        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    /// The pixel at (`x`, `y`).
    pub(super) fn pixel(&self, x: usize, y: usize) -> Option<[u8; 4]> {
        if x >= self.width {
            return None;
        }
        self.pixels
            .get(y.checked_mul(self.width)?.checked_add(x)?)
            .copied()
    }
}

/// A rectangle of pixels, `x0..x1` by `y0..y1`, from the top left.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct PxRect {
    pub(super) x0: usize,
    pub(super) y0: usize,
    pub(super) x1: usize,
    pub(super) y1: usize,
}

impl PxRect {
    /// The pixels holding `rect` (points from the window frame's top left)
    /// grown by `grow` points on each side, at `ratio` pixels per point,
    /// clamped to `width` x `height`; `None` where empty.
    pub(super) fn from_points(
        rect: NSRect,
        grow: f64,
        ratio: f64,
        width: usize,
        height: usize,
    ) -> Option<Self> {
        let (x0, x1) = px_span(rect.origin.x, rect.size.width, grow, ratio, width)?;
        let (y0, y1) = px_span(rect.origin.y, rect.size.height, grow, ratio, height)?;
        Some(Self { x0, y0, x1, y1 })
    }

    pub(super) fn contains(&self, x: usize, y: usize) -> bool {
        (self.x0..self.x1).contains(&x) && (self.y0..self.y1).contains(&y)
    }

    pub(super) fn width(&self) -> usize {
        self.x1.saturating_sub(self.x0)
    }

    pub(super) fn height(&self) -> usize {
        self.y1.saturating_sub(self.y0)
    }

    /// `[x, y, width, height]` in pixels.
    pub(super) fn value(&self) -> Value {
        out::array(
            [self.x0, self.y0, self.width(), self.height()]
                .iter()
                .map(|v| usize_value(*v))
                .collect(),
        )
    }

    /// `[x, y, width, height]` in points at `ratio` pixels per point.
    fn points_value(&self, ratio: f64) -> Value {
        out::floats(&[
            usize_f64(self.x0) / ratio,
            usize_f64(self.y0) / ratio,
            usize_f64(self.width()) / ratio,
            usize_f64(self.height()) / ratio,
        ])
    }

    /// The smallest rectangle holding this one and the pixel (`x`, `y`).
    fn include(self, x: usize, y: usize) -> Self {
        Self {
            x0: self.x0.min(x),
            y0: self.y0.min(y),
            x1: self.x1.max(x.saturating_add(1)),
            y1: self.y1.max(y.saturating_add(1)),
        }
    }

    /// The rectangle of the one pixel (`x`, `y`).
    fn pixel(x: usize, y: usize) -> Self {
        Self {
            x0: x,
            y0: y,
            x1: x.saturating_add(1),
            y1: y.saturating_add(1),
        }
    }

    /// `bounds` grown to hold the pixel (`x`, `y`).
    fn grow(bounds: Option<Self>, x: usize, y: usize) -> Option<Self> {
        Some(bounds.map_or_else(|| Self::pixel(x, y), |b| b.include(x, y)))
    }
}

/// The pixel range of `start..start + length` points grown by `grow` at
/// `ratio` pixels per point, clamped to `0..limit`; `None` where empty.
fn px_span(start: f64, length: f64, grow: f64, ratio: f64, limit: usize) -> Option<(usize, usize)> {
    let limit_f = usize_f64(limit);
    let lo = ((start - grow) * ratio).floor().clamp(0.0, limit_f);
    let hi = ((start + length + grow) * ratio).ceil().clamp(0.0, limit_f);
    if !(lo.is_finite() && hi.is_finite()) || hi <= lo {
        return None;
    }
    // Both in 0.0..=limit after the clamp, so the casts are exact.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let range = (lo as usize, hi as usize);
    Some(range)
}

#[allow(clippy::cast_precision_loss)]
fn usize_f64(v: usize) -> f64 {
    v as f64
}

fn usize_value(v: usize) -> Value {
    i64::try_from(v).map_or_else(|_| out::text(v.to_string()), out::int)
}

/// `#rrggbbaa` of an RGBA pixel's bytes.
pub(super) fn hex(c: [u8; 4]) -> String {
    let [r, g, b, a] = c;
    format!("#{r:02x}{g:02x}{b:02x}{a:02x}")
}

/// A pixel's RGB bytes as 0.0..=1.0.
fn unit_rgb(c: [u8; 4]) -> [f64; 3] {
    let [r, g, b, _] = c;
    [
        f64::from(r) / 255.0,
        f64::from(g) / 255.0,
        f64::from(b) / 255.0,
    ]
}

/// A solved layer: per pixel the premultiplied colour and the alpha, each
/// in 0.0..=1.0.
pub(super) struct Layer {
    width: usize,
    height: usize,
    px: Vec<[f64; 4]>,
}

impl Layer {
    /// Solve the layer from `black` and `white`, the same window captured
    /// over two opaque backdrops whose captured bytes are `black_bg` and
    /// `white_bg`; how well the per-channel solutions agree.
    pub(super) fn solve(
        black: &Rgba,
        white: &Rgba,
        black_bg: [u8; 4],
        white_bg: [u8; 4],
    ) -> Result<(Self, Table), String> {
        if (black.width, black.height) != (white.width, white.height) {
            return Err("the captures differ in size".to_owned());
        }
        let bb = unit_rgb(black_bg);
        let bw = unit_rgb(white_bg);
        let spans = [bw[0] - bb[0], bw[1] - bb[1], bw[2] - bb[2]];
        if spans.iter().any(|s| *s < 1.0 / 255.0) {
            return Err(format!(
                "the backdrops {} and {} do not differ in every channel",
                hex(black_bg),
                hex(white_bg)
            ));
        }
        let mut px = Vec::with_capacity(black.pixels.len());
        let mut spread_max = 0.0_f64;
        let mut spread_over_2 = 0_usize;
        let mut out_of_range = 0_usize;
        let mut not_opaque = 0_usize;
        for (b, w) in black.pixels.iter().zip(white.pixels.iter()) {
            if b[3] != 0xff || w[3] != 0xff {
                not_opaque = not_opaque.saturating_add(1);
            }
            let cb = unit_rgb(*b);
            let cw = unit_rgb(*w);
            let alphas = [
                1.0 - (cw[0] - cb[0]) / spans[0],
                1.0 - (cw[1] - cb[1]) / spans[1],
                1.0 - (cw[2] - cb[2]) / spans[2],
            ];
            let lo = alphas[0].min(alphas[1]).min(alphas[2]);
            let hi = alphas[0].max(alphas[1]).max(alphas[2]);
            let spread = (hi - lo) * 255.0;
            spread_max = spread_max.max(spread);
            if spread > 2.0 {
                spread_over_2 = spread_over_2.saturating_add(1);
            }
            let mean = (alphas[0] + alphas[1] + alphas[2]) / 3.0;
            if !(-1.0 / 255.0..=1.0 + 1.0 / 255.0).contains(&mean) {
                out_of_range = out_of_range.saturating_add(1);
            }
            let a = mean.clamp(0.0, 1.0);
            let premultiplied = |c: usize| -> f64 {
                let (Some(cbc), Some(bbc)) = (cb.get(c), bb.get(c)) else {
                    return 0.0;
                };
                (cbc - (1.0 - a) * bbc).clamp(0.0, a)
            };
            px.push([premultiplied(0), premultiplied(1), premultiplied(2), a]);
        }
        let mut t = Table::new();
        out::put(&mut t, "backdrop_black_raw", out::text(hex(black_bg)));
        out::put(&mut t, "backdrop_white_raw", out::text(hex(white_bg)));
        out::put(
            &mut t,
            "alpha_channel_spread_max_bytes",
            out::float(round3(spread_max)),
        );
        out::put(
            &mut t,
            "alpha_channel_spread_over_2_pixels",
            usize_value(spread_over_2),
        );
        out::put(
            &mut t,
            "alpha_out_of_range_pixels",
            usize_value(out_of_range),
        );
        out::put(&mut t, "capture_not_opaque_pixels", usize_value(not_opaque));
        Ok((
            Self {
                width: black.width,
                height: black.height,
                px,
            },
            t,
        ))
    }

    fn at(&self, x: usize, y: usize) -> Option<[f64; 4]> {
        if x >= self.width {
            return None;
        }
        self.px
            .get(y.checked_mul(self.width)?.checked_add(x)?)
            .copied()
    }

    /// The pixel's alpha, `None` outside the layer.
    fn alpha(&self, x: usize, y: usize) -> Option<f64> {
        self.at(x, y).map(|p| p[3])
    }

    /// The pixel's straight-alpha colour and alpha as bytes.
    fn straight(&self, x: usize, y: usize) -> Option<[u8; 4]> {
        let [r, g, b, a] = self.at(x, y)?;
        let un = |c: f64| if a > 0.0 { out::unit_to_byte(c / a) } else { 0 };
        Some([un(r), un(g), un(b), out::unit_to_byte(a)])
    }

    /// The rectangle `r` of the layer as PNG bytes, straight alpha.
    pub(super) fn png(&self, r: PxRect) -> Result<Vec<u8>, String> {
        let mut pixels = Vec::with_capacity(r.width().saturating_mul(r.height()).saturating_mul(4));
        for y in r.y0..r.y1 {
            for x in r.x0..r.x1 {
                let p = self
                    .straight(x, y)
                    .ok_or_else(|| format!("({x}, {y}) lies outside the layer"))?;
                pixels.extend_from_slice(&p);
            }
        }
        ffi::rgba_png(r.width(), r.height(), &pixels)
    }

    /// The layer's own rectangle.
    pub(super) fn bounds(&self) -> PxRect {
        PxRect {
            x0: 0,
            y0: 0,
            x1: self.width,
            y1: self.height,
        }
    }

    /// The layer composited over `backdrop` (the bytes `capture` reads at a
    /// point of its backdrop) against `capture`, inside `r`: the largest
    /// difference of one byte and the pixels where one exceeds 2; the
    /// capture's pixels that are not opaque (the window's rounded corners)
    /// are counted apart and not compared.
    pub(super) fn check(&self, capture: &Rgba, backdrop: [u8; 4], r: PxRect) -> Table {
        let bg = unit_rgb(backdrop);
        let mut max_delta = 0.0_f64;
        let mut over_2 = 0_usize;
        let mut pixels = 0_usize;
        let mut not_opaque = 0_usize;
        for y in r.y0..r.y1 {
            for x in r.x0..r.x1 {
                let (Some(p), Some(c)) = (self.at(x, y), capture.pixel(x, y)) else {
                    continue;
                };
                if c[3] != 0xff {
                    not_opaque = not_opaque.saturating_add(1);
                    continue;
                }
                pixels = pixels.saturating_add(1);
                let measured = unit_rgb(c);
                let delta = (0..3)
                    .filter_map(|ch| {
                        let predicted = p.get(ch)? + (1.0 - p[3]) * bg.get(ch)?;
                        Some(((predicted - measured.get(ch)?) * 255.0).abs())
                    })
                    .fold(0.0_f64, f64::max);
                max_delta = max_delta.max(delta);
                if delta > 2.0 {
                    over_2 = over_2.saturating_add(1);
                }
            }
        }
        let mut t = Table::new();
        out::put(&mut t, "backdrop_raw", out::text(hex(backdrop)));
        out::put(&mut t, "rect_px_xywh", r.value());
        out::put(&mut t, "pixels", usize_value(pixels));
        out::put(&mut t, "not_opaque_pixels", usize_value(not_opaque));
        out::put(&mut t, "max_byte_delta", out::float(round3(max_delta)));
        out::put(&mut t, "pixels_over_2", usize_value(over_2));
        t
    }

    /// What the layer draws for one button: `frame` its frame, `region` the
    /// frame grown by the margin the captures compare, in pixels at `ratio`
    /// pixels per point.
    pub(super) fn measures(&self, frame: PxRect, region: PxRect, ratio: f64) -> Table {
        let mut t = Table::new();
        out::put(&mut t, "frame_px_xywh", frame.value());
        out::put(&mut t, "region_px_xywh", region.value());

        let mut alpha_max = 0.0_f64;
        let mut drawn = 0_usize;
        for y in frame.y0..frame.y1 {
            for x in frame.x0..frame.x1 {
                let Some(a) = self.alpha(x, y) else {
                    continue;
                };
                alpha_max = alpha_max.max(a);
                if a >= EMPTY_ALPHA {
                    drawn = drawn.saturating_add(1);
                }
            }
        }
        out::put(&mut t, "alpha_max", out::float(round3(alpha_max)));
        out::put(&mut t, "drawn_pixels_in_frame", usize_value(drawn));
        let middle = frame
            .y0
            .saturating_add(frame.height().checked_div(2).unwrap_or_default());
        let Some(body_alpha) = self.row_median(frame.x0, frame.x1, middle) else {
            out::put(
                &mut t,
                "fill",
                out::unavailable("nothing drawn on the frame's middle row"),
            );
            self.outside(&mut t, frame, region);
            return t;
        };
        out::put(&mut t, "body_alpha", out::float(round3(body_alpha)));

        let mut counts: HashMap<[u8; 4], usize> = HashMap::new();
        let mut body = 0_usize;
        for y in frame.y0..frame.y1 {
            for x in frame.x0..frame.x1 {
                let (Some(a), Some(s)) = (self.alpha(x, y), self.straight(x, y)) else {
                    continue;
                };
                if (a - body_alpha).abs() <= BODY_ALPHA_TOLERANCE {
                    body = body.saturating_add(1);
                    let n = counts.entry(s).or_insert(0);
                    *n = n.saturating_add(1);
                }
            }
        }
        let fill = counts
            .iter()
            .max_by_key(|(colour, n)| (**n, **colour))
            .map(|(colour, n)| (*colour, *n));
        let Some((fill_mode, fill_count)) = fill else {
            out::put(
                &mut t,
                "fill",
                out::unavailable("no pixel at the body's alpha"),
            );
            self.outside(&mut t, frame, region);
            return t;
        };
        out::put(&mut t, "fill_mode", out::text(hex(fill_mode)));
        out::put(&mut t, "fill_mode_pixels", usize_value(fill_count));
        out::put(&mut t, "body_pixels", usize_value(body));

        let mut bezel: Option<PxRect> = None;
        for y in region.y0..region.y1 {
            for x in region.x0..region.x1 {
                if self.alpha(x, y).is_some_and(|a| a >= 0.5 * body_alpha) {
                    bezel = PxRect::grow(bezel, x, y);
                }
            }
        }
        if let Some(bezel) = bezel {
            out::put(&mut t, "bezel_px_xywh", bezel.value());
            out::put(&mut t, "bezel_pt_xywh", bezel.points_value(ratio));
            let fill = match self.fill_point(bezel) {
                Some((x, y, colour)) => {
                    out::put(&mut t, "fill", out::text(hex(colour)));
                    out::put(
                        &mut t,
                        "fill_point_px_xy",
                        out::array(vec![usize_value(x), usize_value(y)]),
                    );
                    colour
                }
                None => {
                    out::put(
                        &mut t,
                        "fill",
                        out::unavailable("no fully covered pixel on the bezel's middle row"),
                    );
                    fill_mode
                }
            };
            for (key, from_left) in [("corner_top_left", true), ("corner_top_right", false)] {
                let profile = self.corner_profile(bezel, from_left);
                let mut c = Table::new();
                out::put(
                    &mut c,
                    "insets_px",
                    out::floats(&profile.iter().copied().map(round3).collect::<Vec<_>>()),
                );
                match fit_radius(&profile) {
                    Some((radius, rms)) => {
                        out::put(&mut c, "radius_fit_px", out::float(round3(radius)));
                        out::put(&mut c, "radius_fit_pt", out::float(round3(radius / ratio)));
                        out::put(&mut c, "fit_rms_px", out::float(round3(rms)));
                    }
                    None => out::put(&mut c, "radius_fit_px", out::unavailable("no corner rows")),
                }
                out::put_table(&mut t, key, c);
            }
            self.ink(&mut t, bezel, fill, body_alpha, ratio);
        }
        self.outside(&mut t, frame, region);
        t
    }

    /// The fill's sample point and colour: on the bezel's middle row, two
    /// pixels inside the first one from the bezel's left edge whose alpha
    /// reaches `BODY_COVERAGE` of the row's median alpha (clear of the corner
    /// and, the label being padded, of the label).
    fn fill_point(&self, bezel: PxRect) -> Option<(usize, usize, [u8; 4])> {
        let y = bezel
            .y0
            .saturating_add(bezel.height().checked_div(2).unwrap_or_default());
        let median = self.row_median(bezel.x0, bezel.x1, y)?;
        let first = (bezel.x0..bezel.x1).find(|x| {
            self.alpha(*x, y)
                .is_some_and(|a| a >= BODY_COVERAGE * median)
        })?;
        let x = first.saturating_add(2);
        if x >= bezel.x1 {
            return None;
        }
        Some((x, y, self.straight(x, y)?))
    }

    /// The median alpha of the drawn pixels (alpha >= `EMPTY_ALPHA`) of row
    /// `y` from `x0` to `x1`.
    fn row_median(&self, x0: usize, x1: usize, y: usize) -> Option<f64> {
        let mut alphas: Vec<f64> = (x0..x1)
            .filter_map(|x| self.alpha(x, y))
            .filter(|a| *a >= EMPTY_ALPHA)
            .collect();
        alphas.sort_by(f64::total_cmp);
        alphas
            .get(alphas.len().checked_div(2).unwrap_or_default())
            .copied()
    }

    /// The top corner's profile on one side: for each row from the bezel's
    /// top, the pixels' missing coverage (1 - alpha / the row's median
    /// alpha, clamped) summed from the bezel's edge inward up to the first
    /// pixel at `BODY_COVERAGE`; it ends after two rows under 0.02, at half
    /// the bezel's height or after `CORNER_ROWS`.
    fn corner_profile(&self, bezel: PxRect, from_left: bool) -> Vec<f64> {
        let mut profile = Vec::new();
        let half = bezel.width().checked_div(2).unwrap_or_default();
        let mut flat_rows = 0_u32;
        let rows = CORNER_ROWS.min(bezel.height().div_ceil(2));
        for y in (bezel.y0..bezel.y1).take(rows) {
            let Some(fill_alpha) = self.row_median(bezel.x0, bezel.x1, y) else {
                profile.push(usize_f64(half));
                continue;
            };
            let mut inset = 0.0_f64;
            for i in 0..half {
                let x = if from_left {
                    bezel.x0.saturating_add(i)
                } else {
                    bezel.x1.saturating_sub(1).saturating_sub(i)
                };
                let Some(a) = self.alpha(x, y) else { break };
                let coverage = (a / fill_alpha).clamp(0.0, 1.0);
                if coverage >= BODY_COVERAGE {
                    break;
                }
                inset += 1.0 - coverage;
            }
            profile.push(inset);
            if inset < 0.02 {
                flat_rows = flat_rows.saturating_add(1);
                if flat_rows >= 2 {
                    break;
                }
            } else {
                flat_rows = 0;
            }
        }
        profile
    }

    /// The pixels of `bezel` of alpha at least half `body_alpha` whose
    /// straight colour or alpha differs from `fill` by more than `INK_DELTA`:
    /// their bounds and the one farthest from the fill.
    fn ink(&self, t: &mut Table, bezel: PxRect, fill: [u8; 4], body_alpha: f64, ratio: f64) {
        let mut bounds: Option<PxRect> = None;
        let mut count = 0_usize;
        let mut farthest: Option<([u8; 4], f64)> = None;
        for y in bezel.y0..bezel.y1 {
            for x in bezel.x0..bezel.x1 {
                let (Some(a), Some(s)) = (self.alpha(x, y), self.straight(x, y)) else {
                    continue;
                };
                if a < 0.5 * body_alpha {
                    continue;
                }
                let deltas: Vec<f64> = s
                    .iter()
                    .zip(fill.iter())
                    .map(|(p, q)| f64::from(*p) - f64::from(*q))
                    .collect();
                if !deltas.iter().any(|d| d.abs() > INK_DELTA) {
                    continue;
                }
                count = count.saturating_add(1);
                bounds = PxRect::grow(bounds, x, y);
                let distance = deltas.iter().map(|d| d * d).sum::<f64>();
                if farthest.is_none_or(|(_, best)| distance > best) {
                    farthest = Some((s, distance));
                }
            }
        }
        let mut ink = Table::new();
        out::put(&mut ink, "pixels", usize_value(count));
        if let Some(b) = bounds {
            out::put(&mut ink, "bounds_px_xywh", b.value());
            out::put(&mut ink, "bounds_pt_xywh", b.points_value(ratio));
        }
        if let Some((colour, _)) = farthest {
            out::put(&mut ink, "farthest_from_fill", out::text(hex(colour)));
        }
        out::put_table(t, "ink", ink);
    }

    /// Where this layer differs from `base` inside `region`: the pixels
    /// whose premultiplied colour or alpha differs by more than 2 of 255,
    /// their bounds, and the straight colour of the pixel whose alpha grew
    /// most (a focus ring's colour where it covers a whole pixel).
    pub(super) fn diff(&self, base: &Layer, region: PxRect, ratio: f64) -> Table {
        let mut bounds: Option<PxRect> = None;
        let mut count = 0_usize;
        let mut grown: Option<([u8; 4], f64)> = None;
        for y in region.y0..region.y1 {
            for x in region.x0..region.x1 {
                let (Some(p), Some(q)) = (self.at(x, y), base.at(x, y)) else {
                    continue;
                };
                let delta = (0..4)
                    .filter_map(|c| Some((p.get(c)? - q.get(c)?).abs() * 255.0))
                    .fold(0.0_f64, f64::max);
                if delta <= 2.0 {
                    continue;
                }
                count = count.saturating_add(1);
                bounds = PxRect::grow(bounds, x, y);
                let growth = p[3] - q[3];
                let larger = grown.is_none_or(|(_, best)| growth > best);
                if let Some(s) = self.straight(x, y).filter(|_| larger) {
                    grown = Some((s, growth));
                }
            }
        }
        let mut t = Table::new();
        out::put(&mut t, "differing_pixels", usize_value(count));
        if let Some(b) = bounds {
            out::put(&mut t, "bounds_px_xywh", b.value());
            out::put(&mut t, "bounds_pt_xywh", b.points_value(ratio));
        }
        if let Some((colour, growth)) = grown {
            out::put(&mut t, "largest_alpha_growth", out::float(round3(growth)));
            out::put(&mut t, "at_largest_alpha_growth", out::text(hex(colour)));
        }
        t
    }

    /// What the layer draws in `region` outside `frame`: a focus ring, a
    /// shadow or a glow.
    fn outside(&self, t: &mut Table, frame: PxRect, region: PxRect) {
        let mut bounds: Option<PxRect> = None;
        let mut count = 0_usize;
        let mut strongest: Option<([u8; 4], f64)> = None;
        for y in region.y0..region.y1 {
            for x in region.x0..region.x1 {
                if frame.contains(x, y) {
                    continue;
                }
                let (Some(a), Some(s)) = (self.alpha(x, y), self.straight(x, y)) else {
                    continue;
                };
                if a < EMPTY_ALPHA {
                    continue;
                }
                count = count.saturating_add(1);
                bounds = PxRect::grow(bounds, x, y);
                if strongest.is_none_or(|(_, best)| a > best) {
                    strongest = Some((s, a));
                }
            }
        }
        let mut o = Table::new();
        out::put(&mut o, "pixels", usize_value(count));
        if let Some(b) = bounds {
            out::put(&mut o, "bounds_px_xywh", b.value());
        }
        if let Some((colour, _)) = strongest {
            out::put(&mut o, "strongest", out::text(hex(colour)));
        }
        out::put_table(t, "outside_frame", o);
    }
}

/// The radius of a circular corner whose row insets best match `profile`
/// (least squares over 0.25 px steps up to `CORNER_ROWS` px), and the fit's
/// RMS error; the inset of a radius `r` at row `i` is `r - sqrt(r^2 - (r -
/// (i + 0.5))^2)` above the row `r` and 0 below.
fn fit_radius(profile: &[f64]) -> Option<(f64, f64)> {
    if profile.is_empty() {
        return None;
    }
    let rows = usize_f64(profile.len());
    let mut best: Option<(f64, f64)> = None;
    for step in 1..=(CORNER_ROWS.saturating_mul(4)) {
        let r = usize_f64(step) * 0.25;
        let sse: f64 = profile
            .iter()
            .enumerate()
            .map(|(i, measured)| {
                let y = usize_f64(i) + 0.5;
                let predicted = if y < r {
                    r - (r * r - (r - y) * (r - y)).sqrt()
                } else {
                    0.0
                };
                (predicted - measured) * (predicted - measured)
            })
            .sum();
        if best.is_none_or(|(_, b)| sse < b) {
            best = Some((r, sse));
        }
    }
    best.map(|(r, sse)| (r, (sse / rows).sqrt()))
}

/// The pixels whose bytes differ between `a` and `b`, and their bounds.
pub(super) fn changed(a: &Rgba, b: &Rgba) -> Table {
    let mut t = Table::new();
    if (a.width, a.height) != (b.width, b.height) {
        out::put(
            &mut t,
            "error",
            out::unavailable("the captures differ in size"),
        );
        return t;
    }
    let mut bounds: Option<PxRect> = None;
    let mut count = 0_usize;
    for (i, (p, q)) in a.pixels.iter().zip(b.pixels.iter()).enumerate() {
        if p != q {
            count = count.saturating_add(1);
            let (Some(y), Some(x)) = (i.checked_div(a.width), i.checked_rem(a.width)) else {
                continue;
            };
            bounds = PxRect::grow(bounds, x, y);
        }
    }
    out::put(&mut t, "differing_pixels", usize_value(count));
    if let Some(b) = bounds {
        out::put(&mut t, "bounds_px_xywh", b.value());
    }
    t
}

fn round3(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}
