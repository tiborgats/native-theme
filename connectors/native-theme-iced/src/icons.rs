//! Icon conversion helpers for iced.
//!
//! Converts [`native_theme::icons::IconData`] variants into iced-compatible handles.
//! Since iced separates raster images (`iced::widget::Image`) from SVG
//! images (`iced::widget::Svg`), this module provides separate conversion
//! functions for each variant.
//!
//! An icon by its role comes from [`SystemTheme::icon`](native_theme::SystemTheme::icon),
//! in the theme's icon set and the colour given; pass `None` to
//! [`to_svg_handle`] for it, since `icon` has coloured it already:
//!
//! ```no_run
//! use iced::widget::{image, svg};
//! use native_theme_iced::icons::{to_image_handle, to_svg_handle};
//! use native_theme_iced::{IconRole, SystemTheme, SystemThemeExt, icon_sizes};
//!
//! let sys = SystemTheme::from_system()?;
//! let loaded = sys.to_iced_theme();
//! let fg = loaded.resolved.defaults.text_color;
//! if let Some(data) = sys.icon(IconRole::ActionCopy, sys.mode, Some(fg)) {
//!     let size = icon_sizes(&loaded.resolved).toolbar;
//!     let _icon: Option<iced::Element<'_, ()>> =
//!         match (to_svg_handle(&data, None), to_image_handle(&data)) {
//!             (Some(h), _) => Some(svg(h).width(size).height(size).into()),
//!             (_, Some(h)) => Some(image(h).width(size).height(size).into()),
//!             _ => None,
//!         };
//! }
//! # Ok::<(), native_theme_iced::Error>(())
//! ```

use std::borrow::Cow;

use native_theme::icons::{
    AnimatedIcon, IconData, IconProvider, RgbaData, colorize_monochrome_svg, load_custom_icon_in,
};

/// Converted animation frames with timing metadata.
///
/// Returned by [`animated_frames_to_svg_handles`]. Contains the
/// SVG handles and the per-frame duration needed to drive playback.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct AnimatedSvgHandles {
    /// SVG handles ready for iced rendering.
    pub handles: Vec<iced_core::svg::Handle>,
    /// Duration of each frame in milliseconds.
    pub frame_duration_ms: u32,
}

/// Converts RGBA [`IconData`] to an iced [`iced_core::image::Handle`].
///
/// Returns `Some(Handle)` for [`IconData::Rgba`] data, or `None` for
/// [`IconData::Svg`]. SVG icons should use [`to_svg_handle()`] and
/// `iced::widget::Svg` instead.
#[must_use]
pub fn to_image_handle(data: &IconData) -> Option<iced_core::image::Handle> {
    match data {
        IconData::Rgba(rgba) => Some(image_handle(rgba.clone())),
        _ => None,
    }
}

/// Converts SVG [`IconData`] to an iced [`iced_core::svg::Handle`].
///
/// Returns `Some(Handle)` for [`IconData::Svg`] data, or `None` for
/// [`IconData::Rgba`]. When `color` is `Some`, colorizes the SVG for
/// monochrome icon sets (Material, Lucide) through
/// [`colorize_monochrome_svg`]. Pass `None` for multi-color system icons to
/// preserve their native palette.
#[must_use]
pub fn to_svg_handle(
    data: &IconData,
    color: Option<iced_core::Color>,
) -> Option<iced_core::svg::Handle> {
    match data {
        IconData::Svg(svg) => Some(svg_handle(Cow::Borrowed(svg), color)),
        _ => None,
    }
}

/// Load a custom RGBA icon from an [`IconProvider`] and convert to an iced image handle.
///
/// Returns `None` if the provider has no icon for the given set, or if the loaded
/// icon is SVG (use [`custom_icon_to_svg_handle()`] for SVG icons). The icon is
/// loaded through [`load_custom_icon_in`] in `color`: an SF Symbol or a Segoe glyph
/// arrives as RGBA in that colour instead of its fixed black or white.
///
/// A freedesktop icon is looked up in `icon_theme`, the system's icon theme
/// where it is `None`: pass [`Loaded::icon_theme`](crate::Loaded::icon_theme),
/// so the icon comes from the icon theme the role icons and the `Spinner`
/// beside it come from. `size` is the size the
/// system set's loader draws or looks the icon up at, clamped to
/// `1..=`[`MAX_ICON_SIZE`](native_theme::icons::MAX_ICON_SIZE); `None` keeps
/// each loader's default (see [`load_custom_icon_in`]).
#[must_use]
pub fn custom_icon_to_image_handle(
    provider: &(impl IconProvider + ?Sized),
    icon_set: native_theme::theme::IconSet,
    icon_theme: Option<&str>,
    color: Option<iced_core::Color>,
    size: Option<u32>,
) -> Option<iced_core::image::Handle> {
    into_image_handle(load_custom_icon_in(
        provider,
        icon_set,
        icon_theme,
        color.map(to_rgba),
        size,
    )?)
}

/// Load a custom SVG icon from an [`IconProvider`] and convert to an iced SVG handle.
///
/// Returns `None` if the provider has no icon for the given set, or if the loaded
/// icon is RGBA. The icon is loaded through [`load_custom_icon_in`], which colours
/// each set its own way: a Material or Lucide icon, or the provider's own SVG,
/// through the monochrome colouriser, a freedesktop icon only where its
/// foreground is left to the caller (a symbolic icon's placeholders, a
/// `currentColor` the icon sets no `color` for), so a full-colour one keeps
/// its own palette and one whose stylesheet sets its colour keeps that.
///
/// `icon_theme` and `size` are [`custom_icon_to_image_handle()`]'s: the icon
/// theme a freedesktop icon is looked up in, and the size it is looked up at,
/// at most [`MAX_ICON_SIZE`](native_theme::icons::MAX_ICON_SIZE).
#[must_use]
pub fn custom_icon_to_svg_handle(
    provider: &(impl IconProvider + ?Sized),
    icon_set: native_theme::theme::IconSet,
    icon_theme: Option<&str>,
    color: Option<iced_core::Color>,
    size: Option<u32>,
) -> Option<iced_core::svg::Handle> {
    into_svg_handle(
        load_custom_icon_in(provider, icon_set, icon_theme, color.map(to_rgba), size)?,
        None,
    )
}

/// Convert all frames of an [`AnimatedIcon::Frames`] to iced SVG handles.
///
/// Returns `Some(AnimatedSvgHandles)` when the icon is the `Frames` variant
/// and at least one frame is SVG. Returns `None` for `Transform` variants,
/// empty frame sets, or if all frames are RGBA.
///
/// Only SVG frames are included. RGBA frames are silently excluded because
/// iced's `Svg` widget cannot render raster data. The returned animation may
/// have fewer frames than the input, causing it to play faster. If all frames
/// are non-SVG, returns `None`.
///
/// When `color` is `Some`, colorizes monochrome SVG frames (Material, Lucide)
/// to match the given color. Pass `None` for multi-color system icons.
///
/// **Call this once and cache the result.** Do not call on every frame tick.
/// Index into the cached `handles` using an `iced::time::every()` subscription
/// that increments a frame counter.
///
/// Callers should check [`native_theme::detect::prefers_reduced_motion()`] and fall
/// back to [`AnimatedIcon::first_frame()`] for a static display when the user
/// has requested reduced motion.
///
/// # Examples
///
/// ```no_run
/// use native_theme_iced::icons::animated_frames_to_svg_handles;
///
/// if let Some(anim) = native_theme::icons::MaterialLoader::load_indicator() {
///     if let Some(anim_handles) = animated_frames_to_svg_handles(&anim, None) {
///         // Cache `anim_handles`, then in subscription():
///         // iced::time::every(Duration::from_millis(anim_handles.frame_duration_ms as u64))
///         //     .map(|_| Message::AnimationTick)
///         // In update(): frame_index = (frame_index + 1) % anim_handles.handles.len();
///         // In view(): Svg::new(anim_handles.handles[frame_index].clone())
///     }
/// }
/// ```
#[must_use]
pub fn animated_frames_to_svg_handles(
    anim: &AnimatedIcon,
    color: Option<iced_core::Color>,
) -> Option<AnimatedSvgHandles> {
    match anim {
        AnimatedIcon::Frames(data) => {
            let handles: Vec<_> = data
                .frames()
                .iter()
                .filter_map(|f| to_svg_handle(f, color))
                .collect();
            if handles.is_empty() {
                None
            } else {
                Some(AnimatedSvgHandles {
                    handles,
                    frame_duration_ms: data.frame_duration_ms().get(),
                })
            }
        }
        _ => None,
    }
}

/// Compute the current rotation angle for a spin animation.
///
/// Returns a [`Radians`](iced_core::Radians) value representing the current
/// rotation based on `elapsed` time and `duration_ms` (the full rotation
/// period from [`native_theme::icons::TransformAnimation::Spin`]).
///
/// The angle wraps around via modulo, so values of `elapsed` greater than
/// `duration_ms` cycle correctly.
///
/// Use the result with `Svg::rotation(Rotation::Floating(angle))` -- use
/// `Rotation::Floating` (not `Rotation::Solid`) to avoid layout jitter
/// during rotation.
///
/// Callers should check [`native_theme::detect::prefers_reduced_motion()`] and
/// skip animation when the user has requested reduced motion.
///
/// # Examples
///
/// ```no_run
/// use native_theme_iced::TransformAnimation;
/// use native_theme_iced::icons::spin_rotation_radians;
///
/// # fn spin(anim: &native_theme_iced::AnimatedIcon) {
/// let elapsed = std::time::Duration::from_millis(500);
/// if let Some(TransformAnimation::Spin { duration_ms }) = anim.animation() {
///     let angle = spin_rotation_radians(elapsed, duration_ms.get());
///     // Use with: Svg::new(handle).rotation(Rotation::Floating(angle))
/// }
/// # }
/// ```
#[must_use]
pub fn spin_rotation_radians(elapsed: std::time::Duration, duration_ms: u32) -> iced_core::Radians {
    if duration_ms == 0 {
        return iced_core::Radians(0.0);
    }
    // The remainder in integers: an f32 of the milliseconds steps by 128 ms
    // twelve days in.
    let position = elapsed
        .as_millis()
        .checked_rem(u128::from(duration_ms))
        .unwrap_or(0);
    let progress = position as f32 / duration_ms as f32;
    iced_core::Radians(progress * std::f32::consts::TAU)
}

/// Consuming version of [`to_image_handle`] — moves the [`IconData`] instead of borrowing.
#[must_use]
pub fn into_image_handle(data: IconData) -> Option<iced_core::image::Handle> {
    match data {
        IconData::Rgba(rgba) => Some(image_handle(rgba)),
        _ => None,
    }
}

/// Consuming version of [`to_svg_handle`] — moves the [`IconData`] instead of borrowing.
#[must_use]
pub fn into_svg_handle(
    data: IconData,
    color: Option<iced_core::Color>,
) -> Option<iced_core::svg::Handle> {
    match data {
        IconData::Svg(svg) => Some(svg_handle(svg, color)),
        _ => None,
    }
}

/// The image handle of `rgba`'s pixels, for [`to_image_handle`] and
/// [`into_image_handle`].
fn image_handle(rgba: RgbaData) -> iced_core::image::Handle {
    let (width, height) = (rgba.width(), rgba.height());
    iced_core::image::Handle::from_rgba(width, height, rgba.into_data())
}

/// The SVG handle of `svg`, coloured in `color` where given, for
/// [`to_svg_handle`] and [`into_svg_handle`].
fn svg_handle(svg: Cow<'_, [u8]>, color: Option<iced_core::Color>) -> iced_core::svg::Handle {
    iced_core::svg::Handle::from_memory(match color {
        Some(c) => colorize_monochrome_svg(&svg, to_rgba(c)),
        None => svg.into_owned(),
    })
}

/// An iced colour as an opaque [`native_theme::Rgba`], each channel clamped to
/// `0.0..=1.0`.
fn to_rgba(color: iced_core::Color) -> native_theme::Rgba {
    let channel = |c: f32| (c.clamp(0.0, 1.0) * 255.0).round() as u8;
    native_theme::Rgba::rgb(channel(color.r), channel(color.g), channel(color.b))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    use crate::icon_fixture::Child;
    use native_theme::icons::IconData;

    #[test]
    fn to_image_handle_with_rgba_returns_some() {
        let data = IconData::rgba(24, 24, vec![0u8; 24 * 24 * 4]).unwrap();
        assert!(to_image_handle(&data).is_some());
    }

    #[test]
    fn to_image_handle_with_svg_returns_none() {
        let data = IconData::Svg(std::borrow::Cow::Borrowed(b"<svg></svg>"));
        assert!(to_image_handle(&data).is_none());
    }

    #[test]
    fn to_svg_handle_with_svg_returns_some() {
        let data = IconData::Svg(std::borrow::Cow::Borrowed(b"<svg></svg>"));
        assert!(to_svg_handle(&data, None).is_some());
    }

    #[test]
    fn to_svg_handle_with_rgba_returns_none() {
        let data = IconData::rgba(16, 16, vec![255u8; 16 * 16 * 4]).unwrap();
        assert!(to_svg_handle(&data, None).is_none());
    }

    /// The text of an SVG handle made from bytes.
    fn svg_text(handle: &iced_core::svg::Handle) -> String {
        let iced_core::svg::Data::Bytes(bytes) = handle.data() else {
            panic!("the SVG is not in memory");
        };
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    #[test]
    fn to_svg_handle_colored_replaces_current_color() {
        let data = IconData::Svg(std::borrow::Cow::Borrowed(
            b"<svg><path stroke=\"currentColor\" fill=\"currentColor\"/></svg>",
        ));
        let color = iced_core::Color::from_rgb(1.0, 0.0, 0.0);

        let handle = to_svg_handle(&data, Some(color));
        assert!(handle.is_some());

        // Verify the colorization happened by checking the handle's SVG
        let result = svg_text(&handle.unwrap());
        assert!(result.contains("#ff0000"));
        assert!(!result.contains("currentColor"));
    }

    #[test]
    fn to_svg_handle_colored_injects_fill_for_material_style() {
        let data = IconData::Svg(std::borrow::Cow::Borrowed(
            b"<svg xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M0 0\"/></svg>",
        ));
        let color = iced_core::Color::from_rgb(0.0, 0.5, 1.0);

        let result = svg_text(&to_svg_handle(&data, Some(color)).unwrap());
        assert!(result.contains("fill=\"#0080ff\""));
    }

    #[test]
    fn to_svg_handle_colored_with_rgba_returns_none() {
        let data = IconData::rgba(16, 16, vec![0u8; 16 * 16 * 4]).unwrap();
        let color = iced_core::Color::WHITE;
        assert!(to_svg_handle(&data, Some(color)).is_none());
    }

    // --- custom_icon tests ---

    #[derive(Debug)]
    struct TestSvgProvider;

    impl native_theme::icons::IconProvider for TestSvgProvider {
        fn icon_name(&self, _set: native_theme::theme::IconSet) -> Option<&str> {
            None
        }
        fn icon_svg(
            &self,
            _set: native_theme::theme::IconSet,
        ) -> Option<std::borrow::Cow<'static, [u8]>> {
            Some(std::borrow::Cow::Borrowed(
                b"<svg xmlns='http://www.w3.org/2000/svg'><circle cx='12' cy='12' r='10'/></svg>",
            ))
        }
    }

    #[derive(Debug)]
    struct EmptyProvider;

    impl native_theme::icons::IconProvider for EmptyProvider {
        fn icon_name(&self, _set: native_theme::theme::IconSet) -> Option<&str> {
            None
        }
        fn icon_svg(
            &self,
            _set: native_theme::theme::IconSet,
        ) -> Option<std::borrow::Cow<'static, [u8]>> {
            None
        }
    }

    #[test]
    fn custom_icon_to_image_handle_with_svg_provider_returns_none() {
        // SVG data is not RGBA, so to_image_handle returns None
        let result = custom_icon_to_image_handle(
            &TestSvgProvider,
            native_theme::theme::IconSet::Material,
            None,
            None,
            None,
        );
        assert!(result.is_none());
    }

    #[test]
    fn custom_icon_to_svg_handle_with_svg_provider_returns_some() {
        let result = custom_icon_to_svg_handle(
            &TestSvgProvider,
            native_theme::theme::IconSet::Material,
            None,
            None,
            None,
        );
        assert!(result.is_some());
    }

    #[test]
    fn custom_icon_to_svg_handle_with_color_returns_some() {
        let color = iced_core::Color::from_rgb(1.0, 0.0, 0.0);
        let result = custom_icon_to_svg_handle(
            &TestSvgProvider,
            native_theme::theme::IconSet::Material,
            None,
            Some(color),
            None,
        );
        assert!(result.is_some());
    }

    #[test]
    fn custom_icon_to_image_handle_with_empty_provider_returns_none() {
        let result = custom_icon_to_image_handle(
            &EmptyProvider,
            native_theme::theme::IconSet::Material,
            None,
            None,
            None,
        );
        assert!(result.is_none());
    }

    #[test]
    fn custom_icon_to_svg_handle_with_empty_provider_returns_none() {
        let result = custom_icon_to_svg_handle(
            &EmptyProvider,
            native_theme::theme::IconSet::Material,
            None,
            None,
            None,
        );
        assert!(result.is_none());
    }

    #[test]
    fn custom_icon_helpers_accept_dyn_provider() {
        let boxed: Box<dyn native_theme::icons::IconProvider> = Box::new(TestSvgProvider);
        let result = custom_icon_to_svg_handle(
            &*boxed,
            native_theme::theme::IconSet::Material,
            None,
            None,
            None,
        );
        assert!(result.is_some());
    }

    /// Names `folder` for the freedesktop set and has no SVG of its own.
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    #[derive(Debug)]
    struct FolderProvider;

    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    impl native_theme::icons::IconProvider for FolderProvider {
        fn icon_name(&self, set: native_theme::theme::IconSet) -> Option<&str> {
            (set == native_theme::theme::IconSet::Freedesktop).then_some("folder")
        }
        fn icon_svg(
            &self,
            _set: native_theme::theme::IconSet,
        ) -> Option<std::borrow::Cow<'static, [u8]>> {
            None
        }
    }

    /// A full-colour folder in the shape of Breeze's `places/32/folder.svg`:
    /// its body `fill:currentColor`, the accent its stylesheet states, under
    /// a fill of its own.
    /// The fixture theme holds it in its one scalable directory
    /// ([`write_scalable_theme`](crate::icon_fixture::write_scalable_theme)),
    /// not in a `places/32` one.
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    const FULL_COLOUR_FOLDER: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><style>.ColorScheme-Accent { color: #3daee9 }</style><path style="fill:currentColor" class="ColorScheme-Accent" d="M2 3h12l2 2h14v24H1V10h1z"/><path style="fill:#ffffff;fill-opacity:0.2" d="M14 3l1 3h15V5H16z"/></svg>"##;

    /// A full-colour freedesktop icon loaded for a colour comes back as the
    /// theme has it: the colour is for a symbolic icon, and a second pass
    /// through the monochrome colouriser repainted the folder's body in it.
    /// The theme is a fixture in a child process ([`crate::icon_fixture`]).
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    #[test]
    fn custom_icon_to_svg_handle_leaves_a_full_colour_freedesktop_icon_as_it_is()
    -> std::io::Result<()> {
        const NAME: &str = "icons::tests::custom_icon_to_svg_handle_leaves_a_full_colour_freedesktop_icon_as_it_is";
        if crate::icon_fixture::in_child() {
            let red = iced_core::Color::from_rgb(1.0, 0.0, 0.0);
            let handle = custom_icon_to_svg_handle(
                &FolderProvider,
                native_theme::theme::IconSet::Freedesktop,
                None,
                Some(red),
                None,
            )
            .expect("the fixture theme has the folder");
            let iced_core::svg::Data::Bytes(bytes) = handle.data() else {
                panic!("the icon is not in memory");
            };
            assert_eq!(
                std::str::from_utf8(bytes).unwrap(),
                FULL_COLOUR_FOLDER,
                "the full-colour icon was recoloured"
            );
            return Ok(());
        }
        crate::icon_fixture::run(NAME, Child::kde("full-colour"), |icons| {
            crate::icon_fixture::write_scalable_theme(
                icons,
                "full-colour",
                &[("folder", FULL_COLOUR_FOLDER)],
            )
        })
    }

    /// A freedesktop icon is looked up in the icon theme given, not the
    /// system's: the child's `kdeglobals` names `Other`, which has a folder
    /// of its own, and the provider's `folder` loaded with
    /// `icon_theme = Some("Fixture")` is `Fixture`'s. The themes are fixtures
    /// in a child process ([`crate::icon_fixture`]).
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    #[test]
    fn custom_icon_to_svg_handle_looks_a_freedesktop_icon_up_in_the_icon_theme_given()
    -> std::io::Result<()> {
        const NAME: &str = "icons::tests::custom_icon_to_svg_handle_looks_a_freedesktop_icon_up_in_the_icon_theme_given";
        const FIXTURE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path d="M1 1h14v14H1z"/></svg>"#;
        const OTHER: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path d="M2 2h12v12H2z"/></svg>"#;
        if crate::icon_fixture::in_child() {
            let handle = custom_icon_to_svg_handle(
                &FolderProvider,
                native_theme::theme::IconSet::Freedesktop,
                Some("Fixture"),
                None,
                None,
            )
            .expect("the fixture theme has the folder");
            assert_eq!(svg_text(&handle), FIXTURE);
            return Ok(());
        }
        crate::icon_fixture::run(NAME, Child::kde("Other"), |icons| {
            crate::icon_fixture::write_scalable_theme(icons, "Fixture", &[("folder", FIXTURE)])?;
            crate::icon_fixture::write_scalable_theme(icons, "Other", &[("folder", OTHER)])
        })
    }

    /// The size given reaches the freedesktop lookup, clamped to
    /// `1..=MAX_ICON_SIZE` by native-theme's loader: at 64 the icon comes
    /// from the theme's 64 directory, not the 16 one nearest the loader's
    /// default 24, and at 99 999 from its 512 one, not its 1024 one. The theme is a fixture in a child process
    /// ([`crate::icon_fixture`]).
    #[cfg(all(target_os = "linux", feature = "system-icons"))]
    #[test]
    fn custom_icon_to_svg_handle_looks_a_freedesktop_icon_up_at_the_size_given()
    -> std::io::Result<()> {
        const NAME: &str =
            "icons::tests::custom_icon_to_svg_handle_looks_a_freedesktop_icon_up_at_the_size_given";
        let svg = |size: u32| {
            format!(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {size} {size}"/>"#)
        };
        if crate::icon_fixture::in_child() {
            for (asked, found) in [(64, 64), (99_999, 512)] {
                let handle = custom_icon_to_svg_handle(
                    &FolderProvider,
                    native_theme::theme::IconSet::Freedesktop,
                    Some("sized"),
                    None,
                    Some(asked),
                )
                .expect("the fixture theme has the folder");
                assert_eq!(svg_text(&handle), svg(found), "asked for {asked}");
            }
            return Ok(());
        }
        let (at_16, at_64, at_512, at_1024) = (svg(16), svg(64), svg(512), svg(1024));
        crate::icon_fixture::run(NAME, Child::kde("sized"), |icons| {
            crate::icon_fixture::write_fixed_theme(
                icons,
                "sized",
                &[
                    (16, "folder.svg", at_16.as_bytes()),
                    (64, "folder.svg", at_64.as_bytes()),
                    (512, "folder.svg", at_512.as_bytes()),
                    (1024, "folder.svg", at_1024.as_bytes()),
                ],
            )
        })
    }

    // --- animated icon tests ---

    use std::time::Duration;

    #[test]
    fn animated_frames_returns_handles() {
        let anim = AnimatedIcon::frames(
            vec![
                IconData::Svg(std::borrow::Cow::Borrowed(b"<svg></svg>")),
                IconData::Svg(std::borrow::Cow::Borrowed(b"<svg></svg>")),
            ],
            std::num::NonZeroU32::new(80).expect("test constant"),
        )
        .expect("non-empty frames");
        let result = animated_frames_to_svg_handles(&anim, None);
        assert!(result.is_some());
        let anim_handles = result.unwrap();
        assert_eq!(anim_handles.handles.len(), 2);
        assert_eq!(anim_handles.frame_duration_ms, 80);
    }

    #[test]
    fn animated_frames_transform_returns_none() {
        let anim = AnimatedIcon::transform(
            IconData::Svg(std::borrow::Cow::Borrowed(b"<svg></svg>")),
            native_theme::icons::TransformAnimation::Spin {
                duration_ms: std::num::NonZeroU32::new(1000).expect("test constant"),
            },
        );
        let result = animated_frames_to_svg_handles(&anim, None);
        assert!(result.is_none());
    }

    #[test]
    fn animated_frames_empty_returns_none() {
        // Empty FrameList is rejected at construction, so this test verifies that.
        let result = AnimatedIcon::frames(
            vec![],
            std::num::NonZeroU32::new(80).expect("test constant"),
        );
        assert!(result.is_err());
    }

    #[test]
    fn animated_frames_rgba_only_returns_none() {
        let anim = AnimatedIcon::frames(
            vec![IconData::rgba(16, 16, vec![0u8; 16 * 16 * 4]).unwrap()],
            std::num::NonZeroU32::new(80).expect("test constant"),
        )
        .expect("non-empty frames");
        let result = animated_frames_to_svg_handles(&anim, None);
        assert!(result.is_none());
    }

    #[test]
    fn spin_rotation_zero_elapsed() {
        let radians = spin_rotation_radians(Duration::ZERO, 1000);
        assert_eq!(radians, iced_core::Radians(0.0));
    }

    #[test]
    fn spin_rotation_half() {
        let radians = spin_rotation_radians(Duration::from_millis(500), 1000);
        let expected = std::f32::consts::PI;
        assert!(
            (radians.0 - expected).abs() < 0.001,
            "Expected ~{}, got {}",
            expected,
            radians.0
        );
    }

    #[test]
    fn spin_rotation_full_wraps() {
        let radians = spin_rotation_radians(Duration::from_millis(1000), 1000);
        assert!(
            radians.0.abs() < 0.001,
            "Expected ~0.0 (wrapped), got {}",
            radians.0
        );
    }

    #[test]
    fn spin_rotation_zero_duration_returns_zero() {
        let radians = spin_rotation_radians(Duration::from_millis(500), 0);
        assert_eq!(radians.0, 0.0, "zero duration should return 0.0, not NaN");
        assert!(!radians.0.is_nan(), "must not be NaN");
    }

    #[test]
    fn into_image_handle_with_rgba() {
        let data = IconData::rgba(24, 24, vec![0u8; 24 * 24 * 4]).unwrap();
        assert!(into_image_handle(data).is_some());
    }

    #[test]
    fn into_image_handle_with_svg_returns_none() {
        let data = IconData::Svg(std::borrow::Cow::Borrowed(b"<svg></svg>"));
        assert!(into_image_handle(data).is_none());
    }

    #[test]
    fn into_svg_handle_with_svg() {
        let data = IconData::Svg(std::borrow::Cow::Borrowed(b"<svg></svg>"));
        assert!(into_svg_handle(data, None).is_some());
    }

    #[test]
    fn into_svg_handle_with_rgba_returns_none() {
        let data = IconData::rgba(16, 16, vec![0u8; 16 * 16 * 4]).unwrap();
        assert!(into_svg_handle(data, None).is_none());
    }

    #[test]
    fn animated_frames_mixed_svg_rgba_filters_rgba() {
        let anim = AnimatedIcon::frames(
            vec![
                IconData::Svg(std::borrow::Cow::Borrowed(b"<svg></svg>")),
                IconData::rgba(16, 16, vec![0u8; 16 * 16 * 4]).unwrap(),
                IconData::Svg(std::borrow::Cow::Borrowed(b"<svg></svg>")),
            ],
            std::num::NonZeroU32::new(80).expect("test constant"),
        )
        .expect("non-empty frames");
        let result = animated_frames_to_svg_handles(&anim, None);
        assert!(result.is_some());
        let handles = result.unwrap();
        // RGBA frame should be filtered out, leaving 2 SVG frames
        assert_eq!(handles.handles.len(), 2);
    }

    #[test]
    fn animated_frames_with_color_colorizes_frames() {
        let anim = AnimatedIcon::frames(
            vec![IconData::Svg(std::borrow::Cow::Borrowed(
                b"<svg><path fill=\"currentColor\"/></svg>",
            ))],
            std::num::NonZeroU32::new(80).expect("test constant"),
        )
        .expect("non-empty frames");
        let color = iced_core::Color::from_rgb(1.0, 0.0, 0.0);
        let result = animated_frames_to_svg_handles(&anim, Some(color));
        assert!(result.is_some(), "should produce handles with color");
    }

    // Issue 14.3: spin_rotation_radians with very large elapsed (wraps correctly)
    #[test]
    fn spin_rotation_large_elapsed_wraps() {
        let duration_ms = 1000;
        // 1_000_000 seconds = 1M full rotations. Result should be near 0.
        let elapsed = std::time::Duration::from_secs(1_000_000);
        let result = spin_rotation_radians(elapsed, duration_ms);
        // The progress should be (elapsed_ms % duration_ms) / duration_ms.
        // 1_000_000_000ms % 1000 = 0, so rotation should be ~0.
        assert!(
            result.0.abs() < 0.01,
            "very large elapsed should wrap to near-zero, got {}",
            result.0
        );
    }

    /// Fourteen days and 50 ms in, a 1000 ms turn is a twentieth of the way
    /// round: the remainder is taken before the cast to f32, whose step at
    /// 1.2e9 ms is 128 ms.
    #[test]
    fn spin_rotation_is_exact_two_weeks_in() {
        let result = spin_rotation_radians(std::time::Duration::from_millis(1_209_600_050), 1000);
        assert!(
            (result.0 - 0.1 * std::f32::consts::PI).abs() < 1e-4,
            "expected 0.1π, got {}",
            result.0
        );
    }

    // Issue 14.3: from_preset with a single-variant preset
    #[test]
    fn from_preset_single_variant_fallback() {
        // catppuccin-mocha is dark-only; requesting light should still succeed
        // via fallback (into_variant tries the other mode when primary is empty)
        let result = crate::from_preset("catppuccin-mocha", false);
        assert!(
            result.is_ok(),
            "single-variant preset should fallback to available variant: {:?}",
            result.err()
        );
    }
}
