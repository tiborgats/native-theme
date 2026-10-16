//! Native values for the gpui-base layer, which paints scrollbars and resize
//! handles without going through gpui-component
//! (`docs/archive/todo_v0.5.8_gpui-component-0.6-spec.md` §8.2, §8.3, "the
//! spec" below).
//!
//! gpui-component's `Theme::change`, `sync_system_appearance` and `sync_base`
//! rebuild `gpui_base::Theme` with fixed scrollbar styles (gpui-component
//! `src/theme/mod.rs:403-457`), so these values are written after every such
//! rebuild by the observer `apply` installs (`crate::apply`, spec §3.3). The
//! rebuild also carries 0.7.0's plot theme (`:455`), which this module leaves
//! as upstream projects it.
//!
//! Upstream citations in this module are verified against gpui-component 0.7.3,
//! gpui-base 0.7.3 and gpui-pre 0.3.9.

use gpui::{App, Hsla, Pixels, px};
use gpui_base::{ResizableTheme, Theme as BaseTheme};
use gpui_component::scroll::ScrollbarStyles;
use native_theme::theme::ResolvedTheme;

use crate::colors::rgba_to_hsla;

/// The scrollbar values written onto gpui-base, computed from a
/// [`ResolvedTheme`].
///
/// Exists because gpui-base's style structs have private fields and derive
/// only `Clone, Default` (gpui-base `src/scrollbar.rs:640, 667, 706`),
/// so the geometry is computed into this inspectable struct, tested, and then
/// converted with one setter per field by [`scrollbar_styles`].
///
/// A geometry of its own for [`scrollbar_styles`] or [`apply_overrides`] is
/// made by [`scrollbar_geometry`] and edited field by field.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct ScrollbarGeometry {
    /// `scrollbar.groove_width`, applied to all three track states.
    pub track_width: Pixels,
    /// `scrollbar.thumb_width`, applied to all three thumb states.
    pub thumb_width: Pixels,
    /// `((groove_width − thumb_width) / 2).max(0)`: centres the thumb, which
    /// gpui-base anchors `inset` from the track's outer edge
    /// (gpui-base `src/scrollbar.rs:1200-1231`, painted at `:1505-1530`: 0.6.4
    /// moved the arithmetic into a private `ThumbGeometry`, which subtracts
    /// `2 × inset` from the thumb length and clamps the inset to half of it).
    /// [`scrollbar_styles`] therefore passes `min_thumb_length + 2 × inset` as
    /// the thumb's minimum, so the drawn thumb is at least the stated one; a
    /// proportional thumb still loses `2 × inset` along the axis, which the
    /// one inset gpui-base takes for both directions cannot avoid.
    pub thumb_inset: Pixels,
    /// `defaults.border.corner_radius`, mirroring upstream's projection
    /// (gpui-component `src/theme/mod.rs:419-439`); the theme has no
    /// scrollbar radius and platform-facts records none.
    pub thumb_radius: Pixels,
    /// `scrollbar.min_thumb_length`, the drawn thumb's minimum length along
    /// the axis (see `thumb_inset` for what gpui-base is given).
    pub min_thumb_length: Pixels,
    /// `scrollbar.track_color`, all three track states (upstream uses one colour).
    pub track: Hsla,
    /// `defaults.border.color` for the active track border, mirroring upstream
    /// (`src/theme/mod.rs:418`).
    pub track_active_border: Hsla,
    /// `scrollbar.thumb_color`.
    pub thumb: Hsla,
    /// `scrollbar.thumb_hover_color`.
    pub thumb_hover: Hsla,
    /// `scrollbar.thumb_active_color`, or `thumb_hover_color` when the theme
    /// leaves it unset (a soft option); upstream
    /// itself puts the hover colour in the active slot (`theme/mod.rs:440-449`).
    pub thumb_active: Hsla,
}

/// Compute the scrollbar values (`docs/archive/todo_v0.5.8_gpui-component-0.6-spec.md`
/// §8.2).
#[must_use]
pub fn scrollbar_geometry(resolved: &ResolvedTheme) -> ScrollbarGeometry {
    let sb = &resolved.scrollbar;
    let d = &resolved.defaults;
    ScrollbarGeometry {
        track_width: px(sb.groove_width),
        thumb_width: px(sb.thumb_width),
        thumb_inset: px(((sb.groove_width - sb.thumb_width) / 2.0).max(0.0)),
        thumb_radius: px(d.border.corner_radius),
        min_thumb_length: px(sb.min_thumb_length),
        track: rgba_to_hsla(sb.track_color),
        track_active_border: rgba_to_hsla(d.border.color),
        thumb: rgba_to_hsla(sb.thumb_color),
        thumb_hover: rgba_to_hsla(sb.thumb_hover_color),
        thumb_active: rgba_to_hsla(sb.thumb_active_color.unwrap_or(sb.thumb_hover_color)),
    }
}

/// One setter per field onto gpui-base's builders
/// (gpui-base `src/scrollbar.rs:648-697`).
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
                .min_length(thumb_min_length(g))
        })
        .thumb_hover(|s| {
            s.bg(g.thumb_hover)
                .width(g.thumb_width)
                .inset(g.thumb_inset)
                .radius(g.thumb_radius)
                .min_length(thumb_min_length(g))
        })
        .thumb_active(|s| {
            s.bg(g.thumb_active)
                .width(g.thumb_width)
                .inset(g.thumb_inset)
                .radius(g.thumb_radius)
                .min_length(thumb_min_length(g))
        })
}

/// The `min_length` gpui-base takes for the thumb: `min_thumb_length` and the
/// inset at both ends, which gpui-base takes off the clamped length
/// (`src/scrollbar.rs:1211-1216`), so the drawn thumb is at least the stated
/// minimum.
fn thumb_min_length(g: &ScrollbarGeometry) -> Pixels {
    px(f32::from(g.min_thumb_length) + 2.0 * f32::from(g.thumb_inset))
}

/// Resize-handle colours from the splitter
/// (`docs/archive/todo_v0.5.8_gpui-component-0.6-spec.md` §8.3). Upstream projects
/// `border` / `drag_border` into the same two slots
/// (gpui-component `src/theme/mod.rs:451-453`); the handle width is a
/// constant upstream (gpui-base `src/resizable/resize_handle.rs:13`), Tier U.
///
/// They reach every handle gpui-base draws with its built-in line: groups
/// built with `gpui_base::h_resizable` / `v_resizable` (re-exported as
/// `gpui_kit::base::h_resizable` / `v_resizable`), which install no renderer
/// (gpui-base `src/resizable/resize_handle.rs:309-336`) — the splitter's
/// colours at that 1 px width, not `splitter.divider_width`, the hover colour
/// only while the handle is dragged. They do not reach gpui-component 0.7.3's
/// `h_resizable` / `v_resizable`, which install gpui-component's own
/// renderer (a `border` hairline and a `muted_foreground` pill,
/// gpui-component `src/resizable.rs:27-30, 93, 104-111`), nor `Settings`'
/// divider or the dock's edges, which install it internally
/// (`src/setting/settings.rs:416`, `src/dock/dock.rs:143`).
#[must_use]
pub fn resizable_theme(resolved: &ResolvedTheme) -> ResizableTheme {
    ResizableTheme {
        handle: Some(rgba_to_hsla(resolved.splitter.divider_color)),
        active_handle: Some(rgba_to_hsla(resolved.splitter.hover_color)),
    }
}

/// Write both onto `gpui_base::Theme`, keeping the scrollbar `mode()` and
/// `motion()` upstream projected. Never panics: `gpui_base::Theme::global_mut`
/// creates a default when the global is absent (gpui-base
/// `src/theme.rs:32-37`).
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

    /// `thumb_active_color` is a soft option a theme may leave unset; the
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

    /// gpui-base clamps the thumb's length to `min_length` and then draws it
    /// `2 × inset` shorter (`src/scrollbar.rs:1211-1216`), so the drawn
    /// minimum is the stated one only when `min_length` pays for both insets.
    #[test]
    fn the_drawn_thumb_is_at_least_the_stated_minimum() {
        for preset in ["kde-breeze", "windows-11"] {
            let r = resolved(preset, ColorMode::Light);
            let sb = &r.scrollbar;
            let g = scrollbar_geometry(&r);
            assert_eq!(
                thumb_min_length(&g),
                px(sb.min_thumb_length + (sb.groove_width - sb.thumb_width)),
                "{preset}"
            );
            assert_eq!(
                thumb_min_length(&g) - g.thumb_inset * 2.0,
                px(sb.min_thumb_length),
                "{preset}"
            );
        }
    }

    /// `ScrollbarStyles` is opaque (private fields, `Clone + Default` only);
    /// the conversion is one setter per field, so the test only proves it builds.
    #[test]
    fn scrollbar_styles_builds() {
        let r = resolved("catppuccin-mocha", ColorMode::Dark);
        let _styles: ScrollbarStyles = scrollbar_styles(&scrollbar_geometry(&r));
    }
}
