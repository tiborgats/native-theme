//! The widget [`crate::focus_ring()`] returns: the theme's focus ring,
//! drawn round a focused field after the field.

use iced_core::layout::{self, Layout};
use iced_core::renderer::Quad;
use iced_core::widget::operation::Focusable;
use iced_core::widget::{Id, Operation, Tree, tree};
use iced_core::{
    Background, Border, Clipboard, Color, Element, Event, Length, Padding, Rectangle, Shell, Size,
    Theme, Vector, mouse, overlay, renderer, window,
};
use native_theme::theme::ResolvedTheme;

use crate::palette::to_color;

/// What [`crate::focus_ring()`] draws, read from the theme's
/// `defaults.focus_ring_*` ([`ring_look`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct RingLook {
    /// `defaults.focus_ring_color`.
    pub(crate) color: Color,
    /// `defaults.focus_ring_width`: positive and finite.
    pub(crate) width: f32,
    /// `defaults.focus_ring_offset`: finite; from the field's edge to the
    /// ring's inner edge, outward where positive.
    pub(crate) offset: f32,
    /// The field's corner radius, from which the ring's curves are grown.
    pub(crate) radius: f32,
}

/// [`crate::focus_ring()`]'s look: `None` where the theme states no ring
/// (a width that is not positive, or a width or offset that is not finite),
/// and where the ring is inset at the field's edge (`focus_ring_offset` is
/// minus `focus_ring_width`, libadwaita's), which `styles::text_input` and
/// `styles::text_editor` already draw as the focused field's border.
pub(crate) fn ring_look(resolved: &ResolvedTheme, radius: f32) -> Option<RingLook> {
    let d = &resolved.defaults;
    let (width, offset) = (d.focus_ring_width, d.focus_ring_offset);
    let drawn = width.is_finite() && offset.is_finite() && width > 0.0 && offset != -width;
    drawn.then(|| RingLook {
        color: to_color(d.focus_ring_color),
        width,
        offset,
        radius: if radius.is_finite() {
            radius.max(0.0)
        } else {
            0.0
        },
    })
}

impl RingLook {
    /// The quad that draws the ring round a field whose bounds are `field`,
    /// and its fill: iced draws a quad's border inside its bounds
    /// (iced_wgpu 0.14.0 `src/shader/quad/solid.wgsl:78-83`), so the bounds
    /// are the field's grown by `offset + width`, the ring's outer edge, the
    /// border `width` wide and the radius the field's grown as much, the
    /// curve concentric with the field's, as the egui connector strokes the
    /// ring outside the field's grown by `offset` (its `plugin.rs`,
    /// `paint_focus_ring`). `None` where an inset ring leaves no room.
    pub(crate) fn quad(&self, field: Rectangle) -> Option<(Quad, Background)> {
        let out = self.offset + self.width;
        let bounds = field.expand(Padding::new(out));
        (bounds.width > 0.0 && bounds.height > 0.0).then(|| {
            (
                Quad {
                    bounds,
                    border: Border {
                        color: self.color,
                        width: self.width,
                        radius: (self.radius + out).max(0.0).into(),
                    },
                    ..Quad::default()
                },
                Background::Color(Color::TRANSPARENT),
            )
        })
    }
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
}

impl Default for Focus {
    /// No field focused, in a window taken to hold the focus until iced says
    /// it lost it.
    fn default() -> Self {
        Self {
            field: false,
            window: true,
        }
    }
}

impl Focus {
    /// Whether the ring is drawn.
    fn ringed(self) -> bool {
        self.field && self.window
    }
}

/// Whether `event` can move the keyboard focus, so [`FocusRing`] asks the
/// field again after it: a mouse button, a key, a touch, or a window event.
/// A pointer move or a scroll cannot, and asking on every one would have
/// iced_aw's `NumberInput` rebuild its step elements on each (iced_aw 0.14.1
/// `src/widget/number_input.rs:604-640`).
fn may_move_focus(event: &Event) -> bool {
    match event {
        Event::Mouse(mouse::Event::ButtonPressed(_) | mouse::Event::ButtonReleased(_))
        | Event::Keyboard(_)
        | Event::Touch(_)
        | Event::Window(_) => true,
        Event::Mouse(_) | Event::InputMethod(_) => false,
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
/// each time the tree is laid out or operated on and after each update by an
/// event that can move it ([`may_move_focus`]), the window's from iced's
/// `window::Event::Focused` and `Unfocused`, with the content's tree its one
/// child; every other method is the content's.
pub(crate) struct FocusRing<'a, Message, Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
    look: RingLook,
}

impl<'a, Message, Renderer> FocusRing<'a, Message, Renderer> {
    pub(crate) fn new(content: Element<'a, Message, Theme, Renderer>, look: RingLook) -> Self {
        Self { content, look }
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
/// focused, in a window that holds the focus.
fn ringed(tree: &Tree) -> bool {
    match &tree.state {
        tree::State::Some(state) => state.downcast_ref::<Focus>().is_some_and(|f| f.ringed()),
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
        tree.diff_children(std::slice::from_ref(&self.content));
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
        let node = self.content.as_widget_mut().layout(child, renderer, limits);
        self.learn(tree, Layout::new(&node), renderer);
        node
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
        if ringed(tree)
            && let Some((quad, background)) = self.look.quad(layout.bounds())
        {
            renderer.fill_quad(quad, background);
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
        let Some(child) = tree.children.first_mut() else {
            return;
        };
        self.content.as_widget_mut().update(
            child, event, layout, cursor, renderer, clipboard, shell, viewport,
        );
        if !may_move_focus(event) {
            return;
        }
        let was = ringed(tree);
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
        if ringed(tree) != was {
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
    /// `r` with a corner radius of 4, laid out at (10, 20) and drawn: the
    /// quads it filled.
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
    /// its fill transparent; an unfocused field by none.
    #[test]
    fn a_focused_field_is_ringed_by_the_themes_ring() {
        for preset in ["macos-tahoe", "windows-11"] {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let r = resolved(preset, mode);
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

    /// The ring follows the focus as `update` moves it: under windows-11 a
    /// field focused by a left press is ringed, a pointer move asks nothing
    /// and changes nothing, and while the window has lost the OS's focus
    /// (`window::Event::Unfocused`) no ring is drawn though the field keeps
    /// its own, until the window has it back (`Focused`).
    #[test]
    fn the_ring_follows_the_focus_through_update() {
        let r = resolved("windows-11", ColorMode::Light);
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

    /// Where the view replaces the field with another widget, the ring asks
    /// the new one: a focused field's ring goes once the tree holds an
    /// unfocused widget of another kind in its place.
    #[test]
    fn a_replaced_field_is_asked_again() {
        let r = resolved("windows-11", ColorMode::Light);
        let mut element: Element<'_, (), Theme, Recorder> =
            crate::focus_ring(&r, 4.0, Element::new(Field { focused: true }));
        let mut tree = Tree::new(&element);
        let node = laid(&mut element, &mut tree);
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
}
