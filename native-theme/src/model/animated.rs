// Animated icon types: AnimatedIcon, TransformAnimation
//
// These types define the data model for animated icons in the native-theme
// icon system. Frame-based animations supply pre-rendered frames with timing;
// transform-based animations describe a CSS-like transform on a single icon.

use serde::{Deserialize, Serialize};

use super::icons::IconData;

/// A CSS-like transform animation applied to a single icon.
///
/// # Examples
///
/// ```
/// use native_theme::TransformAnimation;
///
/// let spin = TransformAnimation::Spin { duration_ms: 1000 };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
/// `Frames` carries pre-rendered frames with uniform timing (loops infinitely).
/// `Transform` carries a single icon and a description of the motion.
///
/// # Examples
///
/// ```
/// use native_theme::{AnimatedIcon, IconData, TransformAnimation};
///
/// // Frame-based animation (e.g., sprite sheet)
/// let frames_anim = AnimatedIcon::Frames {
///     frames: vec![
///         IconData::Svg(b"<svg>frame1</svg>".to_vec()),
///         IconData::Svg(b"<svg>frame2</svg>".to_vec()),
///     ],
///     frame_duration_ms: 83,
/// };
///
/// // Transform-based animation (e.g., spinning icon)
/// let spin_anim = AnimatedIcon::Transform {
///     icon: IconData::Svg(b"<svg>spinner</svg>".to_vec()),
///     animation: TransformAnimation::Spin { duration_ms: 1000 },
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AnimatedIcon {
    /// A sequence of pre-rendered frames played at a fixed interval (loops infinitely).
    Frames {
        /// The individual frames, each a complete icon image.
        frames: Vec<IconData>,
        /// Duration of each frame in milliseconds.
        frame_duration_ms: u32,
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
    /// Create a frame-based animation.
    ///
    /// Returns `None` if `frames` is empty or `frame_duration_ms` is zero,
    /// since both would produce an invalid animation (no displayable content
    /// or a division-by-zero in playback code).
    ///
    /// # Examples
    ///
    /// ```
    /// use native_theme::{AnimatedIcon, IconData};
    ///
    /// let anim = AnimatedIcon::new_frames(
    ///     vec![IconData::Svg(b"<svg>f1</svg>".to_vec())],
    ///     83,
    /// );
    /// assert!(anim.is_some());
    ///
    /// // Empty frames or zero duration returns None:
    /// assert!(AnimatedIcon::new_frames(vec![], 83).is_none());
    /// assert!(AnimatedIcon::new_frames(vec![IconData::Svg(vec![])], 0).is_none());
    /// ```
    #[must_use]
    pub fn new_frames(frames: Vec<IconData>, frame_duration_ms: u32) -> Option<Self> {
        if frames.is_empty() || frame_duration_ms == 0 {
            return None;
        }
        Some(AnimatedIcon::Frames {
            frames,
            frame_duration_ms,
        })
    }

    /// Return a reference to the first displayable frame.
    ///
    /// For `Frames`, returns the first element (or `None` if empty).
    /// For `Transform`, returns the underlying icon.
    ///
    /// # Examples
    ///
    /// ```
    /// use native_theme::{AnimatedIcon, IconData, TransformAnimation};
    ///
    /// let frames = AnimatedIcon::Frames {
    ///     frames: vec![IconData::Svg(b"<svg>f1</svg>".to_vec())],
    ///     frame_duration_ms: 83,
    /// };
    /// assert!(frames.first_frame().is_some());
    ///
    /// let transform = AnimatedIcon::Transform {
    ///     icon: IconData::Svg(b"<svg>spinner</svg>".to_vec()),
    ///     animation: TransformAnimation::Spin { duration_ms: 1000 },
    /// };
    /// assert!(transform.first_frame().is_some());
    /// ```
    #[must_use]
    pub fn first_frame(&self) -> Option<&IconData> {
        match self {
            AnimatedIcon::Frames { frames, .. } => frames.first(),
            AnimatedIcon::Transform { icon, .. } => Some(icon),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // === Construction tests ===

    #[test]
    fn frames_variant_constructs() {
        let icon = AnimatedIcon::Frames {
            frames: vec![IconData::Svg(b"<svg>f1</svg>".to_vec())],
            frame_duration_ms: 83,
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
    #[allow(clippy::clone_on_copy)]
    fn transform_animation_is_copy_clone_debug_eq_hash() {
        let a = TransformAnimation::Spin { duration_ms: 500 };
        let a2 = a; // Copy
        let a3 = a.clone(); // Clone
        assert_eq!(a2, a3); // PartialEq + Eq
        use std::hash::Hash;
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        a.hash(&mut hasher);
        let _ = format!("{a:?}"); // Debug
    }

    #[test]
    fn animated_icon_is_clone_debug_eq_not_copy() {
        let icon = AnimatedIcon::Frames {
            frames: vec![IconData::Svg(b"<svg/>".to_vec())],
            frame_duration_ms: 100,
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
        };
        assert_eq!(icon.first_frame(), Some(&f0));
    }

    #[test]
    fn first_frame_frames_empty() {
        let icon = AnimatedIcon::Frames {
            frames: vec![],
            frame_duration_ms: 100,
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

    // === new_frames() constructor tests ===

    #[test]
    fn new_frames_valid() {
        let anim = AnimatedIcon::new_frames(vec![IconData::Svg(b"<svg>f1</svg>".to_vec())], 83);
        assert!(anim.is_some());
    }

    #[test]
    fn new_frames_rejects_empty() {
        assert!(AnimatedIcon::new_frames(vec![], 83).is_none());
    }

    #[test]
    fn new_frames_rejects_zero_duration() {
        let frames = vec![IconData::Svg(b"<svg>f1</svg>".to_vec())];
        assert!(AnimatedIcon::new_frames(frames, 0).is_none());
    }
}
