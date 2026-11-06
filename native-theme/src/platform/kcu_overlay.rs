//! `kcu_overlay`: KColorUtils' overlay, a source-over paint of one colour on an opaque other,
//! through Qt's 8-bit premultiplied raster image.

use super::kde_color_common::{QtColor, Rgba, enter, q16, read};

/// `(x + 128 - ((x + 128) >> 8)) >> 8`: a 16-bit value to 8 bits, rounded.
fn div257(x: u64) -> Option<u64> {
    let y = x.checked_add(128)?;
    Some(y.checked_sub(y >> 8)? >> 8)
}

/// `(x + (x >> 16) + 0x8000) >> 16`: a product of two 16-bit values back to 16 bits, rounded.
fn div65535(x: u64) -> Option<u64> {
    Some(x.checked_add(x >> 16)?.checked_add(0x8000)? >> 16)
}

/// `(x·k + ((x·k) >> 8) + 128) >> 8`: an 8-bit value times `k / 255`, rounded.
fn bytemul(x: u64, k: u64) -> Option<u64> {
    let p = x.checked_mul(k)?;
    Some(p.checked_add(p >> 8)?.checked_add(128)? >> 8)
}

/// The 16-bit channels Qt paints with; a half value is clamped to `[0, 1]` first.
fn channels16(c: QtColor) -> ([u16; 4], bool) {
    match c {
        QtColor::Standard(s) => (s, false),
        QtColor::Extended(h) => (h.map(|x| q16(x.clamp(0.0, 1.0))), true),
        QtColor::Invalid => ([0; 4], false),
    }
}

/// The painted pixel's red, green and blue, 8-bit.
fn paint(under: [u16; 4], over: [u16; 4], extended_over: bool) -> Option<[u64; 3]> {
    let [ur, ug, ub, _] = under.map(u64::from);
    let [vr, vg, vb, va] = over.map(u64::from);
    let base = [div257(ur)?, div257(ug)?, div257(ub)?];
    if va == 0 {
        return Some(base);
    }
    if va == 65535 && !extended_over {
        return Some([div257(vr)?, div257(vg)?, div257(vb)?]);
    }
    let sa = div257(va)?;
    let keep = 255_u64.checked_sub(sa)?;
    let blend = |v: u64, d: u64| -> Option<u64> {
        let s = div257(div65535(v.checked_mul(va)?)?)?;
        s.checked_add(bytemul(d, keep)?)
    };
    let [dr, dg, db] = base;
    Some([blend(vr, dr)?, blend(vg, dg)?, blend(vb, db)?])
}

/// Paints `over` source-over on `under` made opaque, as `KColorUtils::overlayColors` does
/// through a one-pixel 8-bit premultiplied image; the result is opaque and 8-bit.
pub fn kcu_overlay(under: Rgba, over: Rgba) -> Rgba {
    let (u, _) = channels16(enter(under));
    let (v, extended_over) = channels16(enter(over));
    let pixel = paint(u, v, extended_over).and_then(|[r, g, b]| {
        let to16 = |x: u64| u16::try_from(x.checked_mul(257)?).ok();
        Some([to16(r)?, to16(g)?, to16(b)?, 65535])
    });
    match pixel {
        Some(s) => read(QtColor::Standard(s)),
        None => {
            let [r, g, b, _] = read(QtColor::Standard(u));
            [r, g, b, 1.0]
        }
    }
}
