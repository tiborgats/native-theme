//! Theme-drawn controls for the widgets gpui-component draws from its own
//! literals (feature `widgets`, on by default).
//!
//! gpui-component builds its checkbox, radio, switch, slider, progress bar and
//! spinner from sizes and alpha blends of its own, which no `ThemeColor` field
//! or `StyleRefinement` reaches: the indicator in rems per `Size`
//! (`checkbox.rs:230-235`), the switch's track and thumb in pixels
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
//! Each widget reads the variant [`apply`](crate::apply) or
//! [`next::apply`](crate::next::apply) installed for the current mode at
//! render, of either model (`ThemeSource`). With none installed, a length
//! that is not finite, or a value the look needs that a 0.8.0 theme leaves
//! unstated, it renders gpui-component's own control instead, so nothing here
//! ever paints a value no source gives.
//!
//! Rules, from `docs/archive/todo_gpui-widgets-spec.md`:
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
//! Upstream citations are verified against gpui-component 0.9.1, gpui-base
//! 0.9.1 and gpui-pre 0.3.12, but where one names its version.

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
pub use slider::{Slider, SliderLook, ThumbOutline};
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
    SharedString, Styled as _, Window, accesskit, canvas, div, px, relative,
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
/// (gpui-fast 0.1.15 src/fast/layers/paint.rs:233-235,
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
/// controls are marked with, is `pub(crate)` there (:13, :28). gpui-base's
/// copy now gates its `IntoElement` on the core, as this one does (gpui-base
/// 0.10.0 a11y_disabled.rs:63-70).
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

#[cfg(not(feature = "gpui-fast"))]
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
/// 3 px ring in `ThemeColor::ring` with its alpha set to 0.5 whatever the model
/// states (gpui-component 0.10.0 `src/styled.rs:11-12`, `focus_style`). gpui-component's
/// `Tab` now draws another, 2 px inset at the full colour, on keyboard focus
/// only (gpui-component 0.10.0 `src/tab/tab.rs:933`, `:962-973`).
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
/// minus `width`, libadwaita's) and the element states a border of its own,
/// that border takes `focus_ring_color` instead, as the iced connector's
/// focused field draws its ring as its border; an element without a border
/// gets the band inside its edge. The caller adds the ring after the element's
/// other children, so it paints over them, as the iced and egui rings do, and
/// after its own refinement, which it reads the border and the corners from.
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

    /// The ring a 0.8.0 theme states, as [`FocusRing::of`] reads it from the
    /// homes of its fields: `colors.focus_ring`, `focus_ring.stroke.width`
    /// (none where the ring has no stroke, and no ring where its stroke is per
    /// side) and `focus_ring.offset`.
    fn next_of(r: &native_theme::next::ResolvedTheme) -> Option<Self> {
        let width = crate::next::focus_ring_width(r).ok()?;
        let offset = r.focus_ring.offset;
        (width.is_finite() && offset.is_finite() && width > 0.0).then(|| Self {
            color: crate::next::hsla(r.colors.focus_ring),
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
        // An inset ring on an element with a border of its own is that border in the
        // ring's colour; on one without, the band below draws it inside the edge, as the
        // egui connector's inset band (`paint_focus_ring`).
        if offset == -width && [top, right, bottom, left].iter().any(|w| *w > 0.0) {
            return element.border_color(ring);
        }
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

/// Whether a theme-drawn control's focus followed a key, decided when it gets the focus
/// and kept until it loses it (docs/issues/0004_kde-open-items-rationale.md D4).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct FocusVisible {
    /// Whether the control held the focus at its last render.
    focused: bool,
    /// `Window::last_input_was_keyboard` at the render that first saw it focused.
    keyboard: bool,
}

/// Whether a theme-drawn control that is `focused` draws the theme's focus ring, its
/// state keyed by `key` (the control's id and `"focus-visible"`), and `keyboard_only` the
/// theme's `defaults.focus_ring_keyboard_only == Some(true)`.
///
/// Where the theme says the platform shows its ring for keyboard focus only
/// (`defaults.focus_ring_keyboard_only`), a control draws it only for a focus that
/// followed a key press, decided when it gets the focus and kept until it loses it. The
/// answer is taken at render, so a control that loses the focus and gets it back with no
/// frame between keeps the first answer.
fn ring_shows(
    key: impl Into<ElementId>,
    focused: bool,
    keyboard_only: bool,
    window: &mut Window,
    cx: &mut App,
) -> bool {
    let state = window.use_keyed_state(key, cx, |_, _| FocusVisible::default());
    let last = *state.read(cx);
    let mut seen = last;
    if focused && !last.focused {
        seen.keyboard = window.last_input_was_keyboard();
    } else if !focused {
        seen.keyboard = false;
    }
    seen.focused = focused;
    if seen != last {
        state.update(cx, |stored, _| *stored = seen);
    }
    focused && (!keyboard_only || seen.keyboard)
}

/// Whether the window's present focus followed a key, for a control whose focus handles are
/// not its own (`TabBar`: gpui-base's `Tab` keeps each tab's): decided at the render that
/// first sees the focus on another element, and kept until it moves again
/// (docs/issues/0004_kde-open-items-rationale.md D4). Taken at render, as `ring_shows`' answer.
fn focus_followed_key(key: impl Into<ElementId>, window: &mut Window, cx: &mut App) -> bool {
    let now = window.focused(cx).map(|handle| handle.downgrade());
    let state = window.use_keyed_state(key, cx, |_, _| (None::<gpui::WeakFocusHandle>, false));
    let (stored, by_key) = state.read(cx).clone();
    if stored == now {
        return by_key;
    }
    let by_key = now.is_some() && window.last_input_was_keyboard();
    state.update(cx, |stored, _| *stored = (now, by_key));
    by_key
}

/// One colour of a control faded as one widget over `ground`, the colour
/// under the control: `colour` composited over `under`, the part it is
/// painted on (the ground itself, or a fill already composited over it),
/// then faded by `opacity` over `ground`. A platform that dims a disabled
/// control by opacity fades the finished control as a whole
/// (docs/platform-facts.md §2.1.6: libadwaita's `filter: Opacity(..)`),
/// where gpui fades each quad on its own (gpui-pre src/window.rs:4937-4945,
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

/// [`disabled_opacity`] of a 0.8.0 widget's `disabled_opacity`, which is
/// `None` where no source states a whole-widget fade: the control is then not
/// faded, 1, as where the platform dims by its disabled colours alone.
fn next_disabled_opacity(disabled: bool, opacity: Option<f32>) -> f32 {
    match opacity {
        Some(opacity) => disabled_opacity(disabled, opacity),
        None => 1.,
    }
}

/// A stated length in pixels, or `None` when it is not a finite,
/// non-negative number: the widget then falls back to gpui-component's own
/// control (spec §1.1).
fn length(v: f32) -> Option<Pixels> {
    (v.is_finite() && v >= 0.0).then(|| px(v))
}

/// A look a 0.8.0 constructor builds, or `Unmapped(home)` where a length the
/// theme gives at `home` is not a finite, non-negative number: the
/// counterpart of the 0.7.2 constructor's `None`.
fn finite<T>(look: Option<T>, home: &'static str) -> Mapped<T> {
    look.ok_or(Unmapped(home))
}

/// A colour of the 0.8.0 model as the 0.7.2 model's, its channels rounded to
/// eight bits ([`Color::to_srgb8`](native_theme::next::Color::to_srgb8)).
fn rgba(c: native_theme::next::Color) -> Rgba {
    let [r, g, b, a] = c.to_srgb8();
    Rgba::new(r, g, b, a)
}

/// The font a widget letters its label in: its family, size, weight and
/// style (gpui's of the same name), and the line box the model states.
#[derive(Clone, Debug, PartialEq)]
struct LabelFont {
    family: SharedString,
    size: Pixels,
    weight: FontWeight,
    style: gpui::FontStyle,
    /// The line height, a multiple of the size.
    line_height: f32,
}

impl LabelFont {
    /// `font` of a 0.7.2 theme, its size scaled by `n`'s text-scaling factor,
    /// one line `defaults.line_height` tall.
    fn old(font: &ResolvedFontSpec, n: Native<'_>) -> Self {
        Self {
            family: crate::font_family(&font.family),
            size: text_size(font.size, n),
            weight: FontWeight(f32::from(font.weight)),
            style: crate::font_style(font.style),
            line_height: n.resolved.defaults.line_height,
        }
    }

    /// `text` of a 0.8.0 theme, whose size its text scale has already scaled,
    /// one line `line_height` (`typography.line_height_multiplier`) tall; or
    /// `Unmapped(home)` where it leaves its family, size, weight or slant
    /// unstated.
    fn next(text: &TextStyle, line_height: f32, home: &'static str) -> Mapped<Self> {
        let font = crate::next::uncoloured_font(text, home)?;
        Ok(Self {
            family: crate::font_family(font.family),
            size: px(font.size),
            weight: FontWeight(f32::from(font.weight)),
            style: font.style,
            line_height,
        })
    }

    /// The text of a label in this font, one line tall.
    fn text(&self) -> gpui::Div {
        let mut text = div()
            .font_family(self.family.clone())
            .text_size(self.size)
            .font_weight(self.weight)
            .line_height(relative(self.line_height));
        text.text_style().font_style = Some(self.style);
        text
    }
}

/// A font size the text-scaling factor scales, as the `geometry` builders
/// scale theirs.
fn text_size(size: f32, n: Native<'_>) -> Pixels {
    px(crate::scaled_text_size(size, n.accessibility))
}

// ---------------------------------------------------------------------------
// The installed theme, of either model
// ---------------------------------------------------------------------------

use native_theme::next::{FocusTrigger, TextStated, TextStyle};

use crate::ActiveNativeTheme as _;
use crate::next::{Mapped, Unmapped};

#[cfg(test)]
mod parity;

/// The theme a theme-drawn control takes its look from: the variant installed
/// for the current mode, a 0.7.2 one [`apply`](crate::apply) installed, read
/// through [`ActiveNativeTheme::native`](crate::ActiveNativeTheme::native), or a
/// 0.8.0 one [`next::apply`](crate::next::apply) installed, read through
/// [`NativeTheme::next_resolved`](crate::NativeTheme::next_resolved). Every
/// control reads its look, its focus ring and the ground it is faded over
/// through it, so each is drawn from the theme of either model.
#[derive(Clone, Copy, Debug)]
enum ThemeSource<'a> {
    /// A 0.7.2 theme, with the preferences that scale its text sizes.
    Old(Native<'a>),
    /// A 0.8.0 theme, whose text sizes its text scale has already scaled.
    Next(&'a native_theme::next::ResolvedTheme),
}

impl<'a> ThemeSource<'a> {
    /// The theme installed for the current mode, or `None` where none is.
    fn of(cx: &'a App) -> Option<Self> {
        let installed = cx.native_theme()?;
        match installed.native(cx) {
            Some(n) => Some(Self::Old(n)),
            None => installed.next_resolved(cx).map(Self::Next),
        }
    }

    /// The look `old` gives a 0.7.2 theme, or `next` a 0.8.0 one: `None`
    /// where the constructor gives none, and the control is then drawn as
    /// gpui-component's own.
    fn look<T>(
        self,
        old: impl FnOnce(&native_theme::theme::ResolvedTheme) -> Option<T>,
        next: impl FnOnce(&native_theme::next::ResolvedTheme) -> Mapped<T>,
    ) -> Option<T> {
        match self {
            Self::Old(n) => old(n.resolved),
            Self::Next(r) => next(r).ok(),
        }
    }

    /// The window's background, the ground a control is faded over where its
    /// caller gives none: `defaults.background_color`
    /// (`colors.window.background`); `None` where the 0.8.0 theme states none.
    fn background(self) -> Option<Hsla> {
        match self {
            Self::Old(n) => Some(color(n.resolved.defaults.background_color)),
            Self::Next(r) => r.colors.window.background.map(crate::next::hsla),
        }
    }

    /// The theme's focus ring ([`FocusRing::of`], [`FocusRing::next_of`]).
    fn focus_ring(self) -> Option<FocusRing> {
        match self {
            Self::Old(n) => FocusRing::of(n.resolved),
            Self::Next(r) => FocusRing::next_of(r),
        }
    }

    /// Whether the platform shows its focus ring for keyboard focus only:
    /// `defaults.focus_ring_keyboard_only == Some(true)`
    /// (`focus_ring.trigger == Some(FocusTrigger::KeyboardOnly)`).
    fn keyboard_only(self) -> bool {
        match self {
            Self::Old(n) => n.resolved.defaults.focus_ring_keyboard_only == Some(true),
            Self::Next(r) => r.focus_ring.trigger == Some(FocusTrigger::KeyboardOnly),
        }
    }

    /// A label's font: the 0.7.2 font spec `old` picks, or the 0.8.0 text
    /// style `next` gives, at its home `home` ([`LabelFont`]).
    fn label_font(
        self,
        old: impl FnOnce(&native_theme::theme::ResolvedTheme) -> &ResolvedFontSpec,
        next: impl FnOnce(&native_theme::next::ResolvedTheme) -> TextStyle,
        home: &'static str,
    ) -> Option<LabelFont> {
        match self {
            Self::Old(n) => Some(LabelFont::old(old(n.resolved), n)),
            Self::Next(r) => {
                LabelFont::next(&next(r), r.typography.line_height_multiplier, home).ok()
            }
        }
    }

    /// The body text's size, line height and weight, for a label `SwitchTheme`
    /// gives no font of its own: `defaults.font.size`, scaled by the
    /// text-scaling factor, `defaults.line_height` and `defaults.font.weight`
    /// (`typography.body.size_px`, `typography.line_height_multiplier`,
    /// `typography.body.weight`); `None` where the 0.8.0 theme states no body
    /// size or weight.
    fn body_text(self) -> Option<(Pixels, f32, FontWeight)> {
        match self {
            Self::Old(n) => {
                let d = &n.resolved.defaults;
                Some((
                    text_size(d.font.size, n),
                    d.line_height,
                    FontWeight(f32::from(d.font.weight)),
                ))
            }
            Self::Next(r) => {
                let body = &r.typography.body;
                if !body.stated.contains(TextStated::SIZE_PX) {
                    return None;
                }
                let weight = crate::next::font_weight(r).ok()?;
                Some((
                    px(body.size_px),
                    r.typography.line_height_multiplier,
                    FontWeight(f32::from(weight)),
                ))
            }
        }
    }
}
