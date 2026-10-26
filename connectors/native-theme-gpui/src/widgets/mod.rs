//! Theme-drawn controls for the widgets gpui-component draws from its own
//! literals (feature `widgets`, on by default).
//!
//! gpui-component builds its checkbox, radio, switch, slider, progress bar and
//! spinner from sizes and alpha blends of its own, which no `ThemeColor` field
//! or `StyleRefinement` reaches: the indicator in rems per `Size`
//! (`checkbox.rs:229-234`), the switch's track and thumb in pixels
//! (`switch.rs:195-204`), the slider's rail as its fill at 0.2
//! (`slider.rs:301`), the progress bar's track likewise
//! (`progress/progress.rs:135`), and the spinner as a turning icon
//! (`spinner.rs:25`). The controls here are built the way gpui-component
//! builds its own -- on gpui-base's headless primitives, which own
//! activation, focus, keyboard, dragging and the AccessKit role -- and paint
//! every part from the [`ResolvedTheme`](crate::ResolvedTheme) leaf that
//! states it. The spinner
//! draws the application's icon set's own loading indicator, and an arc only
//! for a set without one.
//!
//! The tab bar joins them because no gpui-component `TabBar` variant draws
//! what `tab.*` states: each paints an idle tab transparent
//! (`tab/tab.rs:173`) and marks the selected one its own way, with a primary
//! underline or a frame in `border` (`tab/tab.rs`, `TabVariant`). The
//! separator joins them because gpui-component's draws its line a literal
//! `px(1.)` thick (`separator.rs:78-82`).
//!
//! Each widget reads the variant [`apply`](crate::apply) installed for the
//! current mode at render, through
//! [`ActiveNativeTheme::native`](crate::ActiveNativeTheme::native). With none
//! installed, or with a length that is not finite, it renders
//! gpui-component's own control instead, so nothing here ever paints a value
//! no source gives.
//!
//! Rules, from `docs/archive/todo_gpui-widgets-spec.md` §1:
//!
//! * a hover colour is a layer composited over the idle fill, source-over
//!   (`colors::over`); a `None` soft option leaves the part at rest, which
//!   the colour it would cover, laid over a translucent part, would not (a
//!   tab's hover takes the place of its fill, over the bar, as Breeze paints
//!   it: docs/platform-facts.md §2.11);
//! * a disabled control paints its stated disabled colours and is faded by
//!   its `disabled_opacity` on top: a platform dims by one of the two, and
//!   its data makes the other an identity (docs/platform-facts.md §2.1.6);
//!   a disabled soft fill that is `None` is the enabled one;
//! * a size the theme does not state keeps gpui-component's own value, named
//!   as a constant with its upstream line.
//!
//! Upstream citations are verified against gpui-component 0.8.0, gpui-base
//! 0.8.0 and gpui-pre 0.3.9.

/// `IntoElement` for a `RenderOnce` widget, as gpui's `#[derive(IntoElement)]`
/// writes it (gpui-pre-macros `derive_into_element.rs`). Written out, so the
/// impl does not rest on the derive's path rewriting: the derive turns its
/// `gpui::` paths into `gpui_kit::` whenever the manifest names gpui-kit
/// (`gpui_pre_facade_paths.rs`).
macro_rules! into_element {
    ($($widget:ty),+ $(,)?) => {$(
        impl gpui::IntoElement for $widget {
            type Element = gpui::ViewElement<Self>;

            #[track_caller]
            fn into_element(self) -> Self::Element {
                gpui::ViewElement::new(self)
            }
        }
    )+};
}

mod checkbox;
mod progress;
mod separator;
mod slider;
mod spinner;
mod switch;
mod tab_bar;
#[cfg(test)]
mod tests;

pub use checkbox::{Checkbox, CheckboxLook, Radio, RadioGroup};
pub use progress::{ProgressBar, ProgressBarLook};
pub use separator::{Separator, SeparatorLook};
pub use slider::{Slider, SliderLook};
pub use spinner::{Spinner, SpinnerLook};
pub use switch::{Switch, SwitchLook};
pub use tab_bar::{Tab, TabBar, TabIndicator, TabLook};

into_element!(
    Checkbox,
    Radio,
    RadioGroup,
    Switch,
    Slider,
    ProgressBar,
    Spinner,
    TabBar,
    Separator
);

use std::rc::Rc;

use gpui::{
    A11ySubtreeBuilder, App, Bounds, Element, ElementId, FontWeight, GlobalElementId, Hsla,
    InspectorElementId, InteractiveElement, Interactivity, IntoElement, LayoutId, Pixels,
    Styled as _, Window, accesskit, canvas, div, px, relative,
};
use native_theme::color::Rgba;
use native_theme::theme::ResolvedFontSpec;

use crate::Native;
use crate::colors::over;

/// A part of a theme-drawn control, as [`PartBounds`] names it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Part {
    /// A checkbox's box or a radio button's circle.
    Indicator,
    /// A checkbox's check mark or a radio button's dot, laid out whether it
    /// shows or not.
    Mark,
    /// The text beside a checkbox, a radio button or a switch.
    Label,
    /// A switch's track, or the rail a slider's thumb travels along.
    Track,
    /// The filled stretch of a slider's rail or a progress bar's track.
    Fill,
    /// A switch's or a slider's thumb; a range slider's end thumb.
    Thumb,
    /// A range slider's start thumb.
    StartThumb,
    /// The tab at this index of a [`TabBar`].
    Tab(usize),
}

/// Where a theme-drawn control reports the bounds each of its parts was
/// laid out at, in window coordinates, outer edge of its border included --
/// for an application that measures its own layout, as the showcase's
/// layout dump does. Called while a frame lays the control out (gpui's
/// prepaint), for the parts the control draws, with the window being drawn;
/// a control drawn as gpui-component's own, with no native theme installed,
/// reports nothing. A frame that draws the control from the last one, as
/// GPUI Fast draws a view nothing changed for, does not lay it out and does
/// not call it: the parts are where they were. Nor does a frame in which
/// GPUI Fast composites a scroll area the control is in while it scrolls:
/// it draws the area's content moved by the scroll, without laying it out
/// (gpui-fast 0.1.10 src/fast/layers/paint.rs:233-235,
/// src/fast/layers/reuse.rs:39), so there the parts have moved by the
/// scroll since the last call.
pub type PartBounds = Rc<dyn Fn(Part, Bounds<Pixels>, &mut Window, &mut App)>;

/// An absolute, invisible box laid over a part whose border is `border`
/// wide, reaching out to the border's outer edge: an absolute child is laid
/// out inside its parent's border, so its insets go out by the border's
/// width. It hands `observer` its bounds, the part's.
fn part_bounds(part: Part, observer: &PartBounds, border: Pixels) -> impl IntoElement + use<> {
    part_bounds_above(part, observer, border, border)
}

/// [`part_bounds`] over a part whose bottom border is `bottom` wide and its
/// other sides' `border`.
fn part_bounds_above(
    part: Part,
    observer: &PartBounds,
    border: Pixels,
    bottom: Pixels,
) -> impl IntoElement + use<> {
    let observer = observer.clone();
    outer_edge(border, bottom, move |bounds, window, cx| {
        observer(part, bounds, window, cx)
    })
}

/// An absolute, invisible box laid over an element whose border is `border`
/// wide on its top, left and right and `bottom` on its bottom, reaching out
/// to the border's outer edge, that hands `record` its bounds, the
/// element's, while the frame is laid out (prepainted).
fn outer_edge<F>(border: Pixels, bottom: Pixels, record: F) -> impl IntoElement + use<F>
where
    F: FnOnce(Bounds<Pixels>, &mut Window, &mut App) + 'static,
{
    canvas(record, |_, (), _, _| {})
        .absolute()
        .top(px(-f32::from(border)))
        .left(px(-f32::from(border)))
        .right(px(-f32::from(border)))
        .bottom(px(-f32::from(bottom)))
}

/// A control's root element whose accessibility node is marked disabled
/// while `disabled` is true; every other `Element` method is its inner
/// element's, and so is its `InteractiveElement` interactivity, so a
/// registration or a focus binding made on the wrapper lands on the inner
/// element (as gpui-base's forwards it, a11y_disabled.rs:71-75). GPUI has no
/// `aria_disabled`, and gpui-base's wrapper of this
/// shape, `A11yDisabled` (gpui-base a11y_disabled.rs:13-144), which its own
/// controls are marked with, is `pub(crate)` there (:13, :28).
struct A11yDisabled<E> {
    inner: E,
    disabled: bool,
}

impl<E> A11yDisabled<E> {
    fn new(inner: E, disabled: bool) -> Self {
        Self { inner, disabled }
    }
}

impl<E: InteractiveElement> InteractiveElement for A11yDisabled<E> {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.inner.interactivity()
    }
}

impl<E: Element> IntoElement for A11yDisabled<E> {
    type Element = Self;

    fn into_element(self) -> Self {
        self
    }
}

impl<E: Element> Element for A11yDisabled<E> {
    type RequestLayoutState = E::RequestLayoutState;
    type PrepaintState = E::PrepaintState;

    fn id(&self) -> Option<ElementId> {
        self.inner.id()
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        self.inner.source_location()
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        self.inner.request_layout(id, inspector_id, window, cx)
    }

    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        self.inner
            .prepaint(id, inspector_id, bounds, request_layout, window, cx)
    }

    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.inner.paint(
            id,
            inspector_id,
            bounds,
            request_layout,
            prepaint,
            window,
            cx,
        );
    }

    fn a11y_role(&self) -> Option<accesskit::Role> {
        self.inner.a11y_role()
    }

    fn write_a11y_info(&self, node: &mut accesskit::Node) {
        self.inner.write_a11y_info(node);
        if self.disabled {
            node.set_disabled();
        }
    }

    fn a11y_synthetic_children(
        &mut self,
        prepaint: &mut Self::PrepaintState,
        builder: &mut A11ySubtreeBuilder,
    ) {
        self.inner.a11y_synthetic_children(prepaint, builder);
    }
}

/// A stated colour as gpui's.
fn color(rgba: Rgba) -> Hsla {
    crate::colors::rgba_to_hsla(rgba)
}
/// The theme's focus ring, which [`FocusRing::draw`] puts round a theme-drawn
/// control that has the focus, in place of gpui-component's `focus_ring_style`, which draws a
/// 3 px ring at half `ThemeColor::ring`'s alpha whatever the model states
/// (gpui-component 0.8.0 `src/styled.rs:11-12`, `:301-327`).
///
/// The ring is `defaults.focus_ring_color` as stated, alpha included, a band
/// `defaults.focus_ring_width` wide whose outer edge is the element's bounds
/// grown by `defaults.focus_ring_offset + focus_ring_width` (the offset from
/// the element's edge to the ring's inner edge, outward where positive), its
/// corners the element's grown as much, floored at 0. It is an absolute
/// child laid out against the element's padding box, so it moves nothing
/// (as upstream's `focus_ring`), which puts its edges at the border widths
/// the element states, plus the growth, outside; an unstated border width or
/// radius is none, as gpui lays it out (upstream reads them the same way,
/// `src/styled.rs:197-221`). The same geometry as the iced connector's
/// `focus_ring::RingLook::quad` and the egui connector's `paint_focus_ring`
/// (`connectors/native-theme-iced/src/focus_ring.rs`,
/// `connectors/native-theme-egui/src/plugin.rs`).
///
/// No ring where the width is not positive or the width or the offset is not
/// finite. Where the ring lies inset at the element's edge (`offset` is
/// minus `width`, libadwaita's), the element's border takes
/// `focus_ring_color` instead, as the iced connector's focused field draws
/// its ring as its border.
#[derive(Clone, Copy, Debug, PartialEq)]
struct FocusRing {
    /// `defaults.focus_ring_color`, as stated.
    color: Hsla,
    /// `defaults.focus_ring_width`: positive and finite.
    width: f32,
    /// `defaults.focus_ring_offset`: finite.
    offset: f32,
}

impl FocusRing {
    /// The ring `resolved` states, or `None` where it states none.
    fn of(resolved: &native_theme::theme::ResolvedTheme) -> Option<Self> {
        let d = &resolved.defaults;
        let (width, offset) = (d.focus_ring_width, d.focus_ring_offset);
        (width.is_finite() && offset.is_finite() && width > 0.0).then(|| Self {
            color: color(d.focus_ring_color),
            width,
            offset,
        })
    }

    /// `element` with the ring round it, or with its border in the ring's
    /// colour where the ring lies inset at its edge.
    fn draw<E: gpui::Styled + gpui::ParentElement>(self, mut element: E, window: &Window) -> E {
        use gpui_component::StyledExt as _;
        let Self {
            color: ring,
            width,
            offset,
        } = self;
        if offset == -width {
            return element.border_color(ring);
        }
        let rem = window.rem_size();
        let length = |l: Option<gpui::AbsoluteLength>| {
            l.map(|l| f32::from(l.to_pixels(rem))).unwrap_or_default()
        };
        let style = element.style();
        let (top, right, bottom, left) = (
            length(style.border_widths.top),
            length(style.border_widths.right),
            length(style.border_widths.bottom),
            length(style.border_widths.left),
        );
        let grow = offset + width;
        let corner = |l: Option<gpui::AbsoluteLength>| Some(px((length(l) + grow).max(0.0)).into());
        let mut band = gpui::StyleRefinement::default();
        band.corner_radii.top_left = corner(style.corner_radii.top_left);
        band.corner_radii.top_right = corner(style.corner_radii.top_right);
        band.corner_radii.bottom_right = corner(style.corner_radii.bottom_right);
        band.corner_radii.bottom_left = corner(style.corner_radii.bottom_left);
        element.child(
            div()
                .debug_selector(|| FOCUS_RING_SELECTOR.into())
                .flex_none()
                .absolute()
                .top(px(-(grow + top)))
                .right(px(-(grow + right)))
                .bottom(px(-(grow + bottom)))
                .left(px(-(grow + left)))
                .border(px(width))
                .border_color(ring)
                .refine_style(&band),
        )
    }
}

/// The debug selector of the ring [`FocusRing::draw`] draws.
const FOCUS_RING_SELECTOR: &str = "native-focus-ring";

/// One colour of a control faded as one widget over `ground`, the colour
/// under the control: `colour` composited over `under`, the part it is
/// painted on (the ground itself, or a fill already composited over it),
/// then faded by `opacity` over `ground`. A platform that dims a disabled
/// control by opacity fades the finished control as a whole
/// (docs/platform-facts.md §2.1.6: libadwaita's `filter: Opacity(..)`),
/// where gpui fades each quad on its own (gpui-pre src/window.rs:4687-4695,
/// `paint_quad`), which would show a faded part through another.
fn faded(ground: Hsla, under: Hsla, colour: Hsla, opacity: f32) -> Hsla {
    over(ground, over(under, colour).opacity(opacity))
}

/// The opacity of a control, `disabled` or not: its `disabled_opacity`, the
/// whole-widget fade a platform states on top of its disabled colours
/// (docs/platform-facts.md §2.1.6), when disabled; 1 when enabled, or when the
/// opacity is not a finite number.
fn disabled_opacity(disabled: bool, opacity: f32) -> f32 {
    if disabled && opacity.is_finite() {
        opacity.clamp(0., 1.)
    } else {
        1.
    }
}

/// A stated length in pixels, or `None` when it is not a finite,
/// non-negative number: the widget then falls back to gpui-component's own
/// control (spec §1.1).
fn length(v: f32) -> Option<Pixels> {
    (v.is_finite() && v >= 0.0).then(|| px(v))
}

/// The text of a widget's label in `font`: its family, size, weight and
/// style (gpui's of the same name), one line tall -- its size by
/// `defaults.line_height`, the line box the model states.
fn font_text(font: &ResolvedFontSpec, n: Native<'_>) -> gpui::Div {
    let mut text = div()
        .font_family(crate::font_family(&font.family))
        .text_size(text_size(font.size, n))
        .font_weight(FontWeight(f32::from(font.weight)))
        .line_height(relative(n.resolved.defaults.line_height));
    text.text_style().font_style = Some(crate::font_style(font.style));
    text
}

/// A font size the text-scaling factor scales, as the `geometry` builders
/// scale theirs.
fn text_size(size: f32, n: Native<'_>) -> Pixels {
    px(crate::scaled_text_size(size, n.accessibility))
}
