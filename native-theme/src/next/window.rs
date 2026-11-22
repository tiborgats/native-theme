//! Window chrome.

use std::sync::Arc;

use super::geometry::{CornerRadius, Corners, ShadowLayer};
use super::query::{ControlSize, GroupPosition, Orientation, StyleQuery, Surface, WidgetState};
use super::rules::{IconProp, PartProp, RuleBuilder, RuleSet};
use super::style::{IconStyle, PartStyle, TextStyle};
use super::widgets::WidgetOrigin;

/// A set of a rectangle's edges, as bits of a `u8`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct EdgeFlags(u8);

impl EdgeFlags {
    /// No edge.
    pub const NONE: Self = Self(0);
    /// The top edge.
    pub const TOP: Self = Self(1 << 0);
    /// The right edge.
    pub const RIGHT: Self = Self(1 << 1);
    /// The bottom edge.
    pub const BOTTOM: Self = Self(1 << 2);
    /// The left edge.
    pub const LEFT: Self = Self(1 << 3);
    /// Every edge.
    pub const ALL: Self = Self(0x0F);
}

bit_set!(EdgeFlags, u8);

/// A button of a window's title bar.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CaptionButtonKind {
    /// Closes the window.
    Close,
    /// Minimises the window.
    Minimize,
    /// Maximises the window.
    Maximize,
    /// Restores a maximised window.
    Restore,
}

/// The title bar's buttons at its leading and trailing ends (GNOME's
/// `button-layout`, KDE's `kwinrc`).
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct CaptionLayout {
    /// The buttons at the leading end.
    pub leading: Arc<[CaptionButtonKind]>,
    /// The buttons at the trailing end.
    pub trailing: Arc<[CaptionButtonKind]>,
}

impl CaptionLayout {
    /// A layout of these fields.
    #[must_use]
    pub fn new(leading: Arc<[CaptionButtonKind]>, trailing: Arc<[CaptionButtonKind]>) -> Self {
        Self { leading, trailing }
    }
}

/// Geometry-only window presentation state (spec §4). Active or inactive
/// comes from `ThemeContext::window_active`, which keeps window chrome,
/// selections and controls coherent without a second `active` flag.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct WindowState {
    /// The window is maximized.
    pub maximized: bool,
    /// The window is full screen.
    pub fullscreen: bool,
    /// The edges at which the window is tiled against the screen or another
    /// window.
    pub tiled: EdgeFlags,
}

/// A window frame's style, and that style's inset shadows alone, which a
/// tiled window's frame keeps.
#[derive(Clone, Debug, PartialEq)]
struct FrameStyle {
    style: PartStyle,
    inset_shadows: Option<Arc<[ShadowLayer]>>,
}

impl FrameStyle {
    fn new(style: PartStyle) -> Self {
        let inset_shadows = style.shadows.as_ref().and_then(|layers| {
            if layers.iter().all(|layer| layer.inset) {
                return Some(Arc::clone(layers));
            }
            let inset: Vec<ShadowLayer> =
                layers.iter().filter(|layer| layer.inset).copied().collect();
            (!inset.is_empty()).then(|| Arc::from(inset))
        });
        Self {
            style,
            inset_shadows,
        }
    }
}

/// A rule set of part styles keyed by a caption button's kind.
type CaptionPartRules = RuleSet<PartStyle, PartProp, Option<CaptionButtonKind>>;
/// A rule set of icon styles keyed by a caption button's kind.
type CaptionIconRules = RuleSet<IconStyle, IconProp, Option<CaptionButtonKind>>;

/// A window's chrome: its frame, title bar, title and caption buttons
/// (spec §4). The modal backdrop's dimming lives once on
/// `ColorRoles::scrim`.
///
/// It holds the frame and title bar resolved for a normal window and for a
/// maximized or full-screen one; `frame(state)` squares the corners that
/// touch the edges in `state.tiled` and drops the outer shadows by value, so
/// every query allocates nothing.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct WindowChrome {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The title bar's buttons at its two ends; `None` where no source
    /// states the platform's buttons (an empty layout is a stated "no
    /// buttons").
    pub caption_layout: Option<CaptionLayout>,
    frame: FrameStyle,
    frame_maximized: FrameStyle,
    title_bar: PartStyle,
    title_bar_maximized: PartStyle,
    title: TextStyle,
    pub(crate) caption_button: CaptionPartRules,
    pub(crate) caption_icon: CaptionIconRules,
}

impl WindowChrome {
    /// Window chrome of these styles and rule sets: `frame` and `title_bar`
    /// for a normal window, the `_maximized` ones for a maximized or
    /// full-screen window.
    #[allow(
        clippy::too_many_arguments,
        reason = "one style per part and window state of spec §4, and the caption rule sets"
    )]
    pub(crate) fn new(
        origin: WidgetOrigin,
        caption_layout: Option<CaptionLayout>,
        frame: PartStyle,
        frame_maximized: PartStyle,
        title_bar: PartStyle,
        title_bar_maximized: PartStyle,
        title: TextStyle,
        caption_button: CaptionPartRules,
        caption_icon: CaptionIconRules,
    ) -> Self {
        Self {
            origin,
            caption_layout,
            frame: FrameStyle::new(frame),
            frame_maximized: FrameStyle::new(frame_maximized),
            title_bar,
            title_bar_maximized,
            title,
            caption_button,
            caption_icon,
        }
    }

    /// Window chrome that states nothing: no caption layout, every style
    /// empty, the caption rule sets empty over a default base.
    pub(crate) fn unstated(origin: WidgetOrigin) -> Self {
        Self::new(
            origin,
            None,
            PartStyle::default(),
            PartStyle::default(),
            PartStyle::default(),
            PartStyle::default(),
            TextStyle::default(),
            RuleBuilder::new(PartStyle::default()).build(),
            RuleBuilder::new(IconStyle::default()).build(),
        )
    }

    /// Whether the chrome is the platform's own.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The window frame's style in `state`: the corners touching a tiled
    /// edge square and, where any edge is tiled, only the inset shadows.
    #[must_use]
    pub fn frame(&self, state: WindowState) -> PartStyle {
        let frame = if state.maximized || state.fullscreen {
            &self.frame_maximized
        } else {
            &self.frame
        };
        let mut style = frame.style.clone();
        if !state.tiled.is_empty() {
            style.corners = square_tiled_corners(style.corners, state.tiled);
            style.shadows = frame.inset_shadows.clone();
        }
        style
    }

    /// The title bar's style in `state`.
    #[must_use]
    pub fn title_bar(&self, state: WindowState) -> PartStyle {
        self.title_bar_of(state).clone()
    }

    /// The title bar's height in `state`, `title_bar(state).min_size[1]`;
    /// `None` where unstated.
    #[must_use]
    pub fn title_bar_height(&self, state: WindowState) -> Option<f32> {
        self.title_bar_of(state).min_size[1]
    }

    /// The title's text style, the same in every state.
    #[must_use]
    pub fn title(&self, _state: WindowState) -> TextStyle {
        self.title.clone()
    }

    /// The style of the caption button `kind` in `state`.
    #[must_use]
    pub fn caption_button(&self, kind: CaptionButtonKind, state: WidgetState) -> PartStyle {
        self.caption_button.eval(&caption_query(kind, state))
    }

    /// The style of the caption button `kind`'s icon in `state`.
    #[must_use]
    pub fn caption_icon(&self, kind: CaptionButtonKind, state: WidgetState) -> IconStyle {
        self.caption_icon.eval(&caption_query(kind, state))
    }

    fn title_bar_of(&self, state: WindowState) -> &PartStyle {
        if state.maximized || state.fullscreen {
            &self.title_bar_maximized
        } else {
            &self.title_bar
        }
    }
}

/// The query of the caption button `kind` in `state`, every axis at its
/// default.
fn caption_query(kind: CaptionButtonKind, state: WidgetState) -> StyleQuery<CaptionButtonKind> {
    StyleQuery {
        state,
        size: ControlSize::default(),
        surface: Surface::default(),
        position: GroupPosition::default(),
        orientation: Orientation::default(),
        variant: kind,
    }
}

/// `corners` with each corner that touches an edge of `tiled` square. A
/// pill's and a concentric radius have no value per corner: where any edge
/// is tiled, every corner of them is square.
fn square_tiled_corners(corners: Corners, tiled: EdgeFlags) -> Corners {
    if tiled.is_empty() {
        return corners;
    }
    let [top_left, top_right, bottom_right, bottom_left] = match corners.radius {
        CornerRadius::None => return corners,
        CornerRadius::Uniform(r) => [r; 4],
        CornerRadius::PerCorner {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        } => [top_left, top_right, bottom_right, bottom_left],
        CornerRadius::Pill | CornerRadius::Concentric { .. } => {
            return Corners {
                radius: CornerRadius::None,
                ..corners
            };
        }
    };
    let keep = |r: f32, a: EdgeFlags, b: EdgeFlags| if tiled.intersects(a | b) { 0.0 } else { r };
    Corners {
        radius: CornerRadius::PerCorner {
            top_left: keep(top_left, EdgeFlags::TOP, EdgeFlags::LEFT),
            top_right: keep(top_right, EdgeFlags::TOP, EdgeFlags::RIGHT),
            bottom_right: keep(bottom_right, EdgeFlags::BOTTOM, EdgeFlags::RIGHT),
            bottom_left: keep(bottom_left, EdgeFlags::BOTTOM, EdgeFlags::LEFT),
        },
        ..corners
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::next::color::Color;
    use crate::next::paint::Paint;
    use crate::next::rules::{RuleBuilder, RuleSelector};
    use crate::next::style::PartStated;

    fn shadow(blur: f32, inset: bool) -> ShadowLayer {
        ShadowLayer {
            blur,
            inset,
            ..ShadowLayer::default()
        }
    }

    fn chrome() -> WindowChrome {
        let frame = PartStyle {
            corners: Corners::uniform(8.0),
            shadows: Some(Arc::from([shadow(1.0, false), shadow(2.0, true)])),
            stated: PartStated::CORNERS | PartStated::SHADOWS,
            ..PartStyle::default()
        };
        let maximized = PartStyle {
            fill: Paint::Solid(Color::WHITE),
            stated: PartStated::FILL,
            ..PartStyle::default()
        };
        let bar = |height: f32| PartStyle {
            min_size: [None, Some(height)],
            ..PartStyle::default()
        };
        let close = RuleBuilder::new(PartStyle::default())
            .rule(
                RuleSelector::on(WidgetState::HOVER).matcher(Some(CaptionButtonKind::Close)),
                [PartProp::Fill(Paint::Solid(Color::BLACK))],
            )
            .build();
        WindowChrome::new(
            WidgetOrigin::Native,
            Some(CaptionLayout::new(
                Arc::from([]),
                Arc::from([CaptionButtonKind::Close]),
            )),
            frame,
            maximized,
            bar(30.0),
            bar(24.0),
            TextStyle::default(),
            close,
            RuleBuilder::new(IconStyle::default()).build(),
        )
    }

    #[test]
    fn the_frame_follows_the_window_state() {
        let chrome = chrome();
        let normal = chrome.frame(WindowState::default());
        assert_eq!(normal.corners, Corners::uniform(8.0));
        assert_eq!(normal.shadows.as_ref().map(|s| s.len()), Some(2));
        let tiled = chrome.frame(WindowState {
            tiled: EdgeFlags::LEFT,
            ..WindowState::default()
        });
        assert_eq!(
            tiled.corners.radius,
            CornerRadius::PerCorner {
                top_left: 0.0,
                top_right: 8.0,
                bottom_right: 8.0,
                bottom_left: 0.0,
            }
        );
        assert_eq!(tiled.shadows.as_deref(), Some(&[shadow(2.0, true)][..]));
        assert_eq!(tiled.stated, normal.stated);
        let maximized = WindowState {
            maximized: true,
            ..WindowState::default()
        };
        assert_eq!(chrome.frame(maximized).fill, Paint::Solid(Color::WHITE));
        assert_eq!(chrome.title_bar_height(WindowState::default()), Some(30.0));
        assert_eq!(chrome.title_bar_height(maximized), Some(24.0));
        let fullscreen = WindowState {
            fullscreen: true,
            ..WindowState::default()
        };
        assert_eq!(chrome.title_bar(fullscreen).min_size[1], Some(24.0));
    }

    #[test]
    fn a_pill_or_concentric_frame_is_square_where_an_edge_is_tiled() {
        for corners in [
            Corners::pill(),
            Corners {
                radius: CornerRadius::Concentric {
                    inset: 2.0,
                    min_radius: 1.0,
                },
                ..Corners::default()
            },
        ] {
            assert_eq!(square_tiled_corners(corners, EdgeFlags::NONE), corners);
            assert_eq!(
                square_tiled_corners(corners, EdgeFlags::TOP).radius,
                CornerRadius::None
            );
        }
    }

    #[test]
    fn caption_buttons_are_queried_by_kind() {
        let chrome = chrome();
        let hover = WidgetState::HOVER;
        assert_eq!(
            chrome.caption_button(CaptionButtonKind::Close, hover).fill,
            Paint::Solid(Color::BLACK)
        );
        assert!(
            chrome
                .caption_button(CaptionButtonKind::Minimize, hover)
                .stated
                .is_empty()
        );
        assert!(chrome.is_native());
    }

    #[test]
    fn bit_operators_and_not_masking() {
        let tb = EdgeFlags::TOP | EdgeFlags::BOTTOM;
        assert!(tb.contains(EdgeFlags::TOP));
        assert!(!tb.contains(EdgeFlags::LEFT));
        assert!(tb.intersects(EdgeFlags::TOP | EdgeFlags::LEFT));
        assert_eq!(tb & EdgeFlags::TOP, EdgeFlags::TOP);
        assert_eq!(!tb, EdgeFlags::RIGHT | EdgeFlags::LEFT);
        assert_eq!((!tb).bits(), 0b1010);
        assert_eq!(!EdgeFlags::NONE, EdgeFlags::ALL);
        assert_eq!(!EdgeFlags::ALL, EdgeFlags::NONE);
        assert_eq!(EdgeFlags::from_bits_truncate(0xFF), EdgeFlags::ALL);
        assert_eq!(
            EdgeFlags::TOP | EdgeFlags::RIGHT | EdgeFlags::BOTTOM | EdgeFlags::LEFT,
            EdgeFlags::ALL
        );
        let mut m = EdgeFlags::NONE;
        m |= EdgeFlags::LEFT;
        m |= EdgeFlags::TOP;
        m &= !EdgeFlags::TOP;
        assert_eq!(m, EdgeFlags::LEFT);
        assert!(EdgeFlags::default().is_empty());
        assert_eq!(EdgeFlags::ALL.difference(tb), !tb);
    }

    #[test]
    fn caption_layout_constructor() {
        let layout = CaptionLayout::new(
            Arc::from([CaptionButtonKind::Close]),
            Arc::from([CaptionButtonKind::Minimize, CaptionButtonKind::Maximize]),
        );
        assert_eq!(&*layout.leading, &[CaptionButtonKind::Close]);
        assert_eq!(
            &*layout.trailing,
            &[CaptionButtonKind::Minimize, CaptionButtonKind::Maximize]
        );
    }
}
