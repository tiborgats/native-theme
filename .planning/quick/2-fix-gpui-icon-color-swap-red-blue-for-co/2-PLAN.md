# Quick Task 2: Fix gpui icon color swap (red/blue) for colored SVG themes

## Problem

When loading the breeze-dark icon theme in the gpui showcase, icons that should be red appear in blue. The same icons display correctly in the iced showcase.

## Root Cause

Bug in gpui 0.2.2's `Image::to_image_data()` (`platform.rs:1779-1787`): the `ImageFormat::Svg` branch renders SVGs with resvg (which produces premultiplied RGBA) but **skips** the RGBA→BGRA pixel conversion that all other format branches perform. This causes red and blue channels to be swapped.

## Fix

Rasterize SVGs ourselves using `native_theme::rasterize::rasterize_svg()` and encode as BMP (which gpui handles correctly through its BMP decoder + RGBA→BGRA swap), bypassing the buggy `ImageFormat::Svg` path entirely.

## Tasks

- [x] Task 1: Add `svg-rasterize` feature to gpui connector's native-theme dependency
- [x] Task 2: Replace `ImageFormat::Svg` usage with rasterize→BMP pipeline in `to_image_source` and `to_image_source_colored`
- [x] Task 3: Update test expectations
