# Quick Task 2: Summary

## What changed

### Files modified
- `connectors/native-theme-gpui/Cargo.toml` — added `svg-rasterize` feature
- `connectors/native-theme-gpui/src/icons.rs` — rasterize SVGs to BMP instead of passing as `ImageFormat::Svg`

### Key changes

1. **Added `svg-rasterize` feature** to the gpui connector's native-theme dependency, enabling `native_theme::rasterize::rasterize_svg()`.

2. **New `svg_to_bmp_source()` helper** — rasterizes SVG bytes at 48px (2x display size for HiDPI) using resvg, then encodes as BMP via the existing `encode_rgba_as_bmp()` function.

3. **Updated `to_image_source()`** — SVG icons now go through rasterize→BMP instead of `ImageFormat::Svg`.

4. **Updated `to_image_source_colored()`** — colorized SVGs also go through the rasterize→BMP path.

5. **Updated test** — `to_image_source_svg_returns_bmp_rasterized` verifies SVGs produce BMP output.

## Root cause

gpui 0.2.2 has a bug in `Image::to_image_data()` where the `ImageFormat::Svg` branch skips the RGBA→BGRA pixel conversion that all other format branches perform. Since gpui renders in BGRA, this causes red and blue channels to swap.

## Test results

All 260 tests pass (205 native-theme + 55 gpui connector).
