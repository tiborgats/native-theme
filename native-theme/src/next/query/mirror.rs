//! Right-to-left (S2.4): the query returns values physical, already
//! mirrored for the element's direction. What mirrors with the layout is
//! its own geometry: every `Sides` (`left` and `right` swapped), a
//! presentation's start or end edge, an alignment value's start and end
//! (a widget value's `Leading` and `Trailing`), and of a part's drawing only a
//! translation (`translate_x`, `Translate.dx` negated). Corners, a stroke's
//! edges, a shadow's offsets, paints and a transform's origin are physical
//! as stated and stay.

use crate::next::blocks::focus::ResolvedFocusIndicator;
use crate::next::blocks::icon::ResolvedIconStyle;
use crate::next::blocks::motion::{
    EdgeSide, ResolvedKeyframe, ResolvedLoop, ResolvedPresentationKind, ResolvedPresentationStep,
    ResolvedPresentationTransition, RevealOrigin,
};
use crate::next::blocks::part_style::{ResolvedPartStyle, ResolvedPartStylePatch};
use crate::next::blocks::transform::ResolvedTransform;
use crate::next::units::{Px, Sides};

/// A resolved value mirrored for a right-to-left layout (S2.4).
pub(crate) trait Mirror {
    /// Mirrors `self` in place.
    fn mirror(&mut self);
}

impl<T> Mirror for Sides<T> {
    fn mirror(&mut self) {
        std::mem::swap(&mut self.left, &mut self.right);
    }
}

impl<T: Mirror> Mirror for Option<T> {
    fn mirror(&mut self) {
        if let Some(value) = self {
            value.mirror();
        }
    }
}

impl<T: Mirror> Mirror for Vec<T> {
    fn mirror(&mut self) {
        for value in self {
            value.mirror();
        }
    }
}

/// A horizontal displacement negated.
fn negate(px: &mut Option<Px>) {
    if let Some(Px(x)) = px {
        *x = -*x;
    }
}

impl Mirror for ResolvedTransform {
    /// `translate_x` negated; the origin, the rotation and the scales are
    /// physical (S2.6).
    fn mirror(&mut self) {
        negate(&mut self.translate_x);
    }
}

impl Mirror for ResolvedFocusIndicator {
    /// Its offsets, a `Sides`; its corners are physical.
    fn mirror(&mut self) {
        self.offset.mirror();
    }
}

impl Mirror for ResolvedIconStyle {
    /// Its animation's keyframes.
    fn mirror(&mut self) {
        self.animation.mirror();
    }
}

impl Mirror for ResolvedLoop {
    fn mirror(&mut self) {
        self.keyframes.mirror();
    }
}

impl Mirror for ResolvedKeyframe {
    fn mirror(&mut self) {
        self.patch.mirror();
    }
}

impl Mirror for ResolvedPartStyle {
    fn mirror(&mut self) {
        self.padding.mirror();
        self.margin.mirror();
        self.transform.mirror();
        self.focus.mirror();
        self.icon.mirror();
    }
}

impl Mirror for ResolvedPartStylePatch {
    fn mirror(&mut self) {
        self.padding.mirror();
        self.margin.mirror();
        self.transform.mirror();
        self.focus.mirror();
        self.icon.mirror();
    }
}

impl Mirror for ResolvedPresentationTransition {
    fn mirror(&mut self) {
        self.steps.mirror();
    }
}

impl Mirror for ResolvedPresentationStep {
    fn mirror(&mut self) {
        self.kind.mirror();
    }
}

impl Mirror for ResolvedPresentationKind {
    /// A slide's and a clip reveal's start or end edge swapped, a
    /// translation's `dx` negated.
    fn mirror(&mut self) {
        match self {
            ResolvedPresentationKind::Slide { from, .. } => from.mirror(),
            ResolvedPresentationKind::Translate { dx, .. } => negate(dx),
            ResolvedPresentationKind::ClipReveal { from } => from.mirror(),
            ResolvedPresentationKind::Fade
            | ResolvedPresentationKind::Scale { .. }
            | ResolvedPresentationKind::Morph
            | ResolvedPresentationKind::Keyed { .. }
            | ResolvedPresentationKind::Instant => {}
        }
    }
}

impl Mirror for RevealOrigin {
    fn mirror(&mut self) {
        if let RevealOrigin::Edge(edge) = self {
            edge.mirror();
        }
    }
}

impl Mirror for EdgeSide {
    /// `Start` and `End` swapped; the top, the bottom and the edge toward
    /// the anchor stay.
    fn mirror(&mut self) {
        *self = match *self {
            EdgeSide::Start => EdgeSide::End,
            EdgeSide::End => EdgeSide::Start,
            other @ (EdgeSide::Top | EdgeSide::Bottom | EdgeSide::TowardAnchor) => other,
        };
    }
}
