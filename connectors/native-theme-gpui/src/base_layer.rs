//! Native values for the gpui-base layer, which paints scrollbars and resize
//! handles without going through gpui-component (spec §8.2, §8.3).
//!
//! gpui-component's `Theme::change`, `sync_system_appearance` and `sync_base`
//! rebuild `gpui_base::Theme` with fixed scrollbar styles (gpui-component 0.6.0
//! `src/theme/mod.rs:268-300`), so these values are written after every such
//! rebuild by the observer `apply` installs (`crate::apply`, spec §3.3).

use gpui::{App, Hsla, Pixels, px};
use gpui_base::{ResizableTheme, Theme as BaseTheme};
use gpui_component::scroll::ScrollbarStyles;
use native_theme::theme::ResolvedTheme;

use crate::colors::rgba_to_hsla;

/// The scrollbar values written onto gpui-base, computed from a
/// [`ResolvedTheme`].
///
/// Exists because gpui-base's style structs have private fields and derive
/// only `Clone, Default` (gpui-base 0.6.0 `src/scrollbar.rs:589, 616, 655`),
/// so the geometry is computed into this inspectable struct, tested, and then
/// converted with one setter per field by [`scrollbar_styles`].
#[derive(Debug, Clone, PartialEq)]
pub struct ScrollbarGeometry {
    /// `scrollbar.groove_width`, applied to all three track states.
    pub track_width: Pixels,
    /// `scrollbar.thumb_width`, applied to all three thumb states.
    pub thumb_width: Pixels,
    /// `((groove_width − thumb_width) / 2).max(0)`: centres the thumb, which
    /// gpui-base anchors `inset` from the track's outer edge
    /// (gpui-base 0.6.0 `src/scrollbar.rs:1391-1400`).
    pub thumb_inset: Pixels,
    /// `defaults.border.corner_radius.max(0)`, mirroring upstream's projection
    /// (gpui-component 0.6.0 `src/theme/mod.rs:284-293`); the theme has no
    /// scrollbar radius and platform-facts records none.
    pub thumb_radius: Pixels,
    /// `scrollbar.min_thumb_length`.
    pub min_thumb_length: Pixels,
    /// `scrollbar.track_color`, all three track states (upstream uses one colour).
    pub track: Hsla,
    /// `defaults.border.color` for the active track border, mirroring upstream
    /// (`src/theme/mod.rs:283`).
    pub track_active_border: Hsla,
    /// `scrollbar.thumb_color`.
    pub thumb: Hsla,
    /// `scrollbar.thumb_hover_color`.
    pub thumb_hover: Hsla,
    /// `scrollbar.thumb_active_color`, or `thumb_hover_color` when the theme
    /// leaves it unset (a soft option, `None` in the `*-live` presets); upstream
    /// itself puts the hover colour in the active slot (`theme/mod.rs:291-295`).
    pub thumb_active: Hsla,
}

/// Compute the scrollbar values (spec §8.2).
#[must_use]
pub fn scrollbar_geometry(resolved: &ResolvedTheme) -> ScrollbarGeometry {
    let sb = &resolved.scrollbar;
    let d = &resolved.defaults;
    ScrollbarGeometry {
        track_width: px(sb.groove_width),
        thumb_width: px(sb.thumb_width),
        thumb_inset: px(((sb.groove_width - sb.thumb_width) / 2.0).max(0.0)),
        thumb_radius: px(d.border.corner_radius.max(0.0)),
        min_thumb_length: px(sb.min_thumb_length),
        track: rgba_to_hsla(sb.track_color),
        track_active_border: rgba_to_hsla(d.border.color),
        thumb: rgba_to_hsla(sb.thumb_color),
        thumb_hover: rgba_to_hsla(sb.thumb_hover_color),
        thumb_active: rgba_to_hsla(sb.thumb_active_color.unwrap_or(sb.thumb_hover_color)),
    }
}

/// One setter per field onto gpui-base's builders
/// (gpui-base 0.6.0 `src/scrollbar.rs:597-646`).
#[must_use]
pub fn scrollbar_styles(g: &ScrollbarGeometry) -> ScrollbarStyles {
    ScrollbarStyles::default()
        .track(|s| s.bg(g.track).width(g.track_width))
        .track_hover(|s| s.bg(g.track).width(g.track_width))
        .track_active(|s| {
            s.bg(g.track)
                .border_color(g.track_active_border)
                .width(g.track_width)
        })
        .thumb(|s| {
            s.bg(g.thumb)
                .width(g.thumb_width)
                .inset(g.thumb_inset)
                .radius(g.thumb_radius)
                .min_length(g.min_thumb_length)
        })
        .thumb_hover(|s| {
            s.bg(g.thumb_hover)
                .width(g.thumb_width)
                .inset(g.thumb_inset)
                .radius(g.thumb_radius)
                .min_length(g.min_thumb_length)
        })
        .thumb_active(|s| {
            s.bg(g.thumb_active)
                .width(g.thumb_width)
                .inset(g.thumb_inset)
                .radius(g.thumb_radius)
                .min_length(g.min_thumb_length)
        })
}

/// Resize-handle colours from the splitter (spec §8.3). Upstream projects
/// `border` / `drag_border` into the same two slots
/// (gpui-component 0.6.0 `src/theme/mod.rs:296-298`); the handle width is a
/// constant upstream (gpui-base `src/resizable/resize_handle.rs:12`), Tier U.
#[must_use]
pub fn resizable_theme(resolved: &ResolvedTheme) -> ResizableTheme {
    ResizableTheme {
        handle: Some(rgba_to_hsla(resolved.splitter.divider_color)),
        active_handle: Some(rgba_to_hsla(resolved.splitter.hover_color)),
    }
}

/// Write both onto `gpui_base::Theme`, keeping the scrollbar `mode()` and
/// `motion()` upstream projected. Never panics: `gpui_base::Theme::global_mut`
/// creates a default when the global is absent (gpui-base 0.6.0
/// `src/theme.rs:31-36`).
///
/// Public so an application that writes the base theme itself can restore the
/// native values; the observer `crate::apply` installs calls the same function.
pub fn apply_overrides(g: &ScrollbarGeometry, r: ResizableTheme, cx: &mut App) {
    let styles = scrollbar_styles(g);
    let base = BaseTheme::global_mut(cx);
    base.scrollbar = base.scrollbar.clone().with_styles(styles);
    base.resizable = r;
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::colors::rgba_to_hsla;
    use crate::{ColorMode, ResolvedTheme, Theme};

    fn resolved(preset: &str, mode: ColorMode) -> ResolvedTheme {
        Theme::preset(preset)
            .expect("preset")
            .into_variant(mode)
            .expect("variant")
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .expect("resolves")
    }

    /// §8.2: every receiver value is a theme field or the centring derivation.
    #[test]
    fn scrollbar_geometry_matches_theme_fields() {
        for (preset, mode) in [
            ("catppuccin-mocha", ColorMode::Dark),
            ("catppuccin-latte", ColorMode::Light),
        ] {
            let r = resolved(preset, mode);
            let sb = &r.scrollbar;
            let g = scrollbar_geometry(&r);
            assert_eq!(g.track_width, px(sb.groove_width));
            assert_eq!(g.thumb_width, px(sb.thumb_width));
            assert_eq!(
                g.thumb_inset,
                px(((sb.groove_width - sb.thumb_width) / 2.0).max(0.0))
            );
            assert_eq!(g.thumb_radius, px(r.defaults.border.corner_radius.max(0.0)));
            assert_eq!(g.min_thumb_length, px(sb.min_thumb_length));
            assert_eq!(g.track, rgba_to_hsla(sb.track_color));
            assert_eq!(g.track_active_border, rgba_to_hsla(r.defaults.border.color));
            assert_eq!(g.thumb, rgba_to_hsla(sb.thumb_color));
            assert_eq!(g.thumb_hover, rgba_to_hsla(sb.thumb_hover_color));
            assert_eq!(
                g.thumb_active,
                rgba_to_hsla(sb.thumb_active_color.unwrap_or(sb.thumb_hover_color))
            );
        }
    }

    /// `thumb_active_color` is a soft option (unset in the `*-live` presets); the
    /// active thumb then takes the hover colour, upstream's own choice for that slot.
    #[test]
    fn thumb_active_falls_back_to_hover_when_unset() {
        let mut r = resolved("catppuccin-mocha", ColorMode::Dark);
        r.scrollbar.thumb_active_color = None;
        let g = scrollbar_geometry(&r);
        assert_eq!(g.thumb_active, g.thumb_hover);
    }

    /// The inset derivation clamps at zero when the thumb fills the groove.
    #[test]
    fn thumb_inset_is_clamped_when_thumb_fills_the_groove() {
        let mut r = resolved("catppuccin-mocha", ColorMode::Dark);
        r.scrollbar.thumb_width = r.scrollbar.groove_width + 4.0;
        assert_eq!(scrollbar_geometry(&r).thumb_inset, px(0.0));
        r.scrollbar.thumb_width = r.scrollbar.groove_width;
        assert_eq!(scrollbar_geometry(&r).thumb_inset, px(0.0));
    }

    /// §8.3: upstream's two slots take the splitter colours.
    #[test]
    fn resizable_theme_uses_splitter_colours() {
        let r = resolved("catppuccin-mocha", ColorMode::Dark);
        let t = resizable_theme(&r);
        assert_eq!(t.handle, Some(rgba_to_hsla(r.splitter.divider_color)));
        assert_eq!(t.active_handle, Some(rgba_to_hsla(r.splitter.hover_color)));
    }

    /// `ScrollbarStyles` is opaque (private fields, `Clone + Default` only);
    /// the conversion is one setter per field, so the test only proves it builds.
    #[test]
    fn scrollbar_styles_builds() {
        let r = resolved("catppuccin-mocha", ColorMode::Dark);
        let _styles: ScrollbarStyles = scrollbar_styles(&scrollbar_geometry(&r));
    }
}
