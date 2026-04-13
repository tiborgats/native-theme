// Animated icon types: AnimatedIcon, TransformAnimation
//
// These types define the data model for animated icons in the native-theme
// icon system. Frame-based animations supply pre-rendered frames with timing;
// transform-based animations describe a CSS-like transform on a single icon.
//
// AnimatedIcon variant fields are private (via FramesData/TransformData wrapper
// structs) so that construction must go through validated constructors. This
// prevents invalid states like empty frame lists or zero durations.

use std::num::NonZeroU32;
use std::ops::Deref;

use serde::{Deserialize, Serialize};

use super::icons::IconData;

/// Error returned when attempting to create a [`FrameList`] from an empty vec.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyFrameListError;

impl std::fmt::Display for EmptyFrameListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("frame list must not be empty")
    }
}

impl std::error::Error for EmptyFrameListError {}

/// A non-empty list of icon frames for animation.
///
/// Construction via [`FrameList::new()`] enforces the non-empty invariant.
/// Derefs to `&[IconData]` for ergonomic slice access.
///
/// # Examples
///
/// ```
/// use native_theme::theme::{FrameList, IconData};
/// use std::borrow::Cow;
///
/// let frames = FrameList::new(vec![
///     IconData::Svg(Cow::Borrowed(b"<svg>frame1</svg>")),
/// ]);
/// assert!(frames.is_ok());
///
/// let empty = FrameList::new(vec![]);
/// assert!(empty.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FrameList(Vec<IconData>);

impl FrameList {
    /// Create a new `FrameList`, returning `Err` if `frames` is empty.
    pub fn new(frames: Vec<IconData>) -> Result<Self, EmptyFrameListError> {
        if frames.is_empty() {
            return Err(EmptyFrameListError);
        }
        Ok(Self(frames))
    }

    /// Returns the first frame (infallible -- list is guaranteed non-empty).
    #[must_use]
    pub fn first(&self) -> &IconData {
        // Invariant: FrameList is only constructible via new() which rejects empty,
        // or via custom Deserialize which also rejects empty.
        debug_assert!(!self.0.is_empty(), "FrameList invariant violated: empty list");
        &self.0[0]
    }

    /// Returns the number of frames.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Deref for FrameList {
    type Target = [IconData];
    fn deref(&self) -> &[IconData] {
        &self.0
    }
}

// Custom Deserialize that enforces the non-empty invariant at the
// deserialization boundary (T-87-01 mitigation). Do NOT derive Deserialize.
impl<'de> serde::Deserialize<'de> for FrameList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let frames = Vec::<IconData>::deserialize(deserializer)?;
        FrameList::new(frames)
            .map_err(|_| serde::de::Error::custom("frame list must not be empty"))
    }
}

/// A CSS-like transform animation applied to a single icon.
///
/// # Examples
///
/// ```
/// use native_theme::theme::TransformAnimation;
/// use std::num::NonZeroU32;
///
/// let spin = TransformAnimation::Spin {
///     duration_ms: NonZeroU32::new(1000).unwrap(),
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TransformAnimation {
    /// Continuous 360-degree rotation.
    Spin {
        /// Full rotation period in milliseconds (guaranteed non-zero).
        duration_ms: NonZeroU32,
    },
}

/// Data for a frame-based animation. Fields are private; use accessor methods.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FramesData {
    frames: FrameList,
    frame_duration_ms: NonZeroU32,
}

impl FramesData {
    /// The animation frames (guaranteed non-empty).
    #[must_use]
    pub fn frames(&self) -> &FrameList {
        &self.frames
    }

    /// Duration of each frame in milliseconds (guaranteed non-zero).
    #[must_use]
    pub fn frame_duration_ms(&self) -> NonZeroU32 {
        self.frame_duration_ms
    }
}

/// Data for a transform-based animation. Fields are private; use accessor methods.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransformData {
    icon: IconData,
    animation: TransformAnimation,
}

impl TransformData {
    /// The icon being animated.
    #[must_use]
    pub fn icon(&self) -> &IconData {
        &self.icon
    }

    /// The transform animation applied to the icon.
    #[must_use]
    pub fn animation(&self) -> &TransformAnimation {
        &self.animation
    }
}

/// An animated icon, either frame-based or transform-based.
///
/// `Frames` carries pre-rendered frames with uniform timing (loops infinitely).
/// `Transform` carries a single icon and a description of the motion.
///
/// Variant fields are private (via [`FramesData`] and [`TransformData`] wrapper
/// structs). Use the constructors [`AnimatedIcon::frames()`] and
/// [`AnimatedIcon::transform()`] to build instances, and accessor methods to
/// read fields.
///
/// # Examples
///
/// ```
/// use native_theme::theme::{AnimatedIcon, IconData, TransformAnimation};
/// use std::borrow::Cow;
/// use std::num::NonZeroU32;
///
/// // Frame-based animation (e.g., sprite sheet)
/// let frames_anim = AnimatedIcon::frames(
///     vec![
///         IconData::Svg(Cow::Borrowed(b"<svg>frame1</svg>")),
///         IconData::Svg(Cow::Borrowed(b"<svg>frame2</svg>")),
///     ],
///     NonZeroU32::new(83).unwrap(),
/// ).expect("non-empty frames");
///
/// // Transform-based animation (e.g., spinning icon)
/// let spin_anim = AnimatedIcon::transform(
///     IconData::Svg(Cow::Borrowed(b"<svg>spinner</svg>")),
///     TransformAnimation::Spin { duration_ms: NonZeroU32::new(1000).unwrap() },
/// );
///
/// // Pattern matching uses tuple variants + accessor methods
/// match &frames_anim {
///     AnimatedIcon::Frames(data) => {
///         assert_eq!(data.frames().len(), 2);
///         assert_eq!(data.frame_duration_ms().get(), 83);
///     }
///     AnimatedIcon::Transform(data) => {
///         let _icon = data.icon();
///         let _animation = data.animation();
///     }
///     _ => {}
/// }
///
/// // first_frame() returns &IconData (infallible, not Option)
/// let first = frames_anim.first_frame();
/// assert!(matches!(first, IconData::Svg(_)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AnimatedIcon {
    /// A sequence of pre-rendered frames played at a fixed interval (loops infinitely).
    Frames(FramesData),
    /// A single icon with a continuous transform animation.
    Transform(TransformData),
}

impl AnimatedIcon {
    /// Create a frame-based animation.
    ///
    /// Returns `Err(EmptyFrameListError)` if `frames` is empty.
    /// `frame_duration_ms` uses [`NonZeroU32`] so zero duration is prevented at the type level.
    ///
    /// # Examples
    ///
    /// ```
    /// use native_theme::theme::{AnimatedIcon, IconData};
    /// use std::borrow::Cow;
    /// use std::num::NonZeroU32;
    ///
    /// let anim = AnimatedIcon::frames(
    ///     vec![IconData::Svg(Cow::Borrowed(b"<svg>f1</svg>"))],
    ///     NonZeroU32::new(83).unwrap(),
    /// );
    /// assert!(anim.is_ok());
    ///
    /// // Empty frames returns Err:
    /// let empty = AnimatedIcon::frames(vec![], NonZeroU32::new(83).unwrap());
    /// assert!(empty.is_err());
    /// ```
    pub fn frames(
        frames: Vec<IconData>,
        frame_duration_ms: NonZeroU32,
    ) -> Result<Self, EmptyFrameListError> {
        Ok(AnimatedIcon::Frames(FramesData {
            frames: FrameList::new(frames)?,
            frame_duration_ms,
        }))
    }

    /// Create a transform-based animation.
    ///
    /// # Examples
    ///
    /// ```
    /// use native_theme::theme::{AnimatedIcon, IconData, TransformAnimation};
    /// use std::borrow::Cow;
    /// use std::num::NonZeroU32;
    ///
    /// let anim = AnimatedIcon::transform(
    ///     IconData::Svg(Cow::Borrowed(b"<svg>spinner</svg>")),
    ///     TransformAnimation::Spin { duration_ms: NonZeroU32::new(1000).unwrap() },
    /// );
    /// ```
    #[must_use]
    pub fn transform(icon: IconData, animation: TransformAnimation) -> Self {
        AnimatedIcon::Transform(TransformData { icon, animation })
    }

    /// Return a reference to the first displayable frame (infallible).
    ///
    /// For `Frames`, returns the first element (guaranteed to exist via [`FrameList`]).
    /// For `Transform`, returns the underlying icon.
    ///
    /// # Examples
    ///
    /// ```
    /// use native_theme::theme::{AnimatedIcon, IconData, TransformAnimation};
    /// use std::borrow::Cow;
    /// use std::num::NonZeroU32;
    ///
    /// let anim = AnimatedIcon::frames(
    ///     vec![IconData::Svg(Cow::Borrowed(b"<svg>f1</svg>"))],
    ///     NonZeroU32::new(83).unwrap(),
    /// ).unwrap();
    /// let first: &IconData = anim.first_frame();
    /// assert!(matches!(first, IconData::Svg(_)));
    ///
    /// let spin = AnimatedIcon::transform(
    ///     IconData::Svg(Cow::Borrowed(b"<svg>spinner</svg>")),
    ///     TransformAnimation::Spin { duration_ms: NonZeroU32::new(1000).unwrap() },
    /// );
    /// let first: &IconData = spin.first_frame();
    /// assert!(matches!(first, IconData::Svg(_)));
    /// ```
    #[must_use]
    pub fn first_frame(&self) -> &IconData {
        match self {
            AnimatedIcon::Frames(data) => data.frames.first(),
            AnimatedIcon::Transform(data) => &data.icon,
        }
    }

    /// Return the frame list (for Frames variant) or None (for Transform).
    #[must_use]
    pub fn frame_list(&self) -> Option<&FrameList> {
        match self {
            AnimatedIcon::Frames(data) => Some(&data.frames),
            AnimatedIcon::Transform(_) => None,
        }
    }

    /// Return the frame duration in milliseconds (for Frames variant) or None.
    #[must_use]
    pub fn frame_duration_ms(&self) -> Option<NonZeroU32> {
        match self {
            AnimatedIcon::Frames(data) => Some(data.frame_duration_ms),
            AnimatedIcon::Transform(_) => None,
        }
    }

    /// Return the icon data (for Transform variant) or None.
    #[must_use]
    pub fn icon(&self) -> Option<&IconData> {
        match self {
            AnimatedIcon::Transform(data) => Some(&data.icon),
            AnimatedIcon::Frames(_) => None,
        }
    }

    /// Return the transform animation (for Transform variant) or None.
    #[must_use]
    pub fn animation(&self) -> Option<&TransformAnimation> {
        match self {
            AnimatedIcon::Transform(data) => Some(&data.animation),
            AnimatedIcon::Frames(_) => None,
        }
    }

    /// Deprecated: Use [`AnimatedIcon::frames()`] instead.
    #[deprecated(since = "0.5.7", note = "use AnimatedIcon::frames() which returns Result")]
    #[must_use]
    pub fn new_frames(frames: Vec<IconData>, frame_duration_ms: u32) -> Option<Self> {
        let dur = NonZeroU32::new(frame_duration_ms)?;
        Self::frames(frames, dur).ok()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    fn test_icon(label: &'static str) -> IconData {
        IconData::Svg(Cow::Owned(format!("<svg>{label}</svg>").into_bytes()))
    }

    fn test_icon_borrowed() -> IconData {
        IconData::Svg(Cow::Borrowed(b"<svg>test</svg>"))
    }

    fn nz(val: u32) -> NonZeroU32 {
        NonZeroU32::new(val).unwrap()
    }

    // === FrameList tests ===

    #[test]
    fn frame_list_new_with_one_frame() {
        let fl = FrameList::new(vec![test_icon("f1")]);
        assert!(fl.is_ok());
        assert_eq!(fl.unwrap().len(), 1);
    }

    #[test]
    fn frame_list_new_with_multiple_frames() {
        let fl = FrameList::new(vec![test_icon("f1"), test_icon("f2"), test_icon("f3")]).unwrap();
        assert_eq!(fl.len(), 3);
    }

    #[test]
    fn frame_list_new_rejects_empty() {
        let fl = FrameList::new(vec![]);
        assert!(fl.is_err());
        assert_eq!(fl.unwrap_err(), EmptyFrameListError);
    }

    #[test]
    fn frame_list_first_returns_first_frame() {
        let icon = test_icon("first");
        let fl = FrameList::new(vec![icon.clone(), test_icon("second")]).unwrap();
        assert_eq!(fl.first(), &icon);
    }

    #[test]
    fn frame_list_deref_to_slice() {
        let fl = FrameList::new(vec![test_icon("a"), test_icon("b")]).unwrap();
        let slice: &[IconData] = &fl;
        assert_eq!(slice.len(), 2);
        // Can iterate via Deref
        assert_eq!(fl.iter().count(), 2);
    }

    #[test]
    fn empty_frame_list_error_display() {
        let err = EmptyFrameListError;
        assert_eq!(err.to_string(), "frame list must not be empty");
    }

    #[test]
    fn empty_frame_list_error_is_std_error() {
        let err: Box<dyn std::error::Error> = Box::new(EmptyFrameListError);
        assert_eq!(err.to_string(), "frame list must not be empty");
    }

    // === FrameList serde tests ===

    #[test]
    fn frame_list_deserialize_rejects_empty_json() {
        let json = "[]";
        let result: Result<FrameList, _> = serde_json::from_str(json);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("frame list must not be empty"),
            "expected custom error message, got: {err}"
        );
    }

    #[test]
    fn frame_list_deserialize_non_empty_json_succeeds() {
        // IconData::Svg is serialized as {"Svg": [bytes...]}, so we need valid JSON
        let fl = FrameList::new(vec![test_icon_borrowed()]).unwrap();
        let json = serde_json::to_string(&fl).unwrap();
        let deserialized: FrameList = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.len(), 1);
    }

    #[test]
    fn frame_list_round_trips_through_json() {
        let fl = FrameList::new(vec![test_icon("a"), test_icon("b")]).unwrap();
        let json = serde_json::to_string(&fl).unwrap();
        let back: FrameList = serde_json::from_str(&json).unwrap();
        assert_eq!(fl, back);
    }

    // === AnimatedIcon construction tests ===

    #[test]
    fn frames_variant_constructs() {
        let icon = AnimatedIcon::frames(vec![test_icon("f1")], nz(83)).unwrap();
        assert!(matches!(icon, AnimatedIcon::Frames(_)));
        if let AnimatedIcon::Frames(data) = &icon {
            assert_eq!(data.frames().len(), 1);
            assert_eq!(data.frame_duration_ms().get(), 83);
        }
    }

    #[test]
    fn frames_constructor_rejects_empty() {
        let result = AnimatedIcon::frames(vec![], nz(83));
        assert!(result.is_err());
    }

    #[test]
    fn transform_variant_constructs() {
        let icon = AnimatedIcon::transform(
            test_icon("spinner"),
            TransformAnimation::Spin {
                duration_ms: nz(1000),
            },
        );
        assert!(matches!(icon, AnimatedIcon::Transform(_)));
        if let AnimatedIcon::Transform(data) = &icon {
            assert_eq!(
                *data.animation(),
                TransformAnimation::Spin {
                    duration_ms: nz(1000)
                }
            );
        }
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn transform_animation_is_copy_clone_debug_eq_hash() {
        let a = TransformAnimation::Spin {
            duration_ms: nz(500),
        };
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
        let icon = AnimatedIcon::frames(vec![test_icon("f1")], nz(100)).unwrap();
        let cloned = icon.clone(); // Clone
        assert_eq!(icon, cloned); // PartialEq + Eq
        let _ = format!("{icon:?}"); // Debug
    }

    // === first_frame() tests ===

    #[test]
    fn first_frame_frames_with_items() {
        let f0 = test_icon("frame0");
        let icon = AnimatedIcon::frames(vec![f0.clone(), test_icon("frame1")], nz(100)).unwrap();
        // first_frame() returns &IconData, not Option
        assert_eq!(icon.first_frame(), &f0);
    }

    #[test]
    fn first_frame_transform() {
        let data = test_icon("spin");
        let icon = AnimatedIcon::transform(
            data.clone(),
            TransformAnimation::Spin {
                duration_ms: nz(1000),
            },
        );
        assert_eq!(icon.first_frame(), &data);
    }

    // === Accessor method tests ===

    #[test]
    fn frame_list_accessor_returns_some_for_frames() {
        let icon = AnimatedIcon::frames(vec![test_icon("f1")], nz(83)).unwrap();
        assert!(icon.frame_list().is_some());
        assert_eq!(icon.frame_list().unwrap().len(), 1);
    }

    #[test]
    fn frame_list_accessor_returns_none_for_transform() {
        let icon = AnimatedIcon::transform(
            test_icon("spin"),
            TransformAnimation::Spin {
                duration_ms: nz(1000),
            },
        );
        assert!(icon.frame_list().is_none());
    }

    #[test]
    fn frame_duration_ms_accessor() {
        let icon = AnimatedIcon::frames(vec![test_icon("f1")], nz(42)).unwrap();
        assert_eq!(icon.frame_duration_ms().unwrap().get(), 42);
    }

    #[test]
    fn icon_accessor_returns_some_for_transform() {
        let data = test_icon("spin");
        let icon = AnimatedIcon::transform(
            data.clone(),
            TransformAnimation::Spin {
                duration_ms: nz(1000),
            },
        );
        assert_eq!(icon.icon(), Some(&data));
    }

    #[test]
    fn icon_accessor_returns_none_for_frames() {
        let icon = AnimatedIcon::frames(vec![test_icon("f1")], nz(83)).unwrap();
        assert!(icon.icon().is_none());
    }

    #[test]
    fn animation_accessor_returns_some_for_transform() {
        let anim = TransformAnimation::Spin {
            duration_ms: nz(1000),
        };
        let icon = AnimatedIcon::transform(test_icon("spin"), anim);
        assert_eq!(icon.animation(), Some(&anim));
    }

    #[test]
    fn animation_accessor_returns_none_for_frames() {
        let icon = AnimatedIcon::frames(vec![test_icon("f1")], nz(83)).unwrap();
        assert!(icon.animation().is_none());
    }

    // === Deprecated new_frames() compatibility tests ===

    #[test]
    #[allow(deprecated)]
    fn new_frames_valid() {
        let anim = AnimatedIcon::new_frames(vec![test_icon("f1")], 83);
        assert!(anim.is_some());
    }

    #[test]
    #[allow(deprecated)]
    fn new_frames_rejects_empty() {
        assert!(AnimatedIcon::new_frames(vec![], 83).is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn new_frames_rejects_zero_duration() {
        let frames = vec![test_icon("f1")];
        assert!(AnimatedIcon::new_frames(frames, 0).is_none());
    }

    // === Serde round-trip tests ===

    #[test]
    fn animated_icon_frames_round_trips_json() {
        let icon = AnimatedIcon::frames(vec![test_icon("f1"), test_icon("f2")], nz(83)).unwrap();
        let json = serde_json::to_string(&icon).unwrap();
        let back: AnimatedIcon = serde_json::from_str(&json).unwrap();
        assert_eq!(icon, back);
    }

    #[test]
    fn animated_icon_transform_round_trips_json() {
        let icon = AnimatedIcon::transform(
            test_icon("spinner"),
            TransformAnimation::Spin {
                duration_ms: nz(1000),
            },
        );
        let json = serde_json::to_string(&icon).unwrap();
        let back: AnimatedIcon = serde_json::from_str(&json).unwrap();
        assert_eq!(icon, back);
    }

    #[test]
    fn transform_animation_spin_uses_non_zero_u32() {
        // NonZeroU32::new(0) returns None, so zero is impossible
        assert!(NonZeroU32::new(0).is_none());
        // Valid duration works
        let anim = TransformAnimation::Spin {
            duration_ms: nz(500),
        };
        if let TransformAnimation::Spin { duration_ms } = anim {
            assert_eq!(duration_ms.get(), 500);
        }
    }

    // === FramesData and TransformData accessor tests ===

    #[test]
    fn frames_data_accessors() {
        let icon = AnimatedIcon::frames(vec![test_icon("f1")], nz(83)).unwrap();
        if let AnimatedIcon::Frames(data) = &icon {
            assert_eq!(data.frames().len(), 1);
            assert_eq!(data.frame_duration_ms().get(), 83);
        } else {
            panic!("expected Frames variant");
        }
    }

    #[test]
    fn transform_data_accessors() {
        let icon_data = test_icon("spin");
        let anim = TransformAnimation::Spin {
            duration_ms: nz(1000),
        };
        let icon = AnimatedIcon::transform(icon_data.clone(), anim);
        if let AnimatedIcon::Transform(data) = &icon {
            assert_eq!(data.icon(), &icon_data);
            assert_eq!(*data.animation(), anim);
        } else {
            panic!("expected Transform variant");
        }
    }
}
