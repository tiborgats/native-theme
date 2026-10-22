//! The widget [`crate::tab_flares()`] returns: the selected tab's outward
//! bottom corners, drawn over a row of tabs after the row.

use iced_core::layout::{self, Layout};
use iced_core::renderer::Quad;
use iced_core::widget::{Operation, Tree, tree};
use iced_core::{
    Background, Border, Clipboard, Color, Element, Event, Length, Point, Rectangle, Shell, Size,
    Theme, Vector, mouse, overlay, renderer,
};
use native_theme::theme::{ResolvedTabTheme, TabIndicatorSide};

use crate::palette::to_color;

/// What [`crate::tab_flares()`] draws, read from the tab theme
/// ([`tab_flares_look`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct FlareLook {
    /// `tab.active_background`: the flares' fill and the strip's.
    pub(crate) fill: Color,
    /// `tab.border.color`: the outline continued down each curve.
    pub(crate) outline: Color,
    /// `tab.active_outward_corner_radius`, `r`: positive and finite.
    pub(crate) radius: f32,
    /// `tab.border.line_width`, `w`: 0 where it is not a non-negative,
    /// finite length.
    pub(crate) width: f32,
    /// Whether the strip over the selected tab's bottom edge is drawn: where
    /// the outline has a width and no bottom-side indicator covers that edge
    /// already.
    pub(crate) strip: bool,
}

/// One annulus of a flare, about its curve's centre, from `inner` out to
/// `outer`; a disc where `inner` is 0.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Ring {
    pub(crate) centre: Point,
    pub(crate) inner: f32,
    pub(crate) outer: f32,
    pub(crate) color: Color,
}

impl Ring {
    /// The quad that draws the ring, unsnapped (`Quad::default()` snaps
    /// under iced's default `crisp` feature, iced_core 0.14.0
    /// `src/renderer.rs:89-100`): a transparent circle whose border, drawn
    /// inside its bounds, is the annulus; a filled circle where `inner` is 0.
    pub(crate) fn quad(&self) -> (Quad, Background) {
        let bounds = Rectangle::new(
            Point::new(self.centre.x - self.outer, self.centre.y - self.outer),
            Size::new(self.outer + self.outer, self.outer + self.outer),
        );
        let (border, background) = if self.inner > 0.0 {
            (
                Border {
                    color: self.color,
                    width: self.outer - self.inner,
                    radius: self.outer.into(),
                },
                Color::TRANSPARENT,
            )
        } else {
            (
                Border {
                    radius: self.outer.into(),
                    ..Border::default()
                },
                self.color,
            )
        };
        (
            Quad {
                bounds,
                border,
                snap: false,
                ..Quad::default()
            },
            Background::Color(background),
        )
    }
}

/// One flare: its square, the fill ring and, where the outline has a width,
/// the outline ring, painted in that order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Flare {
    pub(crate) square: Rectangle,
    pub(crate) fill: Ring,
    pub(crate) outline: Option<Ring>,
}

/// [`crate::tab_flares()`]'s look: `None` unless the theme states a
/// positive, finite `tab.active_outward_corner_radius`.
pub(crate) fn tab_flares_look(t: &ResolvedTabTheme) -> Option<FlareLook> {
    let radius = t
        .active_outward_corner_radius
        .filter(|r| r.is_finite() && *r > 0.0)?;
    let w = t.border.line_width;
    let width = if w.is_finite() { w.max(0.0) } else { 0.0 };
    let bottom_line =
        crate::tab_indicator_strip(t).is_some_and(|(_, _, side)| side == TabIndicatorSide::Bottom);
    Some(FlareLook {
        fill: to_color(t.active_background),
        outline: to_color(t.border.color),
        radius,
        width,
        strip: width > 0.0 && !bottom_line,
    })
}

impl FlareLook {
    /// The strip over the bottom edge of the selected tab whose outer bounds
    /// are `tab`, between its side edges, `[x0 + w, x1 - w] x [y1 - w, y1]`,
    /// where [`strip`](Self::strip) is set: iced's `Border` takes one width
    /// for all four sides, so the edge is covered rather than left out.
    pub(crate) fn strip(&self, tab: Rectangle) -> Option<Rectangle> {
        let w = self.width;
        self.strip.then(|| {
            Rectangle::new(
                Point::new(tab.x + w, tab.y + tab.height - w),
                Size::new((tab.width - w - w).max(0.0), w),
            )
        })
    }

    /// The two flares at the feet of the selected tab whose outer bounds are
    /// `tab`, the left one first. With `r` the radius and `w` the outline's
    /// width, each lies in an `r` by `r` square at the tab's bottom, set `w`
    /// in from its outer edge, as WinUI's outline lies just outside its fill
    /// (docs/platform-facts.md §2.11): the left one's right edge at `x0 + w`,
    /// the right one's left edge at `x1 - w`. About the square's upper outer
    /// corner, a fill ring from `max(r - w, 0)` out to `r * sqrt(2) + 1`,
    /// past the square's far corner, in `tab.active_background`, and, where
    /// `w` is positive, an outline ring from `max(r - w, 0)` to `r` in
    /// `tab.border.color`, painted over it.
    pub(crate) fn flares(&self, tab: Rectangle) -> [Flare; 2] {
        let (r, w) = (self.radius, self.width);
        let top = tab.y + tab.height - r;
        let far = r * std::f32::consts::SQRT_2 + 1.0;
        // The outline's inner radius, where the fill begins too.
        let inner = (r - w).max(0.0);
        let right = tab.x + tab.width;
        // Each foot's square's left edge and its curve's centre.
        [(tab.x + w - r, tab.x + w - r), (right - w, right - w + r)].map(|(edge, centre)| {
            let centre = Point::new(centre, top);
            let ring = |outer, color| Ring {
                centre,
                inner,
                outer,
                color,
            };
            Flare {
                square: Rectangle::new(Point::new(edge, top), Size::new(r, r)),
                fill: ring(far, self.fill),
                outline: (w > 0.0).then(|| ring(r, self.outline)),
            }
        })
    }
}

/// The node `path` reaches from `layout`, a child index at each level.
fn node_at<'l>(layout: Layout<'l>, path: &[usize]) -> Option<Layout<'l>> {
    path.iter()
        .try_fold(layout, |node, &index| node.children().nth(index))
}

/// `content`, and over it, about the node at `selected` in its layout, the
/// strip and the flares of `look` ([`crate::tab_flares()`]). It hands every
/// method but `draw` to `content`, its tree state and its layout included,
/// so wrapping changes neither.
pub(crate) struct TabFlares<'a, Message, Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
    look: FlareLook,
    selected: Vec<usize>,
}

impl<'a, Message, Renderer> TabFlares<'a, Message, Renderer> {
    pub(crate) fn new(
        content: Element<'a, Message, Theme, Renderer>,
        look: FlareLook,
        selected: &[usize],
    ) -> Self {
        Self {
            content,
            look,
            selected: selected.to_vec(),
        }
    }
}

impl<Message, Renderer> iced_core::Widget<Message, Theme, Renderer>
    for TabFlares<'_, Message, Renderer>
where
    Renderer: iced_core::Renderer,
{
    fn tag(&self) -> tree::Tag {
        self.content.as_widget().tag()
    }

    fn state(&self) -> tree::State {
        self.content.as_widget().state()
    }

    fn children(&self) -> Vec<Tree> {
        self.content.as_widget().children()
    }

    fn diff(&self, tree: &mut Tree) {
        self.content.as_widget().diff(tree);
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
        self.content.as_widget_mut().layout(tree, renderer, limits)
    }

    /// `content`, then the strip in the layer `content` is drawn in, then
    /// each flare in a layer of its own, clipped to its square cut by
    /// `viewport`: iced's layers do not nest their clips (iced_graphics
    /// 0.14.0 `src/layer.rs:85-98`), so the cut keeps a flare inside a
    /// scrollable's visible area, and a flare whose square lies outside it is
    /// not drawn.
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
        self.content
            .as_widget()
            .draw(tree, renderer, theme, style, layout, cursor, viewport);
        let Some(tab) = node_at(layout, &self.selected).map(|node| node.bounds()) else {
            return;
        };
        if let Some(strip) = self.look.strip(tab) {
            renderer.fill_quad(
                Quad {
                    bounds: strip,
                    ..Quad::default()
                },
                self.look.fill,
            );
        }
        for flare in self.look.flares(tab) {
            let Some(clip) = flare.square.intersection(viewport) else {
                continue;
            };
            renderer.with_layer(clip, |renderer| {
                for ring in std::iter::once(flare.fill).chain(flare.outline) {
                    let (quad, background) = ring.quad();
                    renderer.fill_quad(quad, background);
                }
            });
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        self.content
            .as_widget_mut()
            .operate(tree, layout, renderer, operation);
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
        self.content.as_widget_mut().update(
            tree, event, layout, cursor, renderer, clipboard, shell, viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content
            .as_widget()
            .mouse_interaction(tree, layout, cursor, viewport, renderer)
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.content
            .as_widget_mut()
            .overlay(tree, layout, renderer, viewport, translation)
    }
}

impl<'a, Message, Renderer> From<TabFlares<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_core::Renderer + 'a,
{
    fn from(flares: TabFlares<'a, Message, Renderer>) -> Self {
        Element::new(flares)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use iced_widget::{Row, Space, container};
    use native_theme::theme::{ColorMode, ResolvedTheme};

    /// `windows-11` light, resolved, with `tab.active_outward_corner_radius`
    /// set to 4 on it (no bundled preset states it yet): spec §12.2's theme.
    fn curving_out() -> ResolvedTheme {
        let mut r = native_theme::theme::Theme::preset("windows-11")
            .unwrap()
            .resolve(ColorMode::Light)
            .unwrap()
            .variant;
        r.tab.active_outward_corner_radius = Some(4.0);
        r
    }

    /// What a [`Recorder`] was asked to draw.
    #[derive(Clone, Debug, PartialEq)]
    #[allow(clippy::large_enum_variant)]
    enum Drawn {
        Layer(Rectangle),
        EndLayer,
        Quad(Quad, Background),
    }

    /// A renderer that records its layers and quads, in order.
    #[derive(Default)]
    struct Recorder(Vec<Drawn>);

    impl iced_core::Renderer for Recorder {
        fn start_layer(&mut self, bounds: Rectangle) {
            self.0.push(Drawn::Layer(bounds));
        }

        fn end_layer(&mut self) {
            self.0.push(Drawn::EndLayer);
        }

        fn start_transformation(&mut self, _: iced_core::Transformation) {}

        fn end_transformation(&mut self) {}

        fn fill_quad(&mut self, quad: Quad, background: impl Into<Background>) {
            self.0.push(Drawn::Quad(quad, background.into()));
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

    /// The colour of the test's tabs.
    const TAB: Color = Color::from_rgb(0.2, 0.4, 0.6);

    /// A row of three tabs, each a container 100 by 32 filled [`TAB`], 10
    /// apart, padded 4 at both ends, wrapped in `tab_flares` with
    /// `selected`, drawn in `viewport`: what it drew, in order.
    fn drawn(resolved: &ResolvedTheme, selected: &[usize], viewport: Rectangle) -> Vec<Drawn> {
        let tabs = (0..3).map(|_| {
            container(Space::new())
                .width(100.0)
                .height(32.0)
                .style(|_| iced_widget::container::Style {
                    background: Some(TAB.into()),
                    ..iced_widget::container::Style::default()
                })
                .into()
        });
        let tabs: Row<'_, (), Theme, Recorder> = Row::with_children(tabs)
            .spacing(10.0)
            .padding(iced_core::Padding::ZERO.left(4.0).right(4.0));
        let mut element = crate::tab_flares(resolved, tabs, selected);
        let mut tree = Tree::new(&element);
        let mut recorder = Recorder::default();
        let node = element.as_widget_mut().layout(
            &mut tree,
            &recorder,
            &layout::Limits::new(Size::ZERO, Size::new(1000.0, 100.0)),
        );
        element.as_widget().draw(
            &tree,
            &mut recorder,
            &Theme::Light,
            &renderer::Style {
                text_color: Color::BLACK,
            },
            Layout::new(&node),
            mouse::Cursor::Unavailable,
            &viewport,
        );
        recorder.0
    }

    /// The whole window, for a draw nothing cuts.
    const WINDOW: Rectangle = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 1000.0,
        height: 100.0,
    };

    /// The bounds a quad of `drawn` fills, in order.
    fn tab_quads(drawn: &[Drawn]) -> Vec<Rectangle> {
        drawn
            .iter()
            .filter_map(|d| match d {
                Drawn::Quad(quad, Background::Color(c)) if *c == TAB => Some(quad.bounds),
                _ => None,
            })
            .collect()
    }

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }

    /// The look reads the theme: the fill `tab.active_background`, the
    /// outline `tab.border.color`, `r` and `w`, and the strip only where `w`
    /// is positive and no bottom-side indicator is stated; none for a radius
    /// unstated, 0, negative or not finite.
    #[test]
    fn the_look_is_the_themes() {
        let r = curving_out();
        let look = tab_flares_look(&r.tab).expect("windows-11 with r = 4 curves out");
        assert_eq!(look.fill, to_color(r.tab.active_background));
        assert_eq!(look.outline, to_color(r.tab.border.color));
        assert_eq!((look.radius, look.width, look.strip), (4.0, 1.0, true));
        for radius in [
            None,
            Some(0.0),
            Some(-1.0),
            Some(f32::NAN),
            Some(f32::INFINITY),
        ] {
            let mut t = r.tab.clone();
            t.active_outward_corner_radius = radius;
            assert_eq!(tab_flares_look(&t), None, "{radius:?}");
        }
        for (width, want) in [(0.0, 0.0), (f32::NAN, 0.0), (-1.0, 0.0), (2.0, 2.0)] {
            let mut t = r.tab.clone();
            t.border.line_width = width;
            let look = tab_flares_look(&t).expect("a radius");
            assert_eq!(look.width, want, "{width}");
            assert_eq!(look.strip, want > 0.0, "{width}");
        }
        let mut t = r.tab.clone();
        t.active_indicator_color = Some(r.tab.border.color);
        t.active_indicator_width = Some(2.0);
        t.active_indicator_side = Some(TabIndicatorSide::Bottom);
        assert!(!tab_flares_look(&t).expect("a radius").strip, "bottom");
        t.active_indicator_side = Some(TabIndicatorSide::Top);
        assert!(tab_flares_look(&t).expect("a radius").strip, "top");
    }

    /// Spec §12.2's numbers, for a tab at x 100 to 200, y 10 to 42, under
    /// `windows-11` with `r = 4` (`w = 1`): the left centre `(x0 - 3, y1 -
    /// 4)` and its square `[x0 - 3, x0 + 1] x [y1 - 4, y1]`, the right
    /// centre `(x1 + 3, y1 - 4)` and its square `[x1 - 1, x1 + 3]`; the fill
    /// from 3 to `R = 4 * sqrt(2) + 1`, the outline from 3 to 4; the strip
    /// `[x0 + 1, x1 - 1] x [y1 - 1, y1]`. Where `w` is 0 the fill runs from
    /// `r` and there is no outline; where `w` reaches `r` the fill is a disc.
    #[test]
    fn the_flares_are_the_shapes_geometry() {
        let r = curving_out();
        let look = tab_flares_look(&r.tab).expect("a radius");
        let tab = Rectangle::new(Point::new(100.0, 10.0), Size::new(100.0, 32.0));
        let far = 4.0 * std::f32::consts::SQRT_2 + 1.0;
        let [left, right] = look.flares(tab);
        assert_eq!(
            left.square,
            Rectangle::new(Point::new(97.0, 38.0), Size::new(4.0, 4.0))
        );
        assert_eq!(
            right.square,
            Rectangle::new(Point::new(199.0, 38.0), Size::new(4.0, 4.0))
        );
        for (flare, x) in [(left, 97.0), (right, 203.0)] {
            assert_eq!(flare.fill.centre, Point::new(x, 38.0));
            assert_eq!(flare.fill.inner, 3.0);
            assert!(close(flare.fill.outer, far), "{}", flare.fill.outer);
            assert_eq!(flare.fill.color, look.fill);
            let outline = flare.outline.expect("w = 1");
            assert_eq!(
                (outline.centre, outline.inner, outline.outer, outline.color),
                (Point::new(x, 38.0), 3.0, 4.0, look.outline)
            );
            let (quad, background) = flare.fill.quad();
            assert!(!quad.snap, "the rings are not snapped");
            assert!(close(quad.bounds.x, x - far) && close(quad.bounds.width, 2.0 * far));
            assert!(close(quad.border.width, far - 3.0), "{}", quad.border.width);
            assert_eq!(quad.border.color, look.fill);
            assert_eq!(background, Background::Color(Color::TRANSPARENT));
            let (quad, _) = outline.quad();
            assert_eq!(
                quad.bounds,
                Rectangle::new(Point::new(x - 4.0, 34.0), Size::new(8.0, 8.0))
            );
            assert_eq!((quad.border.width, quad.border.radius), (1.0, 4.0.into()));
        }
        assert_eq!(
            look.strip(tab),
            Some(Rectangle::new(
                Point::new(101.0, 41.0),
                Size::new(98.0, 1.0)
            ))
        );

        let mut t = r.tab.clone();
        t.border.line_width = 0.0;
        let bare = tab_flares_look(&t).expect("a radius");
        let [left, _] = bare.flares(tab);
        assert_eq!(left.fill.inner, 4.0);
        assert_eq!(left.outline, None);
        assert_eq!(left.square.x, 96.0);
        assert_eq!(bare.strip(tab), None);

        t.border.line_width = 4.0;
        let thick = tab_flares_look(&t).expect("a radius");
        let [left, _] = thick.flares(tab);
        let (quad, background) = left.fill.quad();
        assert_eq!(quad.border.width, 0.0);
        assert_eq!(background, Background::Color(thick.fill), "a disc");
    }

    /// The wrapper draws the tabs, then the strip, then the two flares,
    /// each in a layer of its square: the fill ring and the outline ring,
    /// unsnapped, after every tab's quad.
    #[test]
    fn the_flares_are_drawn_after_the_tabs() {
        let r = curving_out();
        let look = tab_flares_look(&r.tab).expect("a radius");
        let drawn = drawn(&r, &[1], WINDOW);
        let tabs: Vec<Rectangle> = (0..3)
            .map(|i| {
                Rectangle::new(
                    Point::new(4.0 + 110.0 * i as f32, 0.0),
                    Size::new(100.0, 32.0),
                )
            })
            .collect();
        assert_eq!(
            tab_quads(&drawn),
            tabs,
            "the tabs, first, at 4 in from the row's start"
        );
        let rest: Vec<Drawn> = drawn.into_iter().skip(3).collect();
        let selected = tabs.get(1).copied().expect("three tabs");
        let [left, right] = look.flares(selected);
        let ring = |ring: Ring| {
            let (quad, background) = ring.quad();
            Drawn::Quad(quad, background)
        };
        let mut want = vec![Drawn::Quad(
            Quad {
                bounds: look.strip(selected).expect("w = 1"),
                ..Quad::default()
            },
            Background::Color(look.fill),
        )];
        for flare in [left, right] {
            want.push(Drawn::Layer(flare.square));
            want.push(ring(flare.fill));
            want.push(ring(flare.outline.expect("w = 1")));
            want.push(Drawn::EndLayer);
        }
        assert_eq!(rest, want);
        let rings = rest.iter().skip(1).filter_map(|d| match d {
            Drawn::Quad(quad, _) => Some(quad),
            _ => None,
        });
        assert!(
            rings.clone().count() == 4 && rings.clone().all(|q| !q.snap),
            "no ring snaps"
        );
    }

    /// Nothing more is drawn with the radius unstated, or about a path that
    /// names no node: past the last tab, or below a tab's one child, an
    /// empty space with none.
    #[test]
    fn nothing_more_is_drawn_without_the_radius_or_a_tab() {
        let mut r = curving_out();
        for path in [&[3][..], &[0, 0, 0], &[5, 1]] {
            let drawn = drawn(&r, path, WINDOW);
            assert_eq!(drawn.len(), 3, "{path:?}: {drawn:?}");
        }
        r.tab.active_outward_corner_radius = None;
        let drawn = drawn(&r, &[1], WINDOW);
        assert_eq!(drawn.len(), 3, "{drawn:?}");
    }

    /// With a bottom-side indicator stated, the strip is not drawn: the
    /// line covers the bottom edge itself; the flares are.
    #[test]
    fn a_bottom_indicator_takes_the_strips_place() {
        let mut r = curving_out();
        r.tab.active_indicator_color = Some(r.tab.border.color);
        r.tab.active_indicator_width = Some(2.0);
        r.tab.active_indicator_side = Some(TabIndicatorSide::Bottom);
        let drawn = drawn(&r, &[0], WINDOW);
        let rest: Vec<&Drawn> = drawn.iter().skip(3).collect();
        assert!(matches!(rest.first(), Some(Drawn::Layer(_))), "{rest:?}");
        assert_eq!(rest.len(), 8, "{rest:?}");
    }

    /// A viewport that cuts a flare's square clips that flare's layer to the
    /// intersection; a viewport the selected tab's squares lie outside of
    /// draws no flare layer.
    #[test]
    fn the_viewport_cuts_the_flares() {
        let r = curving_out();
        let look = tab_flares_look(&r.tab).expect("a radius");
        let selected = Rectangle::new(Point::new(4.0, 0.0), Size::new(100.0, 32.0));
        let [left, right] = look.flares(selected);
        // From the left square's middle onwards.
        let viewport = Rectangle::new(Point::new(3.0, 0.0), Size::new(500.0, 100.0));
        let layers: Vec<Rectangle> = drawn(&r, &[0], viewport)
            .into_iter()
            .filter_map(|d| match d {
                Drawn::Layer(bounds) => Some(bounds),
                _ => None,
            })
            .collect();
        assert_eq!(
            layers,
            vec![
                Rectangle::new(Point::new(3.0, left.square.y), Size::new(2.0, 4.0)),
                right.square
            ]
        );
        let away = Rectangle::new(Point::new(300.0, 0.0), Size::new(500.0, 100.0));
        let drawn = drawn(&r, &[0], away);
        assert!(
            !drawn.iter().any(|d| matches!(d, Drawn::Layer(_))),
            "{drawn:?}"
        );
    }
}
