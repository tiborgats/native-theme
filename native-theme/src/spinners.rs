// Feature-gated bundled spinner construction.
//
// Uses genuine icon files from their respective open-source icon sets,
// the same SVGs used for static icons. No fabricated approximations.
//
// Spinners are delivered as pre-rotated SVG frames (24 frames, 42ms each ≈ 1s cycle).
// Each frame wraps the original SVG content in a <g transform="rotate(...)"> element,
// so rasterization produces clean anti-aliased output at any size. This avoids
// renderer-dependent rotation artifacts (gpui can't rotate BMP images; iced's pixel
// rotation produces noisy edges on high-viewBox SVGs like Material's 960-unit path).

use std::borrow::Cow;
use std::num::NonZeroU32;

use crate::model::animated::{AnimatedIcon, TransformAnimation};
use crate::model::icons::IconData;

/// Number of rotation frames per spin cycle.
const SPIN_FRAME_COUNT: u32 = 24;

/// Duration of each frame in milliseconds (24 frames x 42ms = 1008ms ~ 1s cycle).
///
/// Must be > 0; a zero duration would cause infinite-loop or division-by-zero in renderers.
const SPIN_FRAME_DURATION_MS: u32 = 42;

/// Generate pre-rotated SVG frames from a single SVG icon.
///
/// Parses the SVG to extract the viewBox center, then generates `SPIN_FRAME_COUNT`
/// frames, each with the inner content wrapped in `<g transform="rotate(angle cx cy)">`.
/// This is the same technique used by `scripts/generate_gifs.py` for the README animations.
fn svg_to_spin_frames(svg_bytes: &[u8]) -> Vec<IconData> {
    let svg_str = match std::str::from_utf8(svg_bytes) {
        Ok(s) => s,
        Err(_) => return vec![IconData::Svg(Cow::Owned(svg_bytes.to_vec()))],
    };

    // Extract <svg ...> tag attributes and inner content
    let Some(svg_open_end) = svg_str.find('>') else {
        return vec![IconData::Svg(Cow::Owned(svg_bytes.to_vec()))];
    };
    let svg_tag = &svg_str[..svg_open_end];

    // S-2: guard against a malformed tag shorter than "<svg" (4 bytes).
    // Currently unreachable (only called with bundled SVGs), but prevents
    // a panic on the `&svg_tag[4..]` slice below if ever called with
    // arbitrary input.
    if svg_tag.len() < 4 {
        return vec![IconData::Svg(Cow::Owned(svg_bytes.to_vec()))];
    }

    // Find the closing </svg> to extract inner content
    let inner_start = svg_open_end + 1;
    let Some(close_pos) = svg_str.rfind("</svg>") else {
        return vec![IconData::Svg(Cow::Owned(svg_bytes.to_vec()))];
    };
    let inner_content = &svg_str[inner_start..close_pos];

    const { assert!(SPIN_FRAME_DURATION_MS > 0, "frame duration must be > 0") }

    // Extract viewBox to compute rotation center (handle both quote styles)
    let (cx, cy, valid_viewbox) = {
        let (vb_val_start, quote) = if let Some(i) = svg_tag.find("viewBox=\"") {
            (i + 9, '"')
        } else if let Some(i) = svg_tag.find("viewBox='") {
            (i + 9, '\'')
        } else {
            (0, '"') // no viewBox found; falls through to default
        };

        if vb_val_start > 0 {
            if let Some(vb_end) = svg_str[vb_val_start..].find(quote) {
                let vb = &svg_str[vb_val_start..vb_val_start + vb_end];
                let parts: Vec<f64> = vb
                    .split(|c: char| c.is_whitespace() || c == ',')
                    .filter(|s| !s.is_empty())
                    .filter_map(|s| s.parse::<f64>().ok())
                    .collect();
                if parts.len() == 4 {
                    if parts[2] <= 0.0 || parts[3] <= 0.0 {
                        // Invalid dimensions -- return static frame
                        (12.0, 12.0, false)
                    } else {
                        (parts[0] + parts[2] / 2.0, parts[1] + parts[3] / 2.0, true)
                    }
                } else {
                    (12.0, 12.0, true)
                }
            } else {
                (12.0, 12.0, true)
            }
        } else {
            (12.0, 12.0, true) // fallback for 24x24 icons without viewBox
        }
    };

    if !valid_viewbox {
        return vec![IconData::Svg(Cow::Owned(svg_bytes.to_vec()))];
    }

    let mut frames = Vec::with_capacity(SPIN_FRAME_COUNT as usize);
    for i in 0..SPIN_FRAME_COUNT {
        let angle = i as f64 * (360.0 / SPIN_FRAME_COUNT as f64);
        let rotated = format!(
            "<svg{svg_tag_rest}>\
             <g transform=\"rotate({angle:.1} {cx:.1} {cy:.1})\">\
             {inner_content}\
             </g>\
             </svg>",
            svg_tag_rest = &svg_tag[4..], // skip "<svg" prefix, keep attributes
        );
        frames.push(IconData::Svg(Cow::Owned(rotated.into_bytes())));
    }

    // Guard: if frame generation somehow produced nothing, return static frame
    if frames.is_empty() {
        return vec![IconData::Svg(Cow::Owned(svg_bytes.to_vec()))];
    }

    frames
}

/// Build an `AnimatedIcon` from pre-rotated frames, falling back to a
/// transform-based spin if frame construction fails (e.g. empty vec,
/// which should not happen with bundled SVGs but is handled gracefully).
fn frames_or_spin_fallback(svg_bytes: &'static [u8]) -> AnimatedIcon {
    let frames = svg_to_spin_frames(svg_bytes);
    // SPIN_FRAME_DURATION_MS is compile-time asserted > 0 above.
    let Some(dur) = NonZeroU32::new(SPIN_FRAME_DURATION_MS) else {
        // Fallback: transform spin (compile-time assert prevents this path)
        return AnimatedIcon::transform(
            IconData::Svg(Cow::Borrowed(svg_bytes)),
            TransformAnimation::Spin {
                duration_ms: NonZeroU32::MIN,
            },
        );
    };
    match AnimatedIcon::frames(frames, dur) {
        Ok(anim) => anim,
        Err(_) => AnimatedIcon::transform(
            IconData::Svg(Cow::Borrowed(svg_bytes)),
            TransformAnimation::Spin { duration_ms: dur },
        ),
    }
}

/// Material Design progress_activity icon as pre-rotated frames.
#[cfg(feature = "material-icons")]
pub(crate) fn material_spinner() -> AnimatedIcon {
    let svg_bytes = include_bytes!("../icons/material/progress_activity.svg");
    frames_or_spin_fallback(svg_bytes)
}

/// Lucide loader icon as pre-rotated frames.
#[cfg(feature = "lucide-icons")]
pub(crate) fn lucide_spinner() -> AnimatedIcon {
    let svg_bytes = include_bytes!("../icons/lucide/loader.svg");
    frames_or_spin_fallback(svg_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn svg_to_spin_frames_produces_24_frames() {
        let svg = b"<svg viewBox=\"0 0 24 24\"><path d=\"M12 2v4\"/></svg>";
        let frames = svg_to_spin_frames(svg);
        assert_eq!(frames.len(), SPIN_FRAME_COUNT as usize);
    }

    #[test]
    fn spin_frames_contain_rotate_transform() {
        let svg = b"<svg viewBox=\"0 0 24 24\"><circle r=\"5\"/></svg>";
        let frames = svg_to_spin_frames(svg);
        // First frame: 0 degrees
        if let IconData::Svg(bytes) = &frames[0] {
            let s = std::str::from_utf8(bytes).unwrap();
            assert!(s.contains("rotate(0.0 12.0 12.0)"), "frame 0: {s}");
        }
        // Frame 6 (of 24): 90 degrees
        if let IconData::Svg(bytes) = &frames[6] {
            let s = std::str::from_utf8(bytes).unwrap();
            assert!(s.contains("rotate(90.0 12.0 12.0)"), "frame 6: {s}");
        }
    }

    #[test]
    fn spin_frames_use_viewbox_center() {
        // Material-style viewBox: "0 -960 960 960" → center at (480, -480)
        let svg = b"<svg viewBox=\"0 -960 960 960\"><path d=\"M0 0\"/></svg>";
        let frames = svg_to_spin_frames(svg);
        if let IconData::Svg(bytes) = &frames[0] {
            let s = std::str::from_utf8(bytes).unwrap();
            assert!(s.contains("480.0 -480.0"), "should use viewBox center: {s}");
        }
    }

    #[cfg(feature = "material-icons")]
    #[test]
    fn material_spinner_is_frames() {
        let anim = material_spinner();
        assert!(
            matches!(anim, AnimatedIcon::Frames(_)),
            "material spinner should be Frames, not Transform"
        );
        if let AnimatedIcon::Frames(data) = &anim {
            assert_eq!(data.frames().len(), 24);
            assert_eq!(data.frame_duration_ms().get(), 42);
        }
    }

    #[cfg(feature = "lucide-icons")]
    #[test]
    fn lucide_spinner_is_frames() {
        let anim = lucide_spinner();
        assert!(matches!(anim, AnimatedIcon::Frames(_)));
        if let AnimatedIcon::Frames(data) = &anim {
            assert_eq!(data.frames().len(), 24);
            assert_eq!(data.frame_duration_ms().get(), 42);
        }
    }

    #[test]
    fn single_quote_viewbox() {
        let svg = b"<svg viewBox='0 0 24 24'><path d=\"M12 2v4\"/></svg>";
        let frames = svg_to_spin_frames(svg);
        assert_eq!(frames.len(), SPIN_FRAME_COUNT as usize);
        // Verify center is computed correctly (12.0, 12.0)
        if let IconData::Svg(bytes) = &frames[0] {
            let s = std::str::from_utf8(bytes).unwrap();
            assert!(s.contains("rotate(0.0 12.0 12.0)"), "frame 0: {s}");
        }
    }

    #[test]
    fn zero_dimension_viewbox_returns_static() {
        let svg = b"<svg viewBox=\"0 0 0 24\"><path d=\"M12 2v4\"/></svg>";
        let frames = svg_to_spin_frames(svg);
        assert_eq!(
            frames.len(),
            1,
            "zero-width viewBox should return single static frame"
        );
    }

    #[test]
    fn negative_dimension_viewbox_returns_static() {
        let svg = b"<svg viewBox=\"0 0 -10 24\"><path d=\"M12 2v4\"/></svg>";
        let frames = svg_to_spin_frames(svg);
        assert_eq!(
            frames.len(),
            1,
            "negative-width viewBox should return single static frame"
        );
    }

    #[test]
    fn comma_separated_viewbox() {
        let svg = b"<svg viewBox=\"0,0,24,24\"><circle r=\"5\"/></svg>";
        let frames = svg_to_spin_frames(svg);
        assert_eq!(frames.len(), SPIN_FRAME_COUNT as usize);
        if let IconData::Svg(bytes) = &frames[0] {
            let s = std::str::from_utf8(bytes).unwrap();
            assert!(s.contains("rotate(0.0 12.0 12.0)"), "frame 0: {s}");
        }
    }
}
