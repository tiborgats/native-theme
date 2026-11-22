//! Window chrome.

use std::sync::Arc;

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
