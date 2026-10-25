//! `Separator`: a line `separator.line_width` thick in `separator.line_color`.

use gpui::{
    AnyElement, App, Axis, Hsla, InteractiveElement as _, IntoElement, Pixels, RenderOnce,
    StyleRefinement, Styled, Window, div,
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
/// `separator.line_color`, and at least one device pixel thick, as gpui draws
/// a border: gpui-component's `Separator` draws its line as an
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
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(look) = cx.native().and_then(|n| SeparatorLook::of(n.resolved)) else {
            return self.fallback();
        };
        // The line is a border, not a box of the line's height: gpui rounds
        // a box's stated size to the device-pixel grid half toward zero, so
        // a line thinner than one device pixel (`separator.line_width` 0.5
        // under the macOS presets, on a 1x display) would take no room and
        // paint nothing, where it draws a border at least one device pixel
        // wide (gpui-pre 0.3.9 `src/taffy.rs:416-426`, `src/util.rs:140-151`;
        // gpui-fast 0.1.10 `src/taffy.rs:417` and the same `util.rs` lines),
        // as it draws the 0.5 px frames of the same presets.
        let line = div().flex_none().border_color(look.color);
        match self.axis {
            Axis::Horizontal => line.w_full().border_t(look.width),
            Axis::Vertical => line.h_full().border_l(look.width),
        }
        .refine_style(&self.style)
        .debug_selector(|| "native-separator".into())
        .into_any_element()
    }
}
