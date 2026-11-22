//! Window chrome.

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
