// Animated icon types: AnimatedIcon, TransformAnimation, Repeat
//
// These types define the data model for animated icons in the native-theme
// icon system. Frame-based animations supply pre-rendered frames with timing;
// transform-based animations describe a CSS-like transform on a single icon.

use super::icons::IconData;

/// How an animation repeats after playing through once.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Repeat {
    /// Loop forever.
    Infinite,
}

/// A CSS-like transform animation applied to a single icon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TransformAnimation {
    /// Continuous 360-degree rotation.
    Spin {
        /// Full rotation period in milliseconds.
        duration_ms: u32,
    },
}

/// An animated icon, either frame-based or transform-based.
///
/// `Frames` carries pre-rendered frames with uniform timing.
/// `Transform` carries a single icon and a description of the motion.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AnimatedIcon {
    /// A sequence of pre-rendered frames played at a fixed interval.
    Frames {
        /// The individual frames, each a complete icon image.
        frames: Vec<IconData>,
        /// Duration of each frame in milliseconds.
        frame_duration_ms: u32,
        /// How the animation repeats.
        repeat: Repeat,
    },
    /// A single icon with a continuous transform animation.
    Transform {
        /// The icon to animate.
        icon: IconData,
        /// The transform to apply.
        animation: TransformAnimation,
    },
}

impl AnimatedIcon {
    /// Return a reference to the first displayable frame.
    ///
    /// For `Frames`, returns the first element (or `None` if empty).
    /// For `Transform`, returns the underlying icon.
    #[must_use]
    pub fn first_frame(&self) -> Option<&IconData> {
        // Stub: always returns None (will be fixed in GREEN phase)
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Construction tests ===

    #[test]
    fn frames_variant_constructs() {
        let icon = AnimatedIcon::Frames {
            frames: vec![IconData::Svg(b"<svg>f1</svg>".to_vec())],
            frame_duration_ms: 83,
            repeat: Repeat::Infinite,
        };
        assert!(matches!(
            icon,
            AnimatedIcon::Frames {
                frame_duration_ms: 83,
                ..
            }
        ));
    }

    #[test]
    fn transform_variant_constructs() {
        let icon = AnimatedIcon::Transform {
            icon: IconData::Svg(b"<svg>spinner</svg>".to_vec()),
            animation: TransformAnimation::Spin { duration_ms: 1000 },
        };
        assert!(matches!(
            icon,
            AnimatedIcon::Transform {
                animation: TransformAnimation::Spin { duration_ms: 1000 },
                ..
            }
        ));
    }

    #[test]
    fn repeat_is_copy_clone_debug_eq_hash() {
        let r = Repeat::Infinite;
        let r2 = r; // Copy
        let r3 = r.clone(); // Clone
        assert_eq!(r2, r3); // PartialEq + Eq
        // Hash: just verify it compiles
        use std::hash::Hash;
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        r.hash(&mut hasher);
        let _ = format!("{r:?}"); // Debug
    }

    #[test]
    fn transform_animation_is_copy_clone_debug_eq_hash() {
        let a = TransformAnimation::Spin { duration_ms: 500 };
        let a2 = a; // Copy
        let a3 = a.clone(); // Clone
        assert_eq!(a2, a3); // PartialEq + Eq
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        a.hash(&mut hasher);
        let _ = format!("{a:?}"); // Debug
    }

    #[test]
    fn animated_icon_is_clone_debug_eq_not_copy() {
        let icon = AnimatedIcon::Frames {
            frames: vec![IconData::Svg(b"<svg/>".to_vec())],
            frame_duration_ms: 100,
            repeat: Repeat::Infinite,
        };
        let cloned = icon.clone(); // Clone
        assert_eq!(icon, cloned); // PartialEq + Eq
        let _ = format!("{icon:?}"); // Debug
        // NOT Copy -- contains Vec, so this is inherently non-Copy
    }

    // === first_frame() tests ===

    #[test]
    fn first_frame_frames_with_items() {
        let f0 = IconData::Svg(b"<svg>frame0</svg>".to_vec());
        let f1 = IconData::Svg(b"<svg>frame1</svg>".to_vec());
        let icon = AnimatedIcon::Frames {
            frames: vec![f0.clone(), f1],
            frame_duration_ms: 100,
            repeat: Repeat::Infinite,
        };
        assert_eq!(icon.first_frame(), Some(&f0));
    }

    #[test]
    fn first_frame_frames_empty() {
        let icon = AnimatedIcon::Frames {
            frames: vec![],
            frame_duration_ms: 100,
            repeat: Repeat::Infinite,
        };
        assert_eq!(icon.first_frame(), None);
    }

    #[test]
    fn first_frame_transform() {
        let data = IconData::Svg(b"<svg>spin</svg>".to_vec());
        let icon = AnimatedIcon::Transform {
            icon: data.clone(),
            animation: TransformAnimation::Spin { duration_ms: 1000 },
        };
        assert_eq!(icon.first_frame(), Some(&data));
    }
}
