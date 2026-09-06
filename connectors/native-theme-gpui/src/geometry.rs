//! Per-widget geometry for gpui-component 0.6.0 widgets (spec §9).
//!
//! Every builder is a pure function of a [`Native`] view and returns a
//! [`StyleRefinement`] the application applies with
//! `gpui_component::StyledExt::refine_style`:
//!
//! ```ignore
//! use gpui_component::StyledExt;
//! use native_theme_gpui::{ActiveNativeTheme, geometry};
//!
//! let n = cx.native_theme().and_then(|t| t.native(cx));
//! let button = Button::new("save").label("Save");
//! let button = match n { Some(n) => button.refine_style(&geometry::button(n)), None => button };
//! ```
//!
//! Each widget applies the caller's refinement after its own geometry at the
//! cited upstream line, so the values here win. Text sizes carry the
//! accessibility text-scaling factor; widths, paddings, radii and icon sizes
//! do not (spec §3.4). Every value is a `ResolvedTheme` field or one of the
//! two derivations in spec §9.4 (`scaled_text_size`, [`control_height`]).

use gpui::{FontWeight, Pixels, StyleRefinement, Styled, px};
use gpui_component::Size;
use native_theme::theme::{LayoutTheme, ResolvedBorderSpec, ResolvedFontSpec};

use crate::colors::rgba_to_hsla;
use crate::{Native, text_scale_factor};

/// `font.size × s` in pixels (spec §9.4).
fn scaled_text_size(font: &ResolvedFontSpec, n: Native<'_>) -> Pixels {
    px(font.size * text_scale_factor(n.accessibility))
}

/// CSS weight (100–900) as GPUI's `FontWeight`.
fn weight_of(font: &ResolvedFontSpec) -> FontWeight {
    FontWeight(f32::from(font.weight))
}

/// Text size and weight from a font spec.
fn with_text(r: StyleRefinement, font: &ResolvedFontSpec, n: Native<'_>) -> StyleRefinement {
    r.text_size(scaled_text_size(font, n))
        .font_weight(weight_of(font))
}

/// Control height (spec §9.4, rationale §5.3):
/// `max(theme_height, ceil(font.size × s × defaults.line_height) + 2 × padding_vertical)`.
///
/// At `s = 1` a platform's declared height already accommodates its text, so
/// this returns the theme's own value; it grows only when scaled text would be
/// clipped by gpui-component's fixed heights.
#[must_use]
pub fn control_height(
    theme_height: f32,
    font: &ResolvedFontSpec,
    border: &ResolvedBorderSpec,
    n: Native<'_>,
) -> Pixels {
    let text =
        (font.size * text_scale_factor(n.accessibility) * n.resolved.defaults.line_height).ceil();
    px(theme_height.max(text + 2.0 * border.padding_vertical))
}

/// `Button` (gpui-component 0.6.0 `src/button/button.rs:587-624` → refined at `:650`).
/// The label's text size is set on an inner element (`:658-666`), Tier U.
#[must_use]
pub fn button(n: Native<'_>) -> StyleRefinement {
    let b = &n.resolved.button;
    StyleRefinement::default()
        .h(control_height(b.min_height, &b.font, &b.border, n))
        .min_w(px(b.min_width))
        .px(px(b.border.padding_horizontal))
        .py(px(b.border.padding_vertical))
        .rounded(px(b.border.corner_radius.max(0.0)))
        .border(px(b.border.line_width))
        .border_color(rgba_to_hsla(b.border.color))
}

/// `Input` root (`src/input/input.rs:572-582` → `:587`); padding is inner, Tier U.
#[must_use]
pub fn input(n: Native<'_>) -> StyleRefinement {
    let i = &n.resolved.input;
    with_text(
        StyleRefinement::default()
            .h(control_height(i.min_height, &i.font, &i.border, n))
            .rounded(px(i.border.corner_radius.max(0.0)))
            .border(px(i.border.line_width)),
        &i.font,
        n,
    )
}

/// Application-built `MenuItem` (`src/menu/menu_item.rs:101-103` → `:109`).
#[must_use]
pub fn menu_item(n: Native<'_>) -> StyleRefinement {
    let m = &n.resolved.menu;
    with_text(
        StyleRefinement::default()
            .h(control_height(m.row_height, &m.font, &m.border, n))
            .px(px(m.border.padding_horizontal))
            .py(px(m.border.padding_vertical))
            .gap_x(px(m.icon_text_gap)),
        &m.font,
        n,
    )
}

/// `ListItem` (`src/list/list_item.rs:188-190` → `:196`).
#[must_use]
pub fn list_item(n: Native<'_>) -> StyleRefinement {
    let l = &n.resolved.list;
    with_text(
        StyleRefinement::default()
            .h(control_height(l.row_height, &l.item_font, &l.border, n))
            .px(px(l.border.padding_horizontal))
            .py(px(l.border.padding_vertical)),
        &l.item_font,
        n,
    )
}

/// Application-built `Tooltip::new` (`src/tooltip.rs:120-125` → `:126`).
#[must_use]
pub fn tooltip(n: Native<'_>) -> StyleRefinement {
    let t = &n.resolved.tooltip;
    with_text(
        StyleRefinement::default()
            .max_w(px(t.max_width))
            .px(px(t.border.padding_horizontal))
            .py(px(t.border.padding_vertical))
            .rounded(px(t.border.corner_radius.max(0.0))),
        &t.font,
        n,
    )
}

/// `Popover` (`src/popover.rs:284` → `:312`).
#[must_use]
pub fn popover(n: Native<'_>) -> StyleRefinement {
    let p = &n.resolved.popover;
    StyleRefinement::default()
        .px(px(p.border.padding_horizontal))
        .py(px(p.border.padding_vertical))
        .rounded(px(p.border.corner_radius.max(0.0)))
}

/// `StatusBar` (`src/status_bar.rs:87-89` → `:95`).
#[must_use]
pub fn status_bar(n: Native<'_>) -> StyleRefinement {
    let s = &n.resolved.status_bar;
    with_text(
        StyleRefinement::default()
            .px(px(s.border.padding_horizontal))
            .py(px(s.border.padding_vertical)),
        &s.font,
        n,
    )
}

/// `Dialog` (`src/dialog/dialog.rs:502-512, 578` → `:582`); width through
/// [`dialog_max_width`] and `Dialog::max_w`.
#[must_use]
pub fn dialog(n: Native<'_>) -> StyleRefinement {
    let d = &n.resolved.dialog;
    StyleRefinement::default()
        .px(px(d.border.padding_horizontal))
        .py(px(d.border.padding_vertical))
        .min_h(px(d.min_height))
        .max_h(px(d.max_height))
}

/// `DialogFooter` (`src/dialog/footer.rs:47` → `:51`).
#[must_use]
pub fn dialog_footer(n: Native<'_>) -> StyleRefinement {
    StyleRefinement::default().gap(px(n.resolved.dialog.button_gap))
}

/// `DialogTitle` (`src/dialog/title.rs:42-43` → `:45`).
#[must_use]
pub fn dialog_title(n: Native<'_>) -> StyleRefinement {
    with_text(StyleRefinement::default(), &n.resolved.dialog.title_font, n)
}

/// `DialogDescription` (`src/dialog/description.rs:49` → `:51`).
#[must_use]
pub fn dialog_description(n: Native<'_>) -> StyleRefinement {
    with_text(StyleRefinement::default(), &n.resolved.dialog.body_font, n)
}

/// Declarative `Table` (`src/table/table.rs:111` → `:114`); rows are inner, Tier U.
#[must_use]
pub fn table(n: Native<'_>) -> StyleRefinement {
    with_text(StyleRefinement::default(), &n.resolved.list.item_font, n)
}

/// `Progress` (`src/progress/progress.rs:128-129` → `:130`); the fill copies
/// the caller's radii (`:92-94, 137, 147`), so height and radius are exact.
#[must_use]
pub fn progress(n: Native<'_>) -> StyleRefinement {
    let p = &n.resolved.progress_bar;
    StyleRefinement::default()
        .h(px(p.track_height))
        .rounded(px(p.border.corner_radius.max(0.0)))
        .min_w(px(p.min_width))
}

/// For `GroupBox::content_style` (`src/group_box.rs:105`, applied `:156-161`).
#[must_use]
pub fn group_box_content(n: Native<'_>) -> StyleRefinement {
    let c = &n.resolved.card;
    StyleRefinement::default()
        .px(px(c.border.padding_horizontal))
        .py(px(c.border.padding_vertical))
        .rounded(px(c.border.corner_radius.max(0.0)))
        .border(px(c.border.line_width))
        .border_color(rgba_to_hsla(c.border.color))
}

/// For `AccordionItem::title_style` (`src/accordion.rs:219`, applied `:300-306`).
#[must_use]
pub fn accordion_title(n: Native<'_>) -> StyleRefinement {
    StyleRefinement::default().h(px(n.resolved.expander.header_height))
}

/// `Checkbox` (`src/checkbox.rs:256` → `:271`); the indicator is inner, Tier U.
#[must_use]
pub fn checkbox(n: Native<'_>) -> StyleRefinement {
    let c = &n.resolved.checkbox;
    with_text(StyleRefinement::default().gap(px(c.label_gap)), &c.font, n)
}

/// `Radio` (`src/radio.rs:196` → `:211`). platform-facts §2.5 defines radio
/// metrics as the checkbox's with a circular indicator, so this is a fact,
/// not a substitution (rationale D18).
#[must_use]
pub fn radio(n: Native<'_>) -> StyleRefinement {
    checkbox(n)
}

/// `Select` (`src/select.rs:479-486` → `:490`); the arrow is inner, Tier U.
#[must_use]
pub fn select(n: Native<'_>) -> StyleRefinement {
    let c = &n.resolved.combo_box;
    with_text(
        StyleRefinement::default()
            .min_h(control_height(c.min_height, &c.font, &c.border, n))
            .min_w(px(c.min_width))
            .rounded(px(c.border.corner_radius.max(0.0))),
        &c.font,
        n,
    )
}

/// `Combobox` (`src/combobox.rs:981-988` → `:992`): same sources as [`select`].
#[must_use]
pub fn combobox(n: Native<'_>) -> StyleRefinement {
    select(n)
}

/// `TitleBar` (`src/title_bar.rs:334` → `:342`); the height has no theme field.
#[must_use]
pub fn title_bar(n: Native<'_>) -> StyleRefinement {
    with_text(
        StyleRefinement::default(),
        &n.resolved.window.title_bar_font,
        n,
    )
}

// --- Size helpers (spec §9.3) -------------------------------------------------

/// `Spinner::with_size` (`src/spinner.rs:53-65` → `Icon`, `src/icon.rs:160, 189`).
#[must_use]
pub fn spinner_size(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.spinner.diameter))
}

/// `Icon::with_size` for toolbar icons (`defaults.icon_sizes.toolbar`).
#[must_use]
pub fn icon_size_toolbar(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.defaults.icon_sizes.toolbar))
}

/// `Icon::with_size` for small icons (`defaults.icon_sizes.small`).
#[must_use]
pub fn icon_size_small(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.defaults.icon_sizes.small))
}

/// `Icon::with_size` for large icons (`defaults.icon_sizes.large`).
#[must_use]
pub fn icon_size_large(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.defaults.icon_sizes.large))
}

/// `Icon::with_size` for dialog icons (`defaults.icon_sizes.dialog`).
#[must_use]
pub fn icon_size_dialog(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.defaults.icon_sizes.dialog))
}

/// `Icon::with_size` for panel icons (`defaults.icon_sizes.panel`).
#[must_use]
pub fn icon_size_panel(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.defaults.icon_sizes.panel))
}

// --- Builder helpers (spec §9.3) ----------------------------------------------

/// For `Dialog::max_w` (`src/dialog/dialog.rs:393`).
#[must_use]
pub fn dialog_max_width(n: Native<'_>) -> Pixels {
    px(n.resolved.dialog.max_width)
}

/// For `Input::h` (`src/input/input.rs:232`): the same control height [`input`] sets.
#[must_use]
pub fn input_height(n: Native<'_>) -> Pixels {
    let i = &n.resolved.input;
    control_height(i.min_height, &i.font, &i.border, n)
}

// --- Layout accessors (spec §9.5) ---------------------------------------------
// The input is `Theme::layout` on the preset path and `SystemTheme.layout` on
// the system path. No gpui-component widget reads the gpui-base spacing
// tokens, so there is no receiver to map these into; `None` means the
// platform specifies nothing (platform-facts §2.20).

/// Space between adjacent widgets.
#[must_use]
pub fn widget_gap(layout: &LayoutTheme) -> Option<Pixels> {
    layout.widget_gap.map(px)
}

/// Padding inside containers.
#[must_use]
pub fn container_margin(layout: &LayoutTheme) -> Option<Pixels> {
    layout.container_margin.map(px)
}

/// Padding inside the main window.
#[must_use]
pub fn window_margin(layout: &LayoutTheme) -> Option<Pixels> {
    layout.window_margin.map(px)
}

/// Space between major content sections.
#[must_use]
pub fn section_gap(layout: &LayoutTheme) -> Option<Pixels> {
    layout.section_gap.map(px)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::colors::rgba_to_hsla;
    use crate::{ColorMode, ResolvedTheme, Theme};
    use gpui::{AbsoluteLength, DefiniteLength, Length};
    use native_theme::AccessibilityPreferences;

    fn resolved(preset: &str, mode: ColorMode) -> ResolvedTheme {
        Theme::preset(preset)
            .expect("preset")
            .into_variant(mode)
            .expect("variant")
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .expect("resolves")
    }
    fn scaled(factor: f32) -> AccessibilityPreferences {
        AccessibilityPreferences {
            text_scaling_factor: factor,
            ..AccessibilityPreferences::default()
        }
    }
    fn len(v: f32) -> Option<Length> {
        Some(px(v).into())
    }
    fn def(v: f32) -> Option<DefiniteLength> {
        Some(px(v).into())
    }
    fn abs(v: f32) -> Option<AbsoluteLength> {
        Some(px(v).into())
    }

    const CASES: &[(&str, ColorMode)] = &[
        ("catppuccin-mocha", ColorMode::Dark),
        ("catppuccin-latte", ColorMode::Light),
    ];
    const FACTORS: &[f32] = &[1.0, 1.5];

    /// Runs `check` for both presets and both factors with a `Native` view.
    fn for_each_case(mut check: impl FnMut(&ResolvedTheme, f32, Native<'_>)) {
        for (preset, mode) in CASES {
            let r = resolved(preset, *mode);
            for &s in FACTORS {
                let prefs = scaled(s);
                check(
                    &r,
                    s,
                    Native {
                        resolved: &r,
                        accessibility: &prefs,
                    },
                );
            }
        }
    }

    fn assert_text(out: &StyleRefinement, font: &ResolvedFontSpec, s: f32) {
        assert_eq!(out.text.font_size, abs(font.size * s));
        assert_eq!(
            out.text.font_weight,
            Some(FontWeight(f32::from(font.weight)))
        );
    }

    #[test]
    fn button_refinement_matches_theme_values() {
        for_each_case(|r, _s, n| {
            let b = &r.button;
            let out = button(n);
            assert_eq!(
                out.size.height,
                Some(control_height(b.min_height, &b.font, &b.border, n).into())
            );
            assert_eq!(out.min_size.width, len(b.min_width));
            assert_eq!(out.padding.left, def(b.border.padding_horizontal));
            assert_eq!(out.padding.right, def(b.border.padding_horizontal));
            assert_eq!(out.padding.top, def(b.border.padding_vertical));
            assert_eq!(out.padding.bottom, def(b.border.padding_vertical));
            assert_eq!(
                out.corner_radii.top_left,
                abs(b.border.corner_radius.max(0.0))
            );
            assert_eq!(out.border_widths.top, abs(b.border.line_width));
            assert_eq!(out.border_color, Some(rgba_to_hsla(b.border.color)));
            assert_eq!(
                out.text.font_size, None,
                "button label size is inner (Tier U), not set here"
            );
        });
    }

    #[test]
    fn input_refinement_matches_theme_values() {
        for_each_case(|r, s, n| {
            let i = &r.input;
            let out = input(n);
            assert_eq!(
                out.size.height,
                Some(control_height(i.min_height, &i.font, &i.border, n).into())
            );
            assert_eq!(
                out.corner_radii.top_left,
                abs(i.border.corner_radius.max(0.0))
            );
            assert_eq!(out.border_widths.top, abs(i.border.line_width));
            assert_text(&out, &i.font, s);
            assert_eq!(
                input_height(n),
                control_height(i.min_height, &i.font, &i.border, n)
            );
        });
    }

    #[test]
    fn menu_and_list_items_match_theme_values() {
        for_each_case(|r, s, n| {
            let m = &r.menu;
            let out = menu_item(n);
            assert_eq!(
                out.size.height,
                Some(control_height(m.row_height, &m.font, &m.border, n).into())
            );
            assert_eq!(out.padding.left, def(m.border.padding_horizontal));
            assert_eq!(out.padding.top, def(m.border.padding_vertical));
            assert_eq!(out.gap.width, def(m.icon_text_gap));
            assert_eq!(out.gap.height, None, "gap_x sets the column gap only");
            assert_text(&out, &m.font, s);

            let l = &r.list;
            let out = list_item(n);
            assert_eq!(
                out.size.height,
                Some(control_height(l.row_height, &l.item_font, &l.border, n).into())
            );
            assert_eq!(out.padding.left, def(l.border.padding_horizontal));
            assert_eq!(out.padding.top, def(l.border.padding_vertical));
            assert_text(&out, &l.item_font, s);
        });
    }

    #[test]
    fn tooltip_popover_status_bar_match_theme_values() {
        for_each_case(|r, s, n| {
            let t = &r.tooltip;
            let out = tooltip(n);
            assert_eq!(out.max_size.width, len(t.max_width));
            assert_eq!(out.padding.left, def(t.border.padding_horizontal));
            assert_eq!(out.padding.top, def(t.border.padding_vertical));
            assert_eq!(
                out.corner_radii.top_left,
                abs(t.border.corner_radius.max(0.0))
            );
            assert_text(&out, &t.font, s);

            let p = &r.popover;
            let out = popover(n);
            assert_eq!(out.padding.left, def(p.border.padding_horizontal));
            assert_eq!(out.padding.top, def(p.border.padding_vertical));
            assert_eq!(
                out.corner_radii.top_left,
                abs(p.border.corner_radius.max(0.0))
            );

            let sb = &r.status_bar;
            let out = status_bar(n);
            assert_eq!(out.padding.left, def(sb.border.padding_horizontal));
            assert_eq!(out.padding.top, def(sb.border.padding_vertical));
            assert_text(&out, &sb.font, s);
        });
    }

    #[test]
    fn dialog_family_matches_theme_values() {
        for_each_case(|r, s, n| {
            let d = &r.dialog;
            let out = dialog(n);
            assert_eq!(out.padding.left, def(d.border.padding_horizontal));
            assert_eq!(out.padding.top, def(d.border.padding_vertical));
            assert_eq!(out.min_size.height, len(d.min_height));
            assert_eq!(out.max_size.height, len(d.max_height));
            assert_eq!(dialog_footer(n).gap.width, def(d.button_gap));
            assert_eq!(dialog_footer(n).gap.height, def(d.button_gap));
            assert_text(&dialog_title(n), &d.title_font, s);
            assert_text(&dialog_description(n), &d.body_font, s);
            assert_eq!(dialog_max_width(n), px(d.max_width));
            assert_text(&table(n), &r.list.item_font, s);
            assert_text(&title_bar(n), &r.window.title_bar_font, s);
        });
    }

    #[test]
    fn progress_group_box_accordion_match_theme_values() {
        for_each_case(|r, _s, n| {
            let p = &r.progress_bar;
            let out = progress(n);
            assert_eq!(out.size.height, len(p.track_height));
            assert_eq!(
                out.corner_radii.top_left,
                abs(p.border.corner_radius.max(0.0))
            );
            assert_eq!(out.min_size.width, len(p.min_width));

            let c = &r.card;
            let out = group_box_content(n);
            assert_eq!(out.padding.left, def(c.border.padding_horizontal));
            assert_eq!(out.padding.top, def(c.border.padding_vertical));
            assert_eq!(
                out.corner_radii.top_left,
                abs(c.border.corner_radius.max(0.0))
            );
            assert_eq!(out.border_widths.top, abs(c.border.line_width));
            assert_eq!(out.border_color, Some(rgba_to_hsla(c.border.color)));

            assert_eq!(
                accordion_title(n).size.height,
                len(r.expander.header_height)
            );
        });
    }

    #[test]
    fn checkbox_radio_select_match_theme_values() {
        for_each_case(|r, s, n| {
            let c = &r.checkbox;
            let out = checkbox(n);
            assert_eq!(out.gap.width, def(c.label_gap));
            assert_text(&out, &c.font, s);
            // platform-facts §2.5: radio metrics are the checkbox's.
            assert_eq!(radio(n).gap.width, def(c.label_gap));
            assert_text(&radio(n), &c.font, s);

            let cb = &r.combo_box;
            let out = select(n);
            assert_eq!(
                out.min_size.height,
                Some(control_height(cb.min_height, &cb.font, &cb.border, n).into())
            );
            assert_eq!(out.min_size.width, len(cb.min_width));
            assert_eq!(
                out.corner_radii.top_left,
                abs(cb.border.corner_radius.max(0.0))
            );
            assert_text(&out, &cb.font, s);
            assert_eq!(combobox(n).min_size.width, len(cb.min_width));
        });
    }

    #[test]
    fn size_helpers_use_theme_values() {
        for_each_case(|r, _s, n| {
            assert_eq!(spinner_size(n), Size::Size(px(r.spinner.diameter)));
            let is = &r.defaults.icon_sizes;
            assert_eq!(icon_size_toolbar(n), Size::Size(px(is.toolbar)));
            assert_eq!(icon_size_small(n), Size::Size(px(is.small)));
            assert_eq!(icon_size_large(n), Size::Size(px(is.large)));
            assert_eq!(icon_size_dialog(n), Size::Size(px(is.dialog)));
            assert_eq!(icon_size_panel(n), Size::Size(px(is.panel)));
        });
    }

    #[test]
    fn layout_accessors_pass_through_option() {
        let layout = Theme::preset("kde-breeze").expect("preset").layout;
        assert_eq!(widget_gap(&layout), layout.widget_gap.map(px));
        assert_eq!(container_margin(&layout), layout.container_margin.map(px));
        assert_eq!(window_margin(&layout), layout.window_margin.map(px));
        assert_eq!(section_gap(&layout), layout.section_gap.map(px));
        assert!(
            widget_gap(&layout).is_some(),
            "static presets define all four keys (§1.3)"
        );
        assert_eq!(widget_gap(&LayoutTheme::default()), None);
    }

    /// §9.4, rationale §2.23: at s = 1 the platform's own height wins.
    #[test]
    fn control_height_returns_theme_height_when_text_fits() {
        for preset in ["kde-breeze", "adwaita"] {
            let r = resolved(preset, ColorMode::Light);
            let prefs = scaled(1.0);
            let n = Native {
                resolved: &r,
                accessibility: &prefs,
            };
            let b = &r.button;
            let text =
                (b.font.size * r.defaults.line_height).ceil() + 2.0 * b.border.padding_vertical;
            assert!(
                text <= b.min_height,
                "{preset}: precondition, text {text} must fit in {}",
                b.min_height
            );
            assert_eq!(
                control_height(b.min_height, &b.font, &b.border, n),
                px(b.min_height)
            );
        }
    }

    /// §9.4: at s = 1.5 scaled text no longer fits and the height grows.
    #[test]
    fn control_height_grows_when_scaled_text_does_not_fit() {
        for preset in ["kde-breeze", "adwaita"] {
            let r = resolved(preset, ColorMode::Light);
            let prefs = scaled(1.5);
            let n = Native {
                resolved: &r,
                accessibility: &prefs,
            };
            let b = &r.button;
            let text = (b.font.size * 1.5 * r.defaults.line_height).ceil()
                + 2.0 * b.border.padding_vertical;
            assert!(
                text > b.min_height,
                "{preset}: precondition, scaled text {text} must exceed {}",
                b.min_height
            );
            assert_eq!(
                control_height(b.min_height, &b.font, &b.border, n),
                px(text)
            );
        }
    }

    #[test]
    fn unscaled_view_scales_nothing() {
        let r = resolved("catppuccin-mocha", ColorMode::Dark);
        let n = Native::unscaled(&r);
        assert_eq!(input(n).text.font_size, abs(r.input.font.size));
    }
}
