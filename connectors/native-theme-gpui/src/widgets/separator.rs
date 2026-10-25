//! `Separator`: a line `separator.line_width` thick in `separator.line_color`.

use gpui::{
    AnyElement, App, Axis, Hsla, InteractiveElement as _, IntoElement, Pixels, RenderOnce,
    StyleRefinement, Styled, Window, div, px,
};
use gpui_component::StyledExt as _;
use native_theme::theme::ResolvedTheme;

use super::{color, length};
use crate::ActiveNativeTheme;

/// What a separator paints, from `SeparatorTheme`.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct SeparatorLook {
    /// `separator.line_color`.
    pub color: Hsla,
    /// `separator.line_width`: the line's thickness.
    pub width: Pixels,
}

impl SeparatorLook {
    /// The look of a separator, or `None` when the width the theme gives is
    /// not finite.
    #[must_use]
    pub fn of(resolved: &ResolvedTheme) -> Option<Self> {
        let s = &resolved.separator;
        Some(Self {
            color: color(s.line_color),
            width: length(s.line_width)?,
        })
    }
}

/// A solid line across its container, `separator.line_width` thick in
/// `separator.line_color`, and at least one device pixel thick, a box filled
/// in the colour, which every gpui renderer draws: gpui-component's
/// `Separator` draws its line as an
/// absolute child a literal `px(1.)` thick (separator.rs:78-82), which the
/// caller's style does not reach.
///
/// Without a native theme, or with a width that is not finite, it renders
/// gpui-component's `Separator`.
pub struct Separator {
    axis: Axis,
    style: StyleRefinement,
}

impl Separator {
    /// A line across the container's width.
    #[must_use]
    pub fn horizontal() -> Self {
        Self {
            axis: Axis::Horizontal,
            style: StyleRefinement::default(),
        }
    }

    /// A line down the container's height.
    #[must_use]
    pub fn vertical() -> Self {
        Self {
            axis: Axis::Vertical,
            style: StyleRefinement::default(),
        }
    }

    fn fallback(self) -> AnyElement {
        match self.axis {
            Axis::Horizontal => gpui_component::separator::Separator::horizontal(),
            Axis::Vertical => gpui_component::separator::Separator::vertical(),
        }
        .refine_style(&self.style)
        .into_any_element()
    }
}

impl Styled for Separator {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Separator {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(look) = cx.native().and_then(|n| SeparatorLook::of(n.resolved)) else {
            return self.fallback();
        };
        // The line is a box filled in its colour, at least one device pixel
        // thick. gpui rounds a box's stated size to the device-pixel grid half
        // toward zero (gpui-pre 0.3.9 `src/taffy.rs:522-526`,
        // `src/util.rs:140-142`), so a box `separator.line_width` 0.5 thick
        // (the macOS presets) takes no room on a 1x display; and a line drawn
        // as a border of a box one device pixel thick is drawn by no gpui
        // renderer: the Metal, wgpu and DirectX quad shaders take a pixel's
        // border by the half of the quad its centre lies in, and the one row
        // of such a quad lies on its middle, which takes the opposite, zero,
        // border (gpui-pre-apple 0.3.9 `src/shaders.metal:133-137`,
        // gpui-pre-wgpu 0.3.9 `src/shaders.wgsl:603-611`, gpui-pre-windows
        // 0.3.9 `src/shaders.hlsl:590-593`). A filled box is drawn by them all.
        let device_pixel = px(1.0 / window.scale_factor());
        let thickness = look.width.max(device_pixel);
        let line = div().flex_none().bg(look.color);
        match self.axis {
            Axis::Horizontal => line.w_full().h(thickness),
            Axis::Vertical => line.h_full().w(thickness),
        }
        .refine_style(&self.style)
        .debug_selector(|| "native-separator".into())
        .into_any_element()
    }
}
