//! The widget [`crate::focus_ring()`] returns: the theme's focus ring,
//! drawn round a focused field after the field.
//!
//! Where the theme says keyboard only, the ring shows for a focus that
//! followed a key: the wrapper remembers whether the last key or pointer
//! event it saw was a key, and when its field gains the focus keeps that
//! answer until the field loses it. A column or a row hands every event to
//! every child, so the wrapper sees them while it exists. Two places keep an
//! event from it: a stack does not hand a lower layer what a higher one took
//! (iced_widget 0.14.2 `src/stack.rs:262-264`), and the base does not get
//! what an overlay took; a field under an open dialog can so remember a key
//! where a click came last, and show the ring once.

use iced_core::border::Radius;
use iced_core::layout::{self, Layout};
use iced_core::renderer::Quad;
use iced_core::widget::operation::Focusable;
use iced_core::widget::{Id, Operation, Tree, tree};
use iced_core::{
    Background, Border, Clipboard, Color, Element, Event, Length, Padding, Rectangle, Shell, Size,
    Theme, Vector, keyboard, mouse, overlay, renderer, window,
};
use native_theme::theme::ResolvedTheme;

use crate::palette::to_color;

/// The focus ring a focused field is drawn with, as the theme states it:
/// what [`crate::focus_ring()`] draws, read from the theme's
/// `defaults.focus_ring_*` ([`ring_look`]), and what
/// [`crate::next::focus_ring_with`] draws, read from the 0.8.0 model's
/// `focus_ring` ([`crate::next::ring_look`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RingLook {
    /// `defaults.focus_ring_color`.
    pub(crate) color: Color,
    /// `defaults.focus_ring_width`: positive and finite.
    pub(crate) width: f32,
    /// `defaults.focus_ring_offset`: finite; from the field's edge to the
    /// ring's inner edge, outward where positive.
    pub(crate) offset: f32,
    /// The field's corner radius, from which the ring's curves are grown.
    pub(crate) radius: f32,
    /// `defaults.focus_ring_keyboard_only` is `Some(true)`: the ring shows
    /// only for a focus that followed a key.
    pub(crate) keyboard_only: bool,
    /// The 0.8.0 model's `focus_ring.inner_stroke`: a second ring, its
    /// colour and its positive, finite width, inside the first; `None`
    /// where the theme states none (0.7.2 states none).
    pub(crate) inner: Option<(Color, f32)>,
    /// The 0.8.0 model's `focus_ring.corners`: the corners of the ring's
    /// inner edge, the line `offset` places (spec §2, `FocusIndicator`), as
    /// stated, in place of the field's grown; `None` where the theme states
    /// none (0.7.2 states none).
    pub(crate) corners: Option<Radius>,
}

/// [`crate::focus_ring()`]'s look: `None` where the theme states no ring
/// (a width that is not positive, or a width or offset that is not finite),
/// and where the ring is inset at the field's edge (`focus_ring_offset` is
/// minus `focus_ring_width`, libadwaita's), which `styles::text_input` and
/// `styles::text_editor` already draw as the focused field's border.
pub(crate) fn ring_look(resolved: &ResolvedTheme, radius: f32) -> Option<RingLook> {
    let d = &resolved.defaults;
    ring_look_from(
        to_color(d.focus_ring_color),
        d.focus_ring_width,
        d.focus_ring_offset,
        radius,
        d.focus_ring_keyboard_only == Some(true),
    )
}

/// [`ring_look`] of a ring `width` wide in `color`, `offset` out from the
/// field's edge, round a field of corner `radius`.
pub(crate) fn ring_look_from(
    color: Color,
    width: f32,
    offset: f32,
    radius: f32,
    keyboard_only: bool,
) -> Option<RingLook> {
    let drawn = width.is_finite() && offset.is_finite() && width > 0.0 && offset != -width;
    drawn.then(|| RingLook {
        color,
        width,
        offset,
        radius: if radius.is_finite() {
            radius.max(0.0)
        } else {
            0.0
        },
        keyboard_only,
        inner: None,
        corners: None,
    })
}

impl RingLook {
    /// This ring with a second ring inside it, `width` wide in `color`
    /// (`None`, and so no second ring, for a width that is not positive and
    /// finite), and the corners of its inner edge `corners` where `Some`.
    pub(crate) fn with(self, inner: Option<(Color, f32)>, corners: Option<Radius>) -> Self {
        Self {
            inner: inner.filter(|(_, width)| width.is_finite() && *width > 0.0),
            corners,
            ..self
        }
    }

    /// The quad that draws the ring round a field whose bounds are `field`,
    /// and its fill: iced draws a quad's border inside its bounds
    /// (iced_wgpu 0.14.0 `src/shader/quad/solid.wgsl:78-83`), so the bounds
    /// are the field's grown by `offset + width`, the ring's outer edge, the
    /// border `width` wide. The corners of the ring's inner edge, the line
    /// `offset` places, are the field's radius plus `offset`, floored at 0,
    /// or those the theme states ([`corners`](Self::corners)); the quad's
    /// radius is each grown by `width`, the outer edge concentric with the
    /// inner (spec §2, `FocusIndicator`: the geometry of a `Ring`). `None`
    /// where an inset ring leaves no room.
    pub(crate) fn quad(&self, field: Rectangle) -> Option<(Quad, Background)> {
        let out = self.offset + self.width;
        let grow = |corner: f32| corner.max(0.0) + self.width;
        let radius = self.corners.map_or_else(
            || Radius::from(grow(self.radius + self.offset)),
            |r| Radius {
                top_left: grow(r.top_left),
                top_right: grow(r.top_right),
                bottom_right: grow(r.bottom_right),
                bottom_left: grow(r.bottom_left),
            },
        );
        ring_quad(
            field.expand(Padding::new(out)),
            self.color,
            self.width,
            radius,
        )
    }

    /// The quad that draws the second ring ([`inner`](Self::inner)) round a
    /// field whose bounds are `field`: its outer edge the first ring's inner
    /// edge, the field's bounds grown by `offset`, and its radius the first
    /// ring's less the first ring's width, the curve concentric. `None`
    /// where the theme states no second ring, or an inset ring leaves no
    /// room.
    pub(crate) fn inner_quad(&self, field: Rectangle) -> Option<(Quad, Background)> {
        let (color, width) = self.inner?;
        let (outer, _) = self.quad(field)?;
        let r = outer.border.radius;
        let less = |corner: f32| (corner - self.width).max(0.0);
        ring_quad(
            field.expand(Padding::new(self.offset)),
            color,
            width,
            Radius {
                top_left: less(r.top_left),
                top_right: less(r.top_right),
                bottom_right: less(r.bottom_right),
                bottom_left: less(r.bottom_left),
            },
        )
    }
}

/// A ring of `bounds`, `width` wide in `color` with corners `radius`, as
/// iced draws a quad's border inside its bounds over a transparent fill;
/// `None` where the bounds are empty.
fn ring_quad(
    bounds: Rectangle,
    color: Color,
    width: f32,
    radius: Radius,
) -> Option<(Quad, Background)> {
    (bounds.width > 0.0 && bounds.height > 0.0).then(|| {
        (
            Quad {
                bounds,
                border: Border {
                    color,
                    width,
                    radius,
                },
                ..Quad::default()
            },
            Background::Color(Color::TRANSPARENT),
        )
    })
}

/// What the wrapper keeps between frames: whether a widget [`Probe`] went
/// through holds the keyboard focus, and whether the window holds the OS's
/// keyboard focus, which an iced field keeps its own focus through
/// (iced_widget 0.14.2 `src/text_input.rs:1313-1317`, `:1501-1503`), where
/// the platforms show no ring in a window that has lost it, as the egui
/// connector's ring shows none (its `plugin.rs`, `paint_focus_ring`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Focus {
    field: bool,
    window: bool,
    /// Whether the last key-or-pointer event the wrapper saw was a key
    /// press ([`key_or_pointer`]).
    key_last: bool,
    /// `key_last` at the moment the field gained the focus, kept until it
    /// loses it.
    by_key: bool,
}

impl Default for Focus {
    /// No field focused, in a window taken to hold the OS's focus until iced
    /// says it lost it (`window::Event::Unfocused`). iced keeps the flag in
    /// each wrapper's tree state and sends `Focused` only when the window's
    /// focus changes, so a wrapper built after the window gained it, a field
    /// on a page opened later, would never learn it held it; egui-winit's
    /// window state starts unfocused (egui-winit 0.36.2 `src/lib.rs:149`)
    /// because it is one per window. The corner case: a window opened
    /// without the OS's focus shows a focused field's ring until its first
    /// focus change. A field focused before the wrapper saw any input, or by
    /// the view that a key built, has seen no key and shows no ring where the
    /// theme says keyboard only.
    fn default() -> Self {
        Self {
            field: false,
            window: true,
            key_last: false,
            by_key: false,
        }
    }
}

impl Focus {
    /// Whether the ring is drawn; where `keyboard_only`, only for a focus
    /// that followed a key.
    fn ringed(self, keyboard_only: bool) -> bool {
        self.field && self.window && (!keyboard_only || self.by_key)
    }
}

/// Whether `event` is a key (`Some(true)`) or the pointer (`Some(false)`),
/// or neither (`None`), which leaves the last answer as it is. A key press
/// is a key, but a modifier key pressed alone: iced's winit conversion makes
/// every key a `KeyPressed`, Shift and Control included (iced_winit 0.14.0
/// `src/conversion.rs:291-302`), so the modifier block of iced_core 0.14.0's
/// `keyboard::key::Named` (`src/keyboard/key.rs:133-180`) is left out by
/// name. A pointer move, a pointer button press and a touch are the pointer;
/// every other event (a key release, a modifier change, a scroll, a button
/// release, a window event) is neither.
fn key_or_pointer(event: &Event) -> Option<bool> {
    use keyboard::key::{Key, Named};
    match event {
        Event::Keyboard(keyboard::Event::KeyPressed {
            key:
                Key::Named(
                    Named::Alt
                    | Named::AltGraph
                    | Named::CapsLock
                    | Named::Control
                    | Named::Fn
                    | Named::FnLock
                    | Named::NumLock
                    | Named::ScrollLock
                    | Named::Shift
                    | Named::Symbol
                    | Named::SymbolLock
                    | Named::Meta
                    | Named::Hyper
                    | Named::Super,
                ),
            ..
        }) => None,
        Event::Keyboard(keyboard::Event::KeyPressed { .. }) => Some(true),
        Event::Mouse(mouse::Event::CursorMoved { .. } | mouse::Event::ButtonPressed(_))
        | Event::Touch(_) => Some(false),
        Event::Keyboard(_) | Event::Mouse(_) | Event::Window(_) | Event::InputMethod(_) => None,
    }
}

/// Whether `event` can move the keyboard focus, so [`FocusRing`] asks the
/// field again after it: a mouse button, a key, a touch, or the window
/// gaining or losing the OS's focus (`window::Event::Focused`, `Unfocused`).
/// A pointer move, a scroll or any other window event cannot, and asking on
/// every one would have iced_aw's `NumberInput` rebuild its step elements on
/// each (iced_aw 0.14.1 `src/widget/number_input.rs:604-640`): iced hands
/// `update` a `RedrawRequested` on every frame (iced_winit 0.14.0
/// `src/lib.rs:814-833`).
fn may_move_focus(event: &Event) -> bool {
    match event {
        Event::Mouse(mouse::Event::ButtonPressed(_) | mouse::Event::ButtonReleased(_))
        | Event::Keyboard(_)
        | Event::Touch(_)
        | Event::Window(window::Event::Focused | window::Event::Unfocused) => true,
        Event::Mouse(_) | Event::Window(_) | Event::InputMethod(_) => false,
    }
}

/// The operation that asks the wrapped field whether it is focused, through
/// iced's own [`Focusable`] (the text input and the text editor report
/// themselves, iced_widget 0.14.2 `src/text_input.rs:691`,
/// `src/text_editor.rs:1097`); it changes nothing.
#[derive(Default)]
struct Probe {
    focused: bool,
}

impl Operation for Probe {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
        self.focused |= state.is_focused();
    }
}

/// `content`, and round it while it holds the keyboard focus in a window
/// that holds the OS's, the ring of `look` ([`crate::focus_ring()`]). Its
/// tree keeps the two ([`Focus`]), the content's focus learnt by [`Probe`]
/// each time the tree is operated on and after each update by an event that
/// can move it ([`may_move_focus`]), and taken as lost where the view puts a
/// widget of another kind in the content's place, whose new state holds no
/// focus; the window's from iced's `window::Event::Focused` and `Unfocused`,
/// with the content's tree its one child; every other method is the
/// content's. It asks the content nothing while it is laid out: the content
/// is not placed yet, and an operation hands each widget its layout as
/// placed in the window, which a widget that records its bounds as it is
/// operated on keeps.
///
/// Where the theme says keyboard only, the ring shows for a focus that
/// followed a key: the wrapper remembers whether the last key or pointer
/// event it saw was a key ([`key_or_pointer`]), and when its field gains the
/// focus keeps that answer until the field loses it (the module's doc says
/// which events can miss it).
pub(crate) struct FocusRing<'a, Message, Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
    /// `None` where the theme draws no ring ([`ring_look`]): the wrapper is
    /// there all the same, so the tree keeps its shape across themes.
    look: Option<RingLook>,
}

impl<'a, Message, Renderer> FocusRing<'a, Message, Renderer> {
    pub(crate) fn new(
        content: Element<'a, Message, Theme, Renderer>,
        look: Option<RingLook>,
    ) -> Self {
        Self { content, look }
    }

    /// Whether the theme says the ring shows only for a focus that followed
    /// a key ([`RingLook::keyboard_only`]).
    fn keyboard_only(&self) -> bool {
        self.look.is_some_and(|look| look.keyboard_only)
    }

    /// Asks the content in `tree` laid out as `layout` whether it holds the
    /// focus, and keeps the answer in the wrapper's state ([`focus_mut`]).
    fn learn(&mut self, tree: &mut Tree, layout: Layout<'_>, renderer: &Renderer)
    where
        Renderer: iced_core::Renderer,
    {
        let Some(child) = tree.children.first_mut() else {
            return;
        };
        let mut probe = Probe::default();
        self.content
            .as_widget_mut()
            .operate(child, layout, renderer, &mut probe);
        if let Some(focus) = focus_mut(tree) {
            if !probe.focused {
                focus.by_key = false;
            } else if !focus.field {
                focus.by_key = focus.key_last;
            }
            focus.field = probe.focused;
        }
    }
}

/// The wrapper's state in `tree`. `tree::State`'s own downcasts panic on a
/// state of another type (iced_core 0.14.0 `src/widget/tree.rs:231-241`);
/// this reads it through its variant, `None` for another type.
fn focus_mut(tree: &mut Tree) -> Option<&mut Focus> {
    match &mut tree.state {
        tree::State::Some(state) => state.downcast_mut::<Focus>(),
        tree::State::None => None,
    }
}

/// Whether the wrapper's state in `tree` says the ring is drawn: its content
/// focused, in a window that holds the focus, and where `keyboard_only`, by
/// a focus that followed a key.
fn ringed(tree: &Tree, keyboard_only: bool) -> bool {
    match &tree.state {
        tree::State::Some(state) => state
            .downcast_ref::<Focus>()
            .is_some_and(|f| f.ringed(keyboard_only)),
        tree::State::None => false,
    }
}

impl<Message, Renderer> iced_core::Widget<Message, Theme, Renderer>
    for FocusRing<'_, Message, Renderer>
where
    Renderer: iced_core::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Focus>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(Focus::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        let before = tree.children.first().map(|child| child.tag);
        tree.diff_children(std::slice::from_ref(&self.content));
        let after = tree.children.first().map(|child| child.tag);
        if before != after
            && let Some(focus) = focus_mut(tree)
        {
            focus.field = false;
            focus.by_key = false;
        }
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.content.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let Some(child) = tree.children.first_mut() else {
            return layout::Node::new(Size::ZERO);
        };
        self.content.as_widget_mut().layout(child, renderer, limits)
    }

    /// `content`, then, while it holds the focus, the ring over it in the
    /// layer `content` is drawn in.
    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let Some(child) = tree.children.first() else {
            return;
        };
        self.content
            .as_widget()
            .draw(child, renderer, theme, style, layout, cursor, viewport);
        if ringed(tree, self.keyboard_only())
            && let Some(look) = self.look
        {
            let field = layout.bounds();
            for (quad, background) in [look.quad(field), look.inner_quad(field)]
                .into_iter()
                .flatten()
            {
                renderer.fill_quad(quad, background);
            }
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        if let Some(child) = tree.children.first_mut() {
            self.content
                .as_widget_mut()
                .operate(child, layout, renderer, operation);
        }
        self.learn(tree, layout, renderer);
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if let Some(key) = key_or_pointer(event)
            && let Some(focus) = focus_mut(tree)
        {
            focus.key_last = key;
        }
        let Some(child) = tree.children.first_mut() else {
            return;
        };
        self.content.as_widget_mut().update(
            child, event, layout, cursor, renderer, clipboard, shell, viewport,
        );
        if !may_move_focus(event) {
            return;
        }
        let keyboard_only = self.keyboard_only();
        let was = ringed(tree, keyboard_only);
        if let Event::Window(window_event) = event
            && let Some(focus) = focus_mut(tree)
        {
            match window_event {
                window::Event::Focused => focus.window = true,
                window::Event::Unfocused => focus.window = false,
                _ => {}
            }
        }
        self.learn(tree, layout, renderer);
        if ringed(tree, keyboard_only) != was {
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        tree.children
            .first()
            .map_or(mouse::Interaction::None, |child| {
                self.content
                    .as_widget()
                    .mouse_interaction(child, layout, cursor, viewport, renderer)
            })
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let child = tree.children.first_mut()?;
        self.content
            .as_widget_mut()
            .overlay(child, layout, renderer, viewport, translation)
    }
}

impl<'a, Message, Renderer> From<FocusRing<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_core::Renderer + 'a,
{
    fn from(ring: FocusRing<'a, Message, Renderer>) -> Self {
        Element::new(ring)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use iced_core::widget::Widget;
    use native_theme::theme::ColorMode;

    /// `preset` in `mode`, resolved.
    fn resolved(preset: &str, mode: ColorMode) -> ResolvedTheme {
        native_theme::theme::Theme::preset(preset)
            .unwrap()
            .resolve(mode)
            .unwrap()
            .variant
    }

    /// What a [`Recorder`] was asked to fill, in order.
    #[derive(Default)]
    struct Recorder(Vec<(Quad, Background)>);

    impl iced_core::Renderer for Recorder {
        fn start_layer(&mut self, _: Rectangle) {}

        fn end_layer(&mut self) {}

        fn start_transformation(&mut self, _: iced_core::Transformation) {}

        fn end_transformation(&mut self) {}

        fn fill_quad(&mut self, quad: Quad, background: impl Into<Background>) {
            self.0.push((quad, background.into()));
        }

        fn reset(&mut self, _: Rectangle) {}

        fn allocate_image(
            &mut self,
            _: &iced_core::image::Handle,
            _: impl FnOnce(Result<iced_core::image::Allocation, iced_core::image::Error>)
            + Send
            + 'static,
        ) {
        }
    }

    /// A field's focus, as iced's widgets keep it.
    struct Held(bool);

    impl Focusable for Held {
        fn is_focused(&self) -> bool {
            self.0
        }

        fn focus(&mut self) {
            self.0 = true;
        }

        fn unfocus(&mut self) {
            self.0 = false;
        }
    }

    /// A 100 by 30 field that reports its focus, `focused`, to an operation
    /// as a text input does, and draws nothing.
    struct Field {
        focused: bool,
    }

    impl Widget<(), Theme, Recorder> for Field {
        fn size(&self) -> Size<Length> {
            Size::new(Length::Fixed(100.0), Length::Fixed(30.0))
        }

        fn layout(&mut self, _: &mut Tree, _: &Recorder, _: &layout::Limits) -> layout::Node {
            layout::Node::new(Size::new(100.0, 30.0))
        }

        fn draw(
            &self,
            _: &Tree,
            _: &mut Recorder,
            _: &Theme,
            _: &renderer::Style,
            _: Layout<'_>,
            _: mouse::Cursor,
            _: &Rectangle,
        ) {
        }

        fn operate(
            &mut self,
            _: &mut Tree,
            layout: Layout<'_>,
            _: &Recorder,
            operation: &mut dyn Operation,
        ) {
            operation.focusable(None, layout.bounds(), &mut Held(self.focused));
        }
    }

    /// The field, `focused` or not, wrapped in [`crate::focus_ring()`] under
    /// `r` with a corner radius of 4, laid out at (10, 20), operated on, as
    /// iced's focus operations operate on the tree, and drawn: the quads it
    /// filled.
    fn drawn(r: &ResolvedTheme, focused: bool) -> Vec<(Quad, Background)> {
        let mut element: Element<'_, (), Theme, Recorder> =
            crate::focus_ring(r, 4.0, Element::new(Field { focused }));
        let mut tree = Tree::new(&element);
        let mut recorder = Recorder::default();
        let node = element
            .as_widget_mut()
            .layout(
                &mut tree,
                &recorder,
                &layout::Limits::new(Size::ZERO, Size::new(500.0, 100.0)),
            )
            .move_to(iced_core::Point::new(10.0, 20.0));
        operated(&mut element, &mut tree, &node);
        element.as_widget().draw(
            &tree,
            &mut recorder,
            &Theme::Light,
            &renderer::Style {
                text_color: Color::BLACK,
            },
            Layout::new(&node),
            mouse::Cursor::Unavailable,
            &Rectangle::new(iced_core::Point::ORIGIN, Size::new(500.0, 100.0)),
        );
        recorder.0
    }

    /// Under macos-tahoe (3 at -1) and windows-11 (2 at +1), in both modes,
    /// a focused field is ringed by one quad: its bounds the field's grown
    /// by `offset + width`, its border `width` wide in
    /// `defaults.focus_ring_color`, its radius the field's 4 grown as much,
    /// its fill transparent; an unfocused field by none. The geometry, with
    /// the keyboard rule off: the field is focused before any input, so
    /// windows-11's `defaults.focus_ring_keyboard_only` is set to `None`.
    #[test]
    fn a_focused_field_is_ringed_by_the_themes_ring() {
        for preset in ["macos-tahoe", "windows-11"] {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let mut r = resolved(preset, mode);
                r.defaults.focus_ring_keyboard_only = None;
                let d = &r.defaults;
                let out = d.focus_ring_offset + d.focus_ring_width;
                let quads = drawn(&r, true);
                assert_eq!(quads.len(), 1, "{preset} {mode:?}: {quads:?}");
                let (quad, background) = quads[0];
                assert_eq!(
                    quad.bounds,
                    Rectangle::new(
                        iced_core::Point::new(10.0 - out, 20.0 - out),
                        Size::new(100.0 + 2.0 * out, 30.0 + 2.0 * out)
                    ),
                    "{preset} {mode:?}"
                );
                assert_eq!(quad.border.width, d.focus_ring_width, "{preset} {mode:?}");
                assert_eq!(
                    quad.border.color,
                    to_color(d.focus_ring_color),
                    "{preset} {mode:?}"
                );
                assert_eq!(quad.border.radius, (4.0 + out).into(), "{preset} {mode:?}");
                assert_eq!(background, Background::Color(Color::TRANSPARENT));
                assert!(drawn(&r, false).is_empty(), "{preset} {mode:?} unfocused");
            }
        }
    }

    /// No ring is drawn where the theme states none, or where it is inset
    /// at the edge, which `styles::text_input` draws as the border: adwaita
    /// (2 at -2), a width of 0, and a width or offset that is not finite.
    #[test]
    fn no_ring_is_drawn_where_the_border_is_the_ring_or_none_is_stated() {
        let adwaita = resolved("adwaita", ColorMode::Light);
        assert_eq!(
            adwaita.defaults.focus_ring_offset,
            -adwaita.defaults.focus_ring_width
        );
        assert_eq!(ring_look(&adwaita, 4.0), None, "adwaita's inset ring");
        let tahoe = resolved("macos-tahoe", ColorMode::Light);
        assert!(ring_look(&tahoe, 4.0).is_some());
        for (width, offset) in [(0.0, 1.0), (f32::NAN, 1.0), (2.0, f32::INFINITY)] {
            let mut r = tahoe.clone();
            r.defaults.focus_ring_width = width;
            r.defaults.focus_ring_offset = offset;
            assert_eq!(ring_look(&r, 4.0), None, "{width} at {offset}");
            assert!(drawn(&r, true).is_empty(), "{width} at {offset}");
        }
    }

    /// A 100 by 30 field that keeps its focus in its own tree state, as a
    /// text input does, takes it on a left press and reports it to an
    /// operation.
    struct Clicked;

    impl Widget<(), Theme, Recorder> for Clicked {
        fn tag(&self) -> tree::Tag {
            tree::Tag::of::<Held>()
        }

        fn state(&self) -> tree::State {
            tree::State::new(Held(false))
        }

        fn size(&self) -> Size<Length> {
            Size::new(Length::Fixed(100.0), Length::Fixed(30.0))
        }

        fn layout(&mut self, _: &mut Tree, _: &Recorder, _: &layout::Limits) -> layout::Node {
            layout::Node::new(Size::new(100.0, 30.0))
        }

        fn draw(
            &self,
            _: &Tree,
            _: &mut Recorder,
            _: &Theme,
            _: &renderer::Style,
            _: Layout<'_>,
            _: mouse::Cursor,
            _: &Rectangle,
        ) {
        }

        fn update(
            &mut self,
            tree: &mut Tree,
            event: &Event,
            _: Layout<'_>,
            _: mouse::Cursor,
            _: &Recorder,
            _: &mut dyn Clipboard,
            _: &mut Shell<'_, ()>,
            _: &Rectangle,
        ) {
            if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
                tree.state.downcast_mut::<Held>().focus();
            }
        }

        fn operate(
            &mut self,
            tree: &mut Tree,
            layout: Layout<'_>,
            _: &Recorder,
            operation: &mut dyn Operation,
        ) {
            operation.focusable(None, layout.bounds(), tree.state.downcast_mut::<Held>());
        }
    }

    /// `element` in `tree`, laid out as `node`, operated on by an operation
    /// that changes nothing.
    fn operated(
        element: &mut Element<'_, (), Theme, Recorder>,
        tree: &mut Tree,
        node: &layout::Node,
    ) {
        element.as_widget_mut().operate(
            tree,
            Layout::new(node),
            &Recorder::default(),
            &mut Probe::default(),
        );
    }

    /// `element` in `tree`, laid out as `node`, operated on by `operation`,
    /// and again by each operation its `finish` chains to, as iced's runtime
    /// runs one (iced_winit 0.14.0 `src/lib.rs:1745-1758`; iced_test 0.14.0
    /// `src/emulator.rs:195-205`): `focus_next` first counts the focusable
    /// widgets, then moves the focus.
    fn operate_chained(
        element: &mut Element<'_, (), Theme, Recorder>,
        tree: &mut Tree,
        node: &layout::Node,
        operation: impl Operation + 'static,
    ) {
        let mut current: Box<dyn Operation> = Box::new(operation);
        loop {
            element.as_widget_mut().operate(
                tree,
                Layout::new(node),
                &Recorder::default(),
                current.as_mut(),
            );
            let iced_core::widget::operation::Outcome::Chain(next) = current.finish() else {
                break;
            };
            current = next;
        }
    }

    /// `element` laid out in `tree` at the origin: its node.
    fn laid(element: &mut Element<'_, (), Theme, Recorder>, tree: &mut Tree) -> layout::Node {
        element.as_widget_mut().layout(
            tree,
            &Recorder::default(),
            &layout::Limits::new(Size::ZERO, Size::new(500.0, 100.0)),
        )
    }

    /// `event` handed to `element` in `tree`, laid out as `node`.
    fn send(
        element: &mut Element<'_, (), Theme, Recorder>,
        tree: &mut Tree,
        node: &layout::Node,
        event: Event,
    ) {
        let mut messages = Vec::new();
        let mut shell = Shell::new(&mut messages);
        element.as_widget_mut().update(
            tree,
            &event,
            Layout::new(node),
            mouse::Cursor::Unavailable,
            &Recorder::default(),
            &mut iced_core::clipboard::Null,
            &mut shell,
            &Rectangle::new(iced_core::Point::ORIGIN, Size::new(500.0, 100.0)),
        );
    }

    /// The quads `element` in `tree`, laid out as `node`, draws.
    fn painted(
        element: &Element<'_, (), Theme, Recorder>,
        tree: &Tree,
        node: &layout::Node,
    ) -> usize {
        let mut recorder = Recorder::default();
        element.as_widget().draw(
            tree,
            &mut recorder,
            &Theme::Light,
            &renderer::Style {
                text_color: Color::BLACK,
            },
            Layout::new(node),
            mouse::Cursor::Unavailable,
            &Rectangle::new(iced_core::Point::ORIGIN, Size::new(500.0, 100.0)),
        );
        recorder.0.len()
    }

    /// The ring follows the focus as `update` moves it: under macos-tahoe a
    /// field focused by a left press is ringed, a pointer move asks nothing
    /// and changes nothing, and while the window has lost the OS's focus
    /// (`window::Event::Unfocused`) no ring is drawn though the field keeps
    /// its own, until the window has it back (`Focused`).
    #[test]
    fn the_ring_follows_the_focus_through_update() {
        let r = resolved("macos-tahoe", ColorMode::Light);
        let mut element: Element<'_, (), Theme, Recorder> =
            crate::focus_ring(&r, 4.0, Element::new(Clicked));
        let mut tree = Tree::new(&element);
        let node = laid(&mut element, &mut tree);
        assert_eq!(painted(&element, &tree, &node), 0, "unfocused at first");
        let moved = Event::Mouse(mouse::Event::CursorMoved {
            position: iced_core::Point::new(5.0, 5.0),
        });
        send(&mut element, &mut tree, &node, moved);
        assert_eq!(
            painted(&element, &tree, &node),
            0,
            "a pointer move rings nothing"
        );
        let press = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
        send(&mut element, &mut tree, &node, press);
        assert_eq!(painted(&element, &tree, &node), 1, "focused by the press");
        send(
            &mut element,
            &mut tree,
            &node,
            Event::Window(window::Event::Unfocused),
        );
        assert_eq!(
            painted(&element, &tree, &node),
            0,
            "the window lost the focus"
        );
        send(
            &mut element,
            &mut tree,
            &node,
            Event::Window(window::Event::Focused),
        );
        assert_eq!(painted(&element, &tree, &node), 1, "the window has it back");
    }

    /// A 100 by 30 field that counts the operations it is handed, the
    /// wrapper's [`Probe`] among them.
    struct Asked(std::rc::Rc<std::cell::Cell<u32>>);

    impl Widget<(), Theme, Recorder> for Asked {
        fn size(&self) -> Size<Length> {
            Size::new(Length::Fixed(100.0), Length::Fixed(30.0))
        }

        fn layout(&mut self, _: &mut Tree, _: &Recorder, _: &layout::Limits) -> layout::Node {
            layout::Node::new(Size::new(100.0, 30.0))
        }

        fn draw(
            &self,
            _: &Tree,
            _: &mut Recorder,
            _: &Theme,
            _: &renderer::Style,
            _: Layout<'_>,
            _: mouse::Cursor,
            _: &Rectangle,
        ) {
        }

        fn operate(
            &mut self,
            _: &mut Tree,
            layout: Layout<'_>,
            _: &Recorder,
            operation: &mut dyn Operation,
        ) {
            self.0.set(self.0.get() + 1);
            operation.focusable(None, layout.bounds(), &mut Held(false));
        }
    }

    /// The field is asked again after the window events that move the OS's
    /// focus, `Focused` and `Unfocused`, and after no other: iced hands
    /// `update` a `RedrawRequested` on every frame (iced_winit 0.14.0
    /// `src/lib.rs:814-833`), and a resize or a move changes no focus.
    #[test]
    fn only_the_windows_focus_events_ask_the_field_again() {
        let r = resolved("windows-11", ColorMode::Light);
        let asked = std::rc::Rc::new(std::cell::Cell::new(0));
        let mut element: Element<'_, (), Theme, Recorder> =
            crate::focus_ring(&r, 4.0, Element::new(Asked(asked.clone())));
        let mut tree = Tree::new(&element);
        let node = laid(&mut element, &mut tree);
        let quiet = [
            window::Event::RedrawRequested(iced_core::time::Instant::now()),
            window::Event::Resized(Size::new(800.0, 600.0)),
            window::Event::Moved(iced_core::Point::new(5.0, 5.0)),
        ];
        for event in quiet {
            let before = asked.get();
            send(&mut element, &mut tree, &node, Event::Window(event.clone()));
            assert_eq!(asked.get(), before, "{event:?} asks the field");
        }
        for event in [window::Event::Unfocused, window::Event::Focused] {
            let before = asked.get();
            send(&mut element, &mut tree, &node, Event::Window(event.clone()));
            assert_eq!(asked.get(), before + 1, "{event:?} does not ask the field");
        }
    }

    /// Laying the wrapper out hands its field no operation: the field is not
    /// placed yet, and a widget that records the bounds it is operated with
    /// (the showcases' layout dump) kept them at the origin, where the
    /// layout dump of the controller's parity run on d12398a8 found the
    /// iced Basic page's ring-wrapped fields (M9-172).
    #[test]
    fn laying_the_ring_out_asks_its_field_nothing() {
        let r = resolved("windows-11", ColorMode::Light);
        let asked = std::rc::Rc::new(std::cell::Cell::new(0));
        let mut element: Element<'_, (), Theme, Recorder> =
            crate::focus_ring(&r, 4.0, Element::new(Asked(asked.clone())));
        let mut tree = Tree::new(&element);
        for _ in 0..3 {
            let _ = laid(&mut element, &mut tree);
        }
        assert_eq!(
            asked.get(),
            0,
            "the field was operated on as it was laid out"
        );
    }

    /// A focused field keeps its focus where the theme changes between one
    /// whose ring is the field's border (adwaita, no ring drawn) and one that
    /// rings it (windows-11 with the keyboard rule off, so a ring shows the
    /// focus whatever input gave it), both ways: [`crate::focus_ring()`]
    /// wraps the field whatever the theme states, so the tree keeps its shape
    /// and iced keeps the field's state (round 5 review, item 18 M4).
    #[test]
    fn the_focus_survives_a_theme_that_draws_no_ring() {
        let adwaita = resolved("adwaita", ColorMode::Light);
        let mut windows = resolved("windows-11", ColorMode::Light);
        windows.defaults.focus_ring_keyboard_only = None;
        for (first, second) in [(&windows, &adwaita), (&adwaita, &windows)] {
            let mut element: Element<'_, (), Theme, Recorder> =
                crate::focus_ring(first, 4.0, Element::new(Clicked));
            let mut tree = Tree::new(&element);
            let node = laid(&mut element, &mut tree);
            let press = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
            send(&mut element, &mut tree, &node, press);
            for theme in [second, &windows] {
                let mut next: Element<'_, (), Theme, Recorder> =
                    crate::focus_ring(theme, 4.0, Element::new(Clicked));
                tree.diff(&next);
                let node = laid(&mut next, &mut tree);
                operated(&mut next, &mut tree, &node);
                element = next;
            }
            let node = laid(&mut element, &mut tree);
            assert_eq!(
                painted(&element, &tree, &node),
                1,
                "the field focused under the first theme lost its focus through the second"
            );
        }
    }

    /// Where the view replaces the field with a widget of another kind, whose
    /// new state holds no focus, the ring goes: a focused field's ring goes
    /// once the tree holds the other widget in its place. Under windows-11
    /// with the keyboard rule off, so a ring shows the focus whatever input
    /// gave it.
    #[test]
    fn a_replaced_field_loses_the_ring() {
        let mut r = resolved("windows-11", ColorMode::Light);
        r.defaults.focus_ring_keyboard_only = None;
        let mut element: Element<'_, (), Theme, Recorder> =
            crate::focus_ring(&r, 4.0, Element::new(Field { focused: true }));
        let mut tree = Tree::new(&element);
        let node = laid(&mut element, &mut tree);
        operated(&mut element, &mut tree, &node);
        assert_eq!(
            painted(&element, &tree, &node),
            1,
            "the focused field is ringed"
        );
        let mut replaced: Element<'_, (), Theme, Recorder> =
            crate::focus_ring(&r, 4.0, Element::new(Clicked));
        tree.diff(&replaced);
        let node = laid(&mut replaced, &mut tree);
        assert_eq!(
            painted(&replaced, &tree, &node),
            0,
            "the unfocused widget in the field's place is ringed"
        );
    }

    /// A key press of `key`, with no modifier held.
    fn key(key: keyboard::Key) -> Event {
        Event::Keyboard(keyboard::Event::KeyPressed {
            key: key.clone(),
            modified_key: key,
            physical_key: keyboard::key::Physical::Unidentified(
                keyboard::key::NativeCode::Unidentified,
            ),
            location: keyboard::Location::Standard,
            modifiers: keyboard::Modifiers::empty(),
            text: None,
            repeat: false,
        })
    }

    /// Where the theme says keyboard only (windows-11), a field a left press
    /// focuses draws no ring, nor after a key while it keeps that focus;
    /// unfocused, then focused by `focus_next` after
    /// a Tab, it is ringed, and a pointer move keeps the ring; unfocused
    /// again and focused by a left press, it draws none.
    #[test]
    fn where_the_theme_says_keyboard_only_a_click_draws_no_ring() {
        use iced_core::widget::operation::focusable;
        let r = resolved("windows-11", ColorMode::Light);
        assert_eq!(r.defaults.focus_ring_keyboard_only, Some(true));
        let mut element: Element<'_, (), Theme, Recorder> =
            crate::focus_ring(&r, 4.0, Element::new(Clicked));
        let mut tree = Tree::new(&element);
        let node = laid(&mut element, &mut tree);
        let press = || Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
        send(&mut element, &mut tree, &node, press());
        assert_eq!(painted(&element, &tree, &node), 0, "focused by a click");
        send(
            &mut element,
            &mut tree,
            &node,
            key(keyboard::Key::Character("a".into())),
        );
        assert_eq!(
            painted(&element, &tree, &node),
            0,
            "a key while the click's focus holds"
        );
        operate_chained(&mut element, &mut tree, &node, focusable::unfocus());
        send(
            &mut element,
            &mut tree,
            &node,
            key(keyboard::Key::Named(keyboard::key::Named::Tab)),
        );
        operate_chained(&mut element, &mut tree, &node, focusable::focus_next());
        assert_eq!(painted(&element, &tree, &node), 1, "focused after Tab");
        let moved = Event::Mouse(mouse::Event::CursorMoved {
            position: iced_core::Point::new(5.0, 5.0),
        });
        send(&mut element, &mut tree, &node, moved);
        assert_eq!(
            painted(&element, &tree, &node),
            1,
            "a pointer move took the ring"
        );
        operate_chained(&mut element, &mut tree, &node, focusable::unfocus());
        assert_eq!(painted(&element, &tree, &node), 0, "unfocused");
        send(&mut element, &mut tree, &node, press());
        assert_eq!(
            painted(&element, &tree, &node),
            0,
            "focused by a click again"
        );
    }

    /// Under kde-breeze, which says keyboard only, the field focused before
    /// the wrapper saw any input draws no quad.
    #[test]
    fn a_field_focused_before_any_input_shows_no_ring_where_keyboard_only() {
        for mode in [ColorMode::Light, ColorMode::Dark] {
            let r = resolved("kde-breeze", mode);
            assert_eq!(r.defaults.focus_ring_keyboard_only, Some(true), "{mode:?}");
            assert!(ring_look(&r, 4.0).is_some(), "{mode:?}");
            assert!(drawn(&r, true).is_empty(), "{mode:?}");
        }
    }

    /// A modifier key pressed alone is no key (windows-11): a field focused
    /// through `operate` after Shift or Control draws no ring, one after Tab
    /// draws it, and so does one after Tab and then Shift alone: a modifier
    /// leaves the answer as it is, it is not the pointer.
    #[test]
    fn a_modifier_key_alone_is_no_key() {
        use iced_core::widget::operation::focusable;
        use keyboard::key::Named;
        let r = resolved("windows-11", ColorMode::Light);
        let runs: [(&[Named], usize); 4] = [
            (&[Named::Shift], 0),
            (&[Named::Control], 0),
            (&[Named::Tab], 1),
            (&[Named::Tab, Named::Shift], 1),
        ];
        for (keys, rings) in runs {
            let mut element: Element<'_, (), Theme, Recorder> =
                crate::focus_ring(&r, 4.0, Element::new(Clicked));
            let mut tree = Tree::new(&element);
            let node = laid(&mut element, &mut tree);
            for named in keys {
                send(
                    &mut element,
                    &mut tree,
                    &node,
                    key(keyboard::Key::Named(*named)),
                );
            }
            operate_chained(&mut element, &mut tree, &node, focusable::focus_next());
            assert_eq!(painted(&element, &tree, &node), rings, "{keys:?}");
        }
    }
}
