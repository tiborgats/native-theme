//! The scaffold the platform builders share (spec §6, resolution step 2):
//! the physical sides of a reading direction.

use crate::next::geometry::Edges;
use crate::next::query::Direction;

/// `edges`, stated for a left-to-right layout, on the physical sides of
/// `direction`: as given from left to right, its left and right sides
/// swapped from right to left (spec §6, resolution step 2: "Emit physical
/// left/right geometry").
pub(crate) const fn physical_edges(edges: Edges, direction: Direction) -> Edges {
    match direction {
        Direction::Ltr => edges,
        Direction::Rtl => Edges::new(edges.top, edges.left, edges.bottom, edges.right),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_to_left_swaps_the_left_and_right_sides_only() {
        let edges = Edges::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(physical_edges(edges, Direction::Ltr), edges);
        assert_eq!(
            physical_edges(edges, Direction::Rtl),
            Edges::new(1.0, 4.0, 3.0, 2.0)
        );
    }
}
