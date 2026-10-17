//! Widget tests (docs/archive/todo_gpui-widgets-spec.md §4): every leaf a widget
//! paints is the one the spec names, the drawn boxes measure the stated
//! sizes, and the controls behave as gpui-component's.
//!
//! A helper that draws again calls `window.refresh()` before
//! `window.draw(cx)`, as `redraw` does: on GPUI Fast (the `gpui-fast`
//! feature) a frame otherwise reuses the subtrees no view notified and keeps
//! the last frame's `debug_bounds`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gpui::{
    AnyElement, AppContext as _, Bounds, Context, Element as _, Entity, InteractiveElement as _,
    IntoElement, Modifiers, ParentElement as _, Pixels, Render, Styled as _, TestAppContext,
    VisualTestContext, Window, accesskit, div, point, px,
};
use gpui_base::slider::SliderState;
use native_theme::theme::{ColorMode, FontStyle, ResolvedTheme, TabIndicatorSide, Theme};
use native_theme::{AccessibilityPreferences, ResolutionContext};

use super::*;

/// The two presets the tests draw under: a light desktop theme and a dark
/// one whose sizes all differ from it.
const PRESETS: [(&str, ColorMode); 2] = [
    ("kde-breeze", ColorMode::Light),
    ("material-3", ColorMode::Dark),
];

fn resolved(preset: &str, mode: ColorMode) -> ResolvedTheme {
    Theme::preset(preset)
        .expect("preset loads")
        .into_variant(mode)
        .expect("variant")
        .into_resolved(&ResolutionContext::for_tests())
        .expect("preset resolves")
}

fn c(rgba: native_theme::color::Rgba) -> gpui::Hsla {
    crate::colors::rgba_to_hsla(rgba)
}

// ---------------------------------------------------------------------------
// Looks: every colour and length is the leaf the spec names
// ---------------------------------------------------------------------------

#[test]
fn a_checkbox_paints_checkbox_theme() {
    for (preset, mode) in PRESETS {
        let r = resolved(preset, mode);
        let k = &r.checkbox;
        let unchecked = c(k.unchecked_background.unwrap_or(k.background_color));
        let at = |checked, disabled| CheckboxLook::of(&r, checked, disabled).expect("finite");

        let idle = at(false, false);
        assert_eq!(idle.indicator, px(k.indicator_width), "{preset}");
        assert_eq!(idle.dot, k.radio_dot_diameter.map(px), "{preset}");
        assert_eq!(idle.radius, px(k.border.corner_radius), "{preset}");
        assert_eq!(idle.border_width, px(k.border.line_width), "{preset}");
        assert_eq!(idle.label_gap, px(k.label_gap), "{preset}");
        assert_eq!(idle.fill, unchecked, "{preset}");
        assert_eq!(
            idle.border,
            c(k.unchecked_border_color.unwrap_or(k.border.color)),
            "{preset}"
        );
        assert_eq!(
            idle.hover_fill,
            Some(
                k.hover_background
                    .map_or(unchecked, |hover| over(unchecked, c(hover)))
            ),
            "{preset}"
        );
        assert_eq!(idle.mark, c(k.indicator_color), "{preset}");
        assert_eq!(idle.label, c(k.font.color), "{preset}");

        let checked = at(true, false);
        assert_eq!(checked.fill, c(k.checked_background), "{preset}");
        assert_eq!(checked.border, c(k.border.color), "{preset}");
        assert_eq!(
            checked.hover_fill, None,
            "{preset}: no stated checked hover"
        );

        // A stated disabled fill replaces the box, its mark in the disabled
        // text colour; with none the box is its enabled self. Either way the
        // control fades by `disabled_opacity` (docs/platform-facts.md §2.1.6).
        let disabled = at(true, true);
        match k.disabled_background {
            Some(fill) => {
                assert_eq!(disabled.fill, c(fill), "{preset}");
                assert_eq!(disabled.mark, c(k.disabled_text_color), "{preset}");
            }
            None => {
                assert_eq!(disabled.fill, c(k.checked_background), "{preset}");
                assert_eq!(disabled.mark, c(k.indicator_color), "{preset}");
            }
        }
        assert_eq!(disabled.label, c(k.disabled_text_color), "{preset}");
        assert_eq!(disabled.hover_fill, None, "{preset}");
        assert_eq!(disabled.opacity, k.disabled_opacity, "{preset}");
        assert_eq!(idle.opacity, 1., "{preset}");
    }
}

/// M6-22, the twin of M5-07: a theme that states `unchecked_background` and
/// no `hover_background` leaves the hovered box at rest, where the
/// `background_color` it would otherwise lay over it is another colour.
#[test]
fn an_unset_checkbox_hover_is_the_box_at_rest() {
    let mut r = resolved("kde-breeze", ColorMode::Light);
    let fill = native_theme::color::Rgba::new(0x12, 0x34, 0x56, 0x80);
    assert_ne!(fill, r.checkbox.background_color);
    r.checkbox.unchecked_background = Some(fill);
    r.checkbox.hover_background = None;
    for look in [
        CheckboxLook::of(&r, false, false).expect("finite"),
        CheckboxLook::radio(&r, false, false).expect("finite"),
    ] {
        assert_eq!(look.fill, c(fill));
        assert_eq!(look.hover_fill, Some(look.fill));
    }
}

/// The platforms each dim by one mechanism (docs/platform-facts.md §2.1.6):
/// kde-breeze by its disabled colours at an opacity of 1, adwaita by an
/// opacity of 0.5 over its enabled colours.
#[test]
fn a_disabled_control_is_faded_by_its_disabled_opacity() {
    let kde = CheckboxLook::of(&resolved("kde-breeze", ColorMode::Light), true, true).unwrap();
    assert_eq!(kde.opacity, 1.);
    let adw_theme = resolved("adwaita", ColorMode::Light);
    let adw = CheckboxLook::of(&adw_theme, true, true).unwrap();
    assert_eq!(adw.opacity, 0.5);
    assert_eq!(adw.fill, c(adw_theme.checkbox.checked_background));
    assert_eq!(SwitchLook::of(&adw_theme, true, true).unwrap().opacity, 0.5);
    assert_eq!(SliderLook::of(&adw_theme, true).unwrap().opacity, 0.5);
    assert_eq!(SliderLook::of(&adw_theme, false).unwrap().opacity, 1.);
}

#[test]
fn the_checkbox_look_is_the_presets_values() {
    // The values the audit measured against (SC AUDIT-gpui, the dumps
    // kbl.flat / md.flat): the mapping reaches the stated numbers.
    let kbl = CheckboxLook::of(&resolved("kde-breeze", ColorMode::Light), false, false).unwrap();
    assert_eq!(kbl.indicator, px(20.));
    assert_eq!(
        kbl.mark_stroke,
        Some(px(2.)),
        "Breeze's check pen (docs/platform-facts.md:1242)"
    );
    let md = CheckboxLook::of(&resolved("material-3", ColorMode::Dark), false, false).unwrap();
    assert_eq!(md.indicator, px(18.));
    // The radio is `checkbox.radio_indicator_width` across where stated
    // (material-3's 20, docs/platform-facts.md:1243), the checkbox's elsewhere.
    let radio = |preset: &str| {
        CheckboxLook::radio(&resolved(preset, ColorMode::Dark), false, false).map(|l| l.indicator)
    };
    assert_eq!(radio("material-3"), Some(px(20.)));
    assert_eq!(radio("kde-breeze"), Some(kbl.indicator));
    assert_eq!(
        md.mark_stroke,
        Some(px(2.)),
        "material-web's $_mark-stroke (docs/platform-facts.md:1242)"
    );
}

/// The check mark fills the indicator inside `checkbox.border.padding`
/// where the theme states it (adwaita's 3), and inside its border where it
/// does not (kde-breeze).
#[test]
fn the_check_mark_fills_the_indicator_inside_its_padding() {
    let adw = CheckboxLook::of(&resolved("adwaita", ColorMode::Light), true, false);
    assert_eq!(
        adw.map(|l| (l.mark_size, l.mark_padded)),
        Some((px(20. - 3. - 3.), true))
    );
    let kbl = CheckboxLook::of(&resolved("kde-breeze", ColorMode::Light), true, false);
    assert_eq!(
        kbl.map(|l| (l.mark_size, l.mark_padded)),
        Some((px(20. - 1. - 1.), false))
    );
}

/// A colour as 8-bit sRGB channels, alpha dropped: what a capture reads.
fn rgb8(colour: gpui::Hsla) -> [u8; 3] {
    let rgba = gpui::Rgba::from(colour);
    let channel = |v: f32| (v.clamp(0., 1.) * 255.).round() as u8;
    [channel(rgba.r), channel(rgba.g), channel(rgba.b)]
}

/// docs/platform-facts.md §2.1.6: libadwaita fades a disabled switch as one
/// widget, `filter: Opacity(..)` (`_switch.scss:20-22`), so its white thumb
/// stays white on the track and the pair fades over the window at 0.5:
/// light #fcfcfd, dark #909092 -- where fading each part on its own gave
/// #cbdff7 and #95a9c2. The track, faded as one, is its own colour over the
/// window at half strength.
#[test]
fn a_disabled_switch_fades_as_one_over_its_backdrop() {
    for (mode, thumb) in [
        (ColorMode::Light, [0xfc, 0xfc, 0xfd]),
        (ColorMode::Dark, [0x90, 0x90, 0x92]),
    ] {
        let r = resolved("adwaita", mode);
        let ground = c(r.defaults.background_color);
        let look = SwitchLook::of(&r, true, true).map(|l| l.faded_over(ground));
        assert_eq!(look.map(|l| l.opacity), Some(1.), "{mode:?}");
        let got = look.map(|l| rgb8(l.thumb_color));
        let near = got.is_some_and(|g| g.iter().zip(thumb).all(|(g, want)| g.abs_diff(want) <= 1));
        assert!(near, "{mode:?}: thumb {got:?}, want {thumb:?}");
        let track = r
            .switch
            .disabled_checked_background
            .unwrap_or(r.switch.checked_background);
        assert_eq!(
            look.map(|l| l.track),
            Some(over(
                ground,
                over(ground, c(track)).opacity(r.switch.disabled_opacity)
            )),
            "{mode:?}: the track over the window"
        );
    }
    // An enabled switch, or one the platform dims by colour alone, is as it
    // was.
    let r = resolved("kde-breeze", ColorMode::Light);
    let ground = c(r.defaults.background_color);
    let look = SwitchLook::of(&r, true, true);
    assert_eq!(look.map(|l| l.faded_over(ground)), look);
}

/// A disabled switch faded as one fades its track's outline over the track
/// it is drawn inside, not over the window: a translucent outline (windows-11's
/// `#00000037`, material-3's `#1d1b201f`) shows the track through it. On
/// adwaita, which fades by 0.5, with an outline no bundled preset states there.
#[test]
fn a_faded_switch_fades_its_outline_over_its_track() {
    let mut r = resolved("adwaita", ColorMode::Light);
    let edge = native_theme::color::Rgba::new(0, 0, 0, 0x37);
    r.switch.unchecked_track_border_color = Some(edge);
    r.switch.track_border_line_width = Some(1.);
    let a = r.switch.disabled_opacity;
    assert!(a < 1., "adwaita no longer fades a disabled switch");
    let ground = c(r.defaults.background_color);
    let track = c(r
        .switch
        .disabled_unchecked_background
        .unwrap_or(r.switch.unchecked_background));
    let look = SwitchLook::of(&r, false, true).map(|l| l.faded_over(ground));
    assert_eq!(
        look.and_then(|l| l.track_border),
        Some(over(ground, over(over(ground, track), c(edge)).opacity(a))),
        "the outline over the track, the pair faded over the window"
    );
}

/// The same rule for a checkbox: its mark on its fill, the pair faded over
/// the window.
#[test]
fn a_disabled_checkbox_fades_as_one_over_its_backdrop() {
    let r = resolved("adwaita", ColorMode::Light);
    let ground = c(r.defaults.background_color);
    let a = r.checkbox.disabled_opacity;
    assert!(a < 1., "adwaita dims a disabled checkbox by opacity");
    let look = CheckboxLook::of(&r, true, true);
    let faded = look.map(|l| l.faded_over(ground));
    let fill = look.map(|l| over(ground, l.fill));
    assert_eq!(faded.map(|l| l.opacity), Some(1.));
    assert_eq!(
        faded.map(|l| l.mark),
        look.zip(fill)
            .map(|(l, fill)| over(ground, over(fill, l.mark).opacity(a)))
    );
    assert_eq!(
        faded.map(|l| l.fill),
        fill.map(|fill| over(ground, fill.opacity(a)))
    );
}

/// M6-19: the slider by the same rule, the rail over the window, the fill
/// over the rail, the outline and the thumb over the fill, each faded over
/// the window: the fill comes out opaque, and the opacity is then 1.
#[test]
fn a_disabled_slider_fades_as_one_over_its_backdrop() {
    for mode in [ColorMode::Light, ColorMode::Dark] {
        let r = resolved("adwaita", mode);
        let s = &r.slider;
        let a = s.disabled_opacity;
        assert!(
            a < 1.,
            "{mode:?}: adwaita dims a disabled slider by opacity"
        );
        let ground = c(r.defaults.background_color);
        let track = c(s.disabled_track_color.unwrap_or(s.track_color));
        let fill = c(s.disabled_fill_color.unwrap_or(s.fill_color));
        let thumb = c(s.disabled_thumb_color.unwrap_or(s.thumb_color));
        let rail = over(ground, track);
        let filled = over(rail, fill);
        let look = SliderLook::of(&r, true).map(|l| l.faded_over(ground));
        assert_eq!(look.map(|l| l.opacity), Some(1.), "{mode:?}");
        assert_eq!(
            look.map(|l| l.fill),
            Some(faded(ground, rail, fill, a)),
            "{mode:?}: the fill over the rail"
        );
        assert_eq!(look.map(|l| l.fill.a), Some(1.), "{mode:?}: opaque");
        assert_eq!(
            look.map(|l| l.track),
            Some(faded(ground, ground, track, a)),
            "{mode:?}: the rail over the window"
        );
        assert_eq!(
            look.map(|l| l.thumb_color),
            Some(faded(ground, filled, thumb, a)),
            "{mode:?}: the thumb over the fill"
        );
        assert_eq!(
            look.map(|l| l.outline),
            SliderLook::of(&r, true).map(|l| faded(ground, filled, l.outline, a)),
            "{mode:?}: the outline over the fill"
        );
    }
    // An enabled slider, or one the platform dims by colour alone, is as it
    // was.
    let r = resolved("kde-breeze", ColorMode::Light);
    let ground = c(r.defaults.background_color);
    let look = SliderLook::of(&r, true);
    assert_eq!(look.map(|l| l.faded_over(ground)), look);
    let enabled = SliderLook::of(&resolved("adwaita", ColorMode::Light), false);
    assert_eq!(enabled.map(|l| l.faded_over(ground)), enabled);
}

#[test]
fn a_switch_paints_switch_theme() {
    for (preset, mode) in PRESETS {
        let r = resolved(preset, mode);
        let s = &r.switch;
        let on = SwitchLook::of(&r, true, false).expect("finite");
        assert_eq!(on.track_width, px(s.track_width), "{preset}");
        assert_eq!(on.track_height, px(s.track_height), "{preset}");
        assert_eq!(on.track_radius, px(s.track_radius), "{preset}");
        assert_eq!(on.thumb, px(s.thumb_diameter), "{preset}");
        assert_eq!(
            on.inset,
            (px(s.track_height) - px(s.thumb_diameter)) / 2.,
            "{preset}"
        );
        assert_eq!(on.track, c(s.checked_background), "{preset}");
        assert_eq!(
            on.hover_track,
            Some(
                s.hover_checked_background
                    .map_or(c(s.checked_background), |h| {
                        over(c(s.checked_background), c(h))
                    })
            ),
            "{preset}"
        );
        assert_eq!(on.thumb_color, c(s.thumb_background), "{preset}");
        assert_eq!(on.label, c(r.defaults.text_color), "{preset}");
        assert_eq!(on.end_inset, on.inset.max(px(0.)), "{preset}");
        assert_eq!(
            on.thumb_left(true),
            on.track_width - on.end_inset - on.thumb,
            "{preset}"
        );
        assert_eq!(on.thumb_left(false), on.end_inset, "{preset}");

        let off = SwitchLook::of(&r, false, false).expect("finite");
        assert_eq!(off.track, c(s.unchecked_background), "{preset}");
        let off_thumb = s.unchecked_thumb_diameter.unwrap_or(s.thumb_diameter);
        assert_eq!(off.thumb, px(off_thumb), "{preset}");
        assert_eq!(
            off.inset,
            (px(s.track_height) - px(off_thumb)) / 2.,
            "{preset}"
        );
        assert_eq!(
            off.thumb_color,
            c(s.unchecked_thumb_background.unwrap_or(s.thumb_background)),
            "{preset}"
        );

        let disabled_on = SwitchLook::of(&r, true, true).expect("finite");
        assert_eq!(
            disabled_on.track,
            c(s.disabled_checked_background
                .unwrap_or(s.checked_background)),
            "{preset}"
        );
        assert_eq!(
            disabled_on.thumb_color,
            c(s.disabled_thumb_color.unwrap_or(s.thumb_background)),
            "{preset}"
        );
        assert_eq!(disabled_on.hover_track, None, "{preset}");
        assert_eq!(
            disabled_on.label,
            c(r.defaults.disabled_text_color),
            "{preset}"
        );
        let disabled_off = SwitchLook::of(&r, false, true).expect("finite");
        assert_eq!(
            disabled_off.track,
            c(s.disabled_unchecked_background
                .unwrap_or(s.unchecked_background)),
            "{preset}"
        );
        assert_eq!(
            disabled_off.thumb_color,
            c(s.disabled_thumb_color
                .or(s.unchecked_thumb_background)
                .unwrap_or(s.thumb_background)),
            "{preset}"
        );
    }
}

/// An unset hover is the part at rest, also where the part is translucent:
/// a translucent colour laid over itself covers more than it does alone, so
/// copying the colour it covers would show a hover no source gives.
/// macos-tahoe states neither an off-track hover nor a thumb hover, over a
/// translucent off track (both variants) and a translucent thumb (dark).
#[test]
fn an_unset_hover_leaves_a_translucent_part_at_rest() {
    for mode in [ColorMode::Light, ColorMode::Dark] {
        let r = resolved("macos-tahoe", mode);
        let s = &r.switch;
        assert!(
            s.hover_unchecked_background.is_none() && c(s.unchecked_background).a < 1.,
            "macos-tahoe {mode:?} no longer discriminates: its off-track hover is \
             {:?} over {:?}",
            s.hover_unchecked_background,
            s.unchecked_background
        );
        let off = SwitchLook::of(&r, false, false).expect("finite");
        assert_eq!(off.hover_track, Some(off.track), "{mode:?}: the off track");
    }
    let r = resolved("macos-tahoe", ColorMode::Dark);
    let s = &r.slider;
    assert!(
        s.thumb_hover_color.is_none() && c(s.thumb_color).a < 1.,
        "macos-tahoe dark no longer discriminates: its thumb hover is {:?} over {:?}",
        s.thumb_hover_color,
        s.thumb_color
    );
    let look = SliderLook::of(&r, false).expect("finite");
    assert_eq!(look.hover_thumb, Some(look.thumb_color), "the thumb");
}

#[test]
fn a_slider_paints_slider_theme() {
    for (preset, mode) in PRESETS {
        let r = resolved(preset, mode);
        let s = &r.slider;
        let look = SliderLook::of(&r, false).expect("finite");
        assert_eq!(look.track_height, px(s.track_height), "{preset}");
        assert_eq!(look.thumb, px(s.thumb_diameter), "{preset}");
        assert_eq!(
            look.height,
            px(s.thumb_diameter.max(s.track_height)),
            "{preset}"
        );
        assert_eq!(look.track, c(s.track_color), "{preset}");
        assert_eq!(look.fill, c(s.fill_color), "{preset}");
        assert_eq!(look.thumb_color, c(s.thumb_color), "{preset}");
        assert_eq!(
            look.hover_thumb,
            Some(
                s.thumb_hover_color
                    .map_or(c(s.thumb_color), |h| over(c(s.thumb_color), c(h)))
            ),
            "{preset}"
        );
        let disabled = SliderLook::of(&r, true).expect("finite");
        assert_eq!(
            disabled.track,
            c(s.disabled_track_color.unwrap_or(s.track_color)),
            "{preset}"
        );
        assert_eq!(
            disabled.fill,
            c(s.disabled_fill_color.unwrap_or(s.fill_color)),
            "{preset}"
        );
        assert_eq!(
            disabled.thumb_color,
            c(s.disabled_thumb_color.unwrap_or(s.thumb_color)),
            "{preset}"
        );
        assert_eq!(disabled.hover_thumb, None, "{preset}");
    }
}

#[test]
fn a_progress_bar_and_a_spinner_paint_their_themes() {
    for (preset, mode) in PRESETS {
        let r = resolved(preset, mode);
        let p = &r.progress_bar;
        let bar = ProgressBarLook::of(&r).expect("finite");
        assert_eq!(bar.height, px(p.track_height), "{preset}");
        assert_eq!(bar.min_width, px(p.min_width), "{preset}");
        assert_eq!(bar.radius, px(p.border.corner_radius), "{preset}");
        assert_eq!(bar.border_width, px(p.border.line_width), "{preset}");
        assert_eq!(bar.border, c(p.border.color), "{preset}");
        assert_eq!(bar.track, c(p.track_color), "{preset}");
        assert_eq!(bar.fill, c(p.fill_color), "{preset}");

        let s = &r.spinner;
        let spinner = SpinnerLook::of(&r).expect("finite");
        assert_eq!(
            spinner.diameter,
            px(s.diameter.max(s.min_diameter)),
            "{preset}"
        );
        assert_eq!(spinner.stroke, px(s.stroke_width), "{preset}");
        assert_eq!(spinner.color, c(s.fill_color), "{preset}");
        assert_eq!(
            spinner.path_radius(),
            (px(s.diameter) - px(s.stroke_width)) / 2.,
            "{preset}"
        );
    }
}

/// Every Size draws the platform's spinner: Medium at `spinner.diameter`,
/// the others at gpui-component's own ladder anchored there, none below
/// `spinner.min_diameter`.
#[test]
fn a_spinner_of_every_size_is_the_platforms() {
    use gpui_component::Size;
    for (preset, mode) in PRESETS {
        let r = resolved(preset, mode);
        let s = &r.spinner;
        let at = |size| SpinnerLook::of_size(&r, size).expect("finite");
        let (xsmall, small, medium, large) = (
            at(Size::XSmall),
            at(Size::Small),
            at(Size::Medium),
            at(Size::Large),
        );
        assert_eq!(Some(medium), SpinnerLook::of(&r), "{preset}");
        assert!(xsmall.diameter <= small.diameter, "{preset}");
        assert!(small.diameter <= medium.diameter, "{preset}");
        assert!(medium.diameter <= large.diameter, "{preset}");
        assert_eq!(
            small.diameter,
            px((s.diameter * 0.875).max(s.min_diameter)),
            "{preset}"
        );
        assert_eq!(
            large.diameter,
            px((s.diameter * 1.5).max(s.min_diameter)),
            "{preset}"
        );
        assert_eq!(
            at(Size::Size(px(10.))).diameter,
            px(10_f32.max(s.min_diameter)),
            "{preset}"
        );
        for look in [xsmall, small, large] {
            assert_eq!(look.stroke, px(s.stroke_width), "{preset}");
            assert_eq!(look.color, c(s.fill_color), "{preset}");
        }
    }
}

#[test]
fn a_tab_bar_paints_tab_theme() {
    for (preset, mode) in PRESETS {
        let r = resolved(preset, mode);
        let t = &r.tab;
        let look = TabLook::of(&r).expect("finite");
        assert_eq!(look.bar, c(t.bar_background), "{preset}");
        assert_eq!(look.idle, c(t.background_color), "{preset}");
        assert_eq!(look.idle_text, c(t.font.color), "{preset}");
        assert_eq!(look.hover_text, c(t.hover_text_color), "{preset}");
        assert_eq!(look.active, c(t.active_background), "{preset}");
        assert_eq!(look.active_text, c(t.active_text_color), "{preset}");
        assert_eq!(look.border, c(t.border.color), "{preset}");
        assert_eq!(look.border_width, px(t.border.line_width), "{preset}");
        assert_eq!(look.radius, px(t.border.corner_radius), "{preset}");
        assert_eq!(look.min_width, px(t.min_width), "{preset}");
        assert_eq!(look.min_height, px(t.min_height), "{preset}");
        if let Some(gap) = t.item_gap {
            assert_eq!(look.gap, px(gap), "{preset}: tab.item_gap");
        }
        let hover = t.hover_background.expect("both presets state a tab hover");
        assert_eq!(look.hover, over(c(t.bar_background), c(hover)), "{preset}");
    }
    // Breeze: the unselected tab is darker than the bar, and the hover
    // replaces that fill (docs/platform-facts.md §2.11).
    let r = resolved("kde-breeze", ColorMode::Dark);
    let look = TabLook::of(&r).expect("finite");
    assert_ne!(look.idle, look.bar);
    assert_eq!(look.padding_left, px(8.));
}

#[gpui::test]
fn the_tab_bar_outlines_only_the_selected_tab(cx: &mut TestAppContext) {
    let cx = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(|_, _| {
            div()
                .child(
                    TabBar::new("t")
                        .child(Tab::new("One").debug_selector(|| "one".into()))
                        .child(Tab::new("Two").debug_selector(|| "two".into()))
                        .selected_index(1),
                )
                .into_any_element()
        }),
    );
    let r = resolved("kde-breeze", ColorMode::Light);
    let (one, two) = (bounds(cx, "one"), bounds(cx, "two"));
    assert_eq!(one.size.height, px(r.tab.min_height));
    assert!(one.size.width >= px(r.tab.min_width));
    assert_eq!(
        Some(two.origin.x - (one.origin.x + one.size.width)),
        r.tab.item_gap.map(px),
        "neighbouring tabs tab.item_gap apart"
    );
    // gpui paints a box's fill and its border as two quads.
    let painted = |cx: &mut VisualTestContext, b: Bounds<Pixels>| {
        cx.update(|window, _| {
            let b = b.scale(window.scale_factor());
            let quads = window.painted_quads();
            let fill = quads
                .iter()
                .find(|q| q.bounds == b && !q.background.is_transparent())
                .and_then(|q| q.background.as_solid());
            let edge = quads
                .iter()
                .find(|q| q.bounds == b && q.border_widths.top.0 > 0.)
                .map(|q| q.border_color);
            (fill, edge)
        })
    };
    assert_eq!(
        painted(cx, two),
        (
            Some(c(r.tab.active_background)),
            Some(c(r.tab.border.color))
        ),
        "the selected tab: active_background outlined in tab.border"
    );
    let look = TabLook::of(&r).expect("finite");
    cx.simulate_mouse_move(centre(one), None, Modifiers::default());
    redraw(cx);
    assert_eq!(
        painted(cx, one).0,
        Some(look.hover),
        "hovered: tab.hover_background"
    );
    cx.simulate_mouse_move(point(px(500.), px(400.)), None, Modifiers::default());
    redraw(cx);
    let (fill, edge) = painted(cx, one);
    assert_eq!(fill, Some(c(r.tab.background_color)));
    assert!(
        edge.is_none_or(|e| e.is_transparent()),
        "the unselected tab has no outline"
    );
}

/// docs/platform-facts.md:1361-1363: the selected tab's line, as thick as
/// stated, across the tab's outer edge on the stated side -- Breeze's on top,
/// Material's at the bottom.
#[gpui::test]
fn the_selected_breeze_tab_carries_its_indicator(cx: &mut TestAppContext) {
    let (preset, mode) = PRESETS[0];
    selected_tab_carries_the_stated_indicator(cx, preset, mode);
}

#[gpui::test]
fn the_selected_material_tab_carries_its_indicator(cx: &mut TestAppContext) {
    let (preset, mode) = PRESETS[1];
    selected_tab_carries_the_stated_indicator(cx, preset, mode);
}

fn selected_tab_carries_the_stated_indicator(
    cx: &mut TestAppContext,
    preset: &str,
    mode: ColorMode,
) {
    {
        let cx = window_with(
            cx,
            Some((preset, mode)),
            Box::new(|_, _| {
                div()
                    .child(
                        TabBar::new("t")
                            .child(Tab::new("One"))
                            .child(Tab::new("Two").debug_selector(|| "two".into()))
                            .selected_index(1),
                    )
                    .into_any_element()
            }),
        );
        let r = resolved(preset, mode);
        let t = &r.tab;
        assert_eq!(
            TabLook::of(&r).and_then(|l| l.indicator),
            match (
                t.active_indicator_color,
                t.active_indicator_width,
                t.active_indicator_side
            ) {
                (Some(colour), Some(width), Some(side)) => Some(TabIndicator {
                    color: c(colour),
                    width: px(width),
                    side,
                }),
                _ => None,
            },
            "{preset}"
        );
        let (two, line) = (bounds(cx, "two"), bounds(cx, "native-tab-indicator"));
        assert_eq!(line.size.width, two.size.width, "{preset}: across the tab");
        assert_eq!(
            Some(line.size.height),
            t.active_indicator_width.map(px),
            "{preset}"
        );
        let edge = match t.active_indicator_side {
            Some(TabIndicatorSide::Top) => line.origin.y == two.origin.y,
            Some(TabIndicatorSide::Bottom) => {
                line.origin.y + line.size.height == two.origin.y + two.size.height
            }
            None => false,
        };
        assert!(edge, "{preset}: along the stated edge");
    }
}

#[gpui::test]
fn the_separator_is_the_stated_thickness(cx: &mut TestAppContext) {
    let cx = window_with(
        cx,
        Some(("material-3", ColorMode::Dark)),
        Box::new(|_, _| {
            div()
                .w(px(200.))
                .child(Separator::horizontal())
                .into_any_element()
        }),
    );
    let r = resolved("material-3", ColorMode::Dark);
    let line = bounds(cx, "native-separator");
    assert_eq!(line.size.height, px(r.separator.line_width));
    assert_eq!(line.size.width, px(200.));
    let look = SeparatorLook::of(&r).expect("finite");
    assert_eq!(look.color, c(r.separator.line_color));
}

/// M6-21: a checkbox's or a radio's label is drawn in `checkbox.font`'s
/// family and style, as in its size and weight, where the theme states
/// them; a style is gpui's of the same name.
#[test]
fn the_checkbox_label_takes_the_stated_family_and_style() {
    let mut r = resolved("kde-breeze", ColorMode::Light);
    r.checkbox.font.family = "Noto Serif".into();
    for (style, drawn) in [
        (FontStyle::Normal, gpui::FontStyle::Normal),
        (FontStyle::Italic, gpui::FontStyle::Italic),
        (FontStyle::Oblique, gpui::FontStyle::Oblique),
    ] {
        r.checkbox.font.style = style;
        let mut label = font_text(&r.checkbox.font, crate::Native::unscaled(&r));
        let text = label.text_style();
        assert_eq!(
            text.font_family,
            Some(crate::font_family(&r.checkbox.font.family)),
            "{style:?}"
        );
        assert_eq!(text.font_style, Some(drawn), "{style:?}");
        assert_eq!(text.font_size, Some(px(r.checkbox.font.size).into()));
        assert_eq!(
            text.font_weight,
            Some(gpui::FontWeight(f32::from(r.checkbox.font.weight)))
        );
        assert_eq!(
            text.line_height,
            Some(gpui::relative(r.defaults.line_height))
        );
    }
}

/// M6-21: a tab's label is drawn in `tab.font`'s family and style, as in its
/// size and weight, where the theme states them.
#[test]
fn the_tab_label_takes_the_stated_family_and_style() {
    let mut r = resolved("kde-breeze", ColorMode::Light);
    r.tab.font.family = "Noto Serif".into();
    for (style, drawn) in [
        (FontStyle::Normal, gpui::FontStyle::Normal),
        (FontStyle::Italic, gpui::FontStyle::Italic),
        (FontStyle::Oblique, gpui::FontStyle::Oblique),
    ] {
        r.tab.font.style = style;
        let mut label = font_text(&r.tab.font, crate::Native::unscaled(&r));
        let text = label.text_style();
        assert_eq!(
            text.font_family,
            Some(crate::font_family(&r.tab.font.family)),
            "{style:?}"
        );
        assert_eq!(text.font_style, Some(drawn), "{style:?}");
        assert_eq!(text.font_size, Some(px(r.tab.font.size).into()));
        assert_eq!(
            text.font_weight,
            Some(gpui::FontWeight(f32::from(r.tab.font.weight)))
        );
        assert_eq!(
            text.line_height,
            Some(gpui::relative(r.defaults.line_height))
        );
    }
}

/// The tabs of `tab_window`'s bar, each its own debug selector.
const TAB_NAMES: [&str; 8] = [
    "tab0", "tab1", "tab2", "tab3", "tab4", "tab5", "tab6", "tab7",
];

/// A window drawing a kde-breeze tab bar of [`TAB_NAMES`]' eight tabs,
/// across the harness's 200px column, `selected` selected and its scroll
/// tracked by `handle` where one is given; a click hands `on_click` the
/// tab's index.
fn tab_window(
    cx: &mut TestAppContext,
    selected: Rc<Cell<usize>>,
    on_click: Rc<dyn Fn(usize)>,
    handle: Option<gpui::ScrollHandle>,
) -> &mut VisualTestContext {
    window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(move |_, _| {
            use gpui::prelude::FluentBuilder as _;
            let on_click = on_click.clone();
            TabBar::new("t")
                .w_full()
                .children(TAB_NAMES.map(|name| Tab::new(name).debug_selector(|| name.into())))
                .selected_index(selected.get())
                .on_click(move |ix, _, _| on_click(*ix))
                .when_some(handle.clone(), |bar, handle| bar.track_scroll(&handle))
                .into_any_element()
        }),
    )
}

/// The first tab, of `tab_window`'s, that the bar's right edge cuts.
fn half_hidden_tab(cx: &mut VisualTestContext) -> (usize, &'static str) {
    let bar = bounds(cx, "native-tab-bar");
    TAB_NAMES
        .into_iter()
        .enumerate()
        .find(|(_, name)| {
            let tab = bounds(cx, name);
            tab.left() < bar.right() && tab.right() > bar.right()
        })
        .expect("the bar's right edge cuts a tab")
}

/// M6-26: a click on a tab the bar's edge half hides scrolls it into view
/// on the next frame, with the bar's own handle where the caller tracks
/// none.
#[gpui::test]
fn a_clicked_half_hidden_tab_scrolls_into_view(cx: &mut TestAppContext) {
    let clicked = Rc::new(Cell::new(None));
    let c = clicked.clone();
    let cx = tab_window(
        cx,
        Rc::new(Cell::new(0)),
        Rc::new(move |ix| c.set(Some(ix))),
        None,
    );
    let bar = bounds(cx, "native-tab-bar");
    let (ix, name) = half_hidden_tab(cx);
    let tab = bounds(cx, name);
    let visible = point((tab.left() + bar.right()) / 2., tab.center().y);
    cx.simulate_click(visible, Modifiers::default());
    redraw(cx);
    assert_eq!(clicked.get(), Some(ix), "the caller's handler ran");
    let shown = bounds(cx, name);
    assert_eq!(shown.right(), bar.right(), "scrolled just into view");
    assert!(shown.left() >= bar.left());
}

/// M6-26: a tracked handle is the one the bar scrolls; a change of the
/// selection scrolls nothing, as gpui-component's `TabBar::track_scroll`
/// says (tab/tab_bar.rs:124-128).
#[gpui::test]
fn a_tracked_tab_bar_scrolls_its_handle_on_a_click_only(cx: &mut TestAppContext) {
    let handle = gpui::ScrollHandle::new();
    let selected = Rc::new(Cell::new(0));
    let cx = tab_window(cx, selected.clone(), Rc::new(|_| {}), Some(handle.clone()));
    let (ix, name) = half_hidden_tab(cx);
    selected.set(ix);
    redraw(cx);
    assert_eq!(
        handle.offset().x,
        px(0.),
        "a selection change reveals nothing"
    );
    let bar = bounds(cx, "native-tab-bar");
    let tab = bounds(cx, name);
    let visible = point((tab.left() + bar.right()) / 2., tab.center().y);
    cx.simulate_click(visible, Modifiers::default());
    redraw(cx);
    assert_eq!(
        handle.offset().x,
        bar.right() - tab.right(),
        "the tracked handle scrolled the tab just into view"
    );
}

/// M6-26: the bar asks for the clicked tab before the caller's handler runs,
/// and the scroll lands on the next frame: the handler still sees the old
/// offset, and a reveal it asks for itself replaces the bar's.
#[gpui::test]
fn a_click_asks_for_the_reveal_before_the_handler(cx: &mut TestAppContext) {
    let handle = gpui::ScrollHandle::new();
    let seen = Rc::new(Cell::new(None));
    let (h, s) = (handle.clone(), seen.clone());
    let cx = tab_window(
        cx,
        Rc::new(Cell::new(0)),
        Rc::new(move |_| {
            s.set(Some(h.offset().x));
            h.scroll_to_item(0);
        }),
        Some(handle.clone()),
    );
    let bar = bounds(cx, "native-tab-bar");
    let (_, name) = half_hidden_tab(cx);
    let tab = bounds(cx, name);
    let visible = point((tab.left() + bar.right()) / 2., tab.center().y);
    cx.simulate_click(visible, Modifiers::default());
    redraw(cx);
    assert_eq!(seen.get(), Some(px(0.)), "the handler saw the old offset");
    assert_eq!(handle.offset().x, px(0.), "the handler's reveal won");
}

#[test]
fn a_length_that_is_not_finite_falls_back() {
    let mut r = resolved("kde-breeze", ColorMode::Light);
    r.checkbox.indicator_width = f32::NAN;
    r.switch.track_width = f32::INFINITY;
    r.slider.thumb_diameter = -1.;
    r.progress_bar.track_height = f32::NAN;
    r.spinner.stroke_width = f32::NEG_INFINITY;
    assert!(CheckboxLook::of(&r, false, false).is_none());
    assert!(SwitchLook::of(&r, false, false).is_none());
    assert!(SliderLook::of(&r, false).is_none());
    assert!(ProgressBarLook::of(&r).is_none());
    assert!(SpinnerLook::of(&r).is_none());
    for size in [
        gpui_component::Size::XSmall,
        gpui_component::Size::Small,
        gpui_component::Size::Medium,
        gpui_component::Size::Large,
        gpui_component::Size::Size(px(10.)),
    ] {
        assert!(SpinnerLook::of_size(&r, size).is_none(), "{size:?}");
    }
}

// ---------------------------------------------------------------------------
// Rendered: geometry and behaviour on the headless platform
// ---------------------------------------------------------------------------

type Build = Box<dyn Fn(&mut Window, &mut Context<Harness>) -> AnyElement>;

struct Harness {
    build: Build,
}

impl Render for Harness {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("harness")
            .tab_group()
            .size(px(400.))
            .flex()
            .items_start()
            .child(
                div()
                    .flex_none()
                    .w(px(200.))
                    .child((self.build)(window, cx)),
            )
    }
}

/// A window drawing `build`, under `preset` when one is given, with no
/// native theme installed otherwise.
fn window_with<'a>(
    cx: &'a mut TestAppContext,
    theme: Option<(&str, ColorMode)>,
    build: Build,
) -> &'a mut VisualTestContext {
    cx.update(|cx| {
        gpui_component::init(cx);
        if let Some((preset, mode)) = theme {
            let r = resolved(preset, mode);
            let prefs = AccessibilityPreferences::default();
            let theme = crate::to_theme(&r, preset, mode == ColorMode::Dark, &prefs);
            crate::apply(theme, &r, &prefs, cx);
        }
    });
    let (_, cx) = cx.add_window_view(|_, _| Harness { build });
    cx.update(|window, cx| window.draw(cx).clear(cx));
    cx
}

fn bounds(cx: &mut VisualTestContext, selector: &'static str) -> Bounds<Pixels> {
    cx.debug_bounds(selector)
        .unwrap_or_else(|| panic!("{selector} was not laid out"))
}

fn centre(b: Bounds<Pixels>) -> gpui::Point<Pixels> {
    b.center()
}

fn redraw(cx: &mut VisualTestContext) {
    // A full frame: GPUI Fast renders again only the views that were
    // notified, and in test mode a partly reused frame keeps every bound the
    // last frame drew (gpui-fast 0.1.7/0.1.8 src/fast/retained.rs:1360-1371). On
    // gpui-pre it only marks the window dirty.
    cx.update(|window, cx| {
        window.refresh();
        window.draw(cx).clear(cx)
    });
}

/// A key pressed and released, as gpui-component's own tests activate a
/// control (checkbox.rs, `activate_key`).
#[allow(clippy::unwrap_used)]
fn activate_key(cx: &mut VisualTestContext, key: &str) {
    let keystroke = gpui::Keystroke::parse(key).unwrap();
    cx.simulate_event(gpui::KeyDownEvent {
        keystroke: keystroke.clone(),
        is_held: false,
        prefer_character_input: false,
    });
    cx.simulate_event(gpui::KeyUpEvent { keystroke });
}

/// Focus moved to the next tab stop, which must exist.
fn focus_next(cx: &mut VisualTestContext) {
    cx.update(|window, cx| window.focus_next(cx));
    cx.update(|window, cx| assert!(window.focused(cx).is_some(), "nothing took focus"));
}

#[gpui::test]
fn the_indicator_is_the_stated_size(cx: &mut TestAppContext) {
    let cx = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(|_, _| Checkbox::new("c").label("Label").into_any_element()),
    );
    let r = resolved("kde-breeze", ColorMode::Light);
    let b = bounds(cx, "native-checkbox-indicator");
    assert_eq!(b.size.width, px(r.checkbox.indicator_width));
    assert_eq!(b.size.height, px(r.checkbox.indicator_width));
    let label = bounds(cx, "native-checkbox-label");
    assert_eq!(
        label.origin.x - b.origin.x - b.size.width,
        px(r.checkbox.label_gap),
        "the label sits label_gap from the indicator"
    );
}

#[gpui::test]
fn the_radio_indicator_is_the_stated_size(cx: &mut TestAppContext) {
    let cx = window_with(
        cx,
        Some(("material-3", ColorMode::Dark)),
        Box::new(|_, _| Radio::new("r").label("A").into_any_element()),
    );
    let r = resolved("material-3", ColorMode::Dark);
    assert_eq!(r.checkbox.radio_indicator_width, Some(20.));
    let b = bounds(cx, "native-checkbox-indicator");
    assert_eq!(b.size.width, px(20.));
    assert_eq!(b.size.height, px(20.));
}

/// docs/platform-facts.md:1241: Breeze's dot is 6px across, centred in the
/// 20px circle.
#[gpui::test]
fn a_selected_radio_draws_the_stated_dot(cx: &mut TestAppContext) {
    let cx = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(|_, _| Radio::new("r").label("A").checked(true).into_any_element()),
    );
    let r = resolved("kde-breeze", ColorMode::Light);
    assert_eq!(r.checkbox.radio_dot_diameter, Some(6.));
    let circle = bounds(cx, "native-checkbox-indicator");
    let dot = bounds(cx, "native-radio-dot");
    assert_eq!(dot.size.width, px(6.));
    assert_eq!(dot.size.height, px(6.));
    assert_eq!(dot.center(), circle.center(), "centred in the circle");
}

#[gpui::test]
fn the_switch_track_and_thumb_are_the_stated_sizes(cx: &mut TestAppContext) {
    let cx = window_with(
        cx,
        Some(("material-3", ColorMode::Dark)),
        Box::new(|_, _| {
            Switch::new("s")
                .label("On")
                .checked(true)
                .into_any_element()
        }),
    );
    let r = resolved("material-3", ColorMode::Dark);
    let s = &r.switch;
    let track = bounds(cx, "native-switch-track");
    assert_eq!(track.size.width, px(s.track_width));
    assert_eq!(track.size.height, px(s.track_height));
    let thumb = bounds(cx, "native-switch-thumb");
    assert_eq!(thumb.size.width, px(s.thumb_diameter));
    assert_eq!(thumb.size.height, px(s.thumb_diameter));
    let inset = (px(s.track_height) - px(s.thumb_diameter)) / 2.;
    assert_eq!(
        thumb.origin.y - track.origin.y,
        inset,
        "centred on the axis"
    );
    assert_eq!(
        track.origin.x + track.size.width - (thumb.origin.x + thumb.size.width),
        inset,
        "on: inset from the right end"
    );
}

/// docs/platform-facts.md §2.21: qqc2-desktop-style's handle is the 18 of the
/// indicator on a 12-tall track, so it overhangs the track by 3 above and
/// below and sits flush with the track's ends (`SwitchIndicator.qml:41-48`,
/// `:65-76`, `:68`).
#[gpui::test]
fn a_thumb_taller_than_its_track_sits_flush_with_its_ends(cx: &mut TestAppContext) {
    for checked in [false, true] {
        let cx = window_with(
            cx,
            Some(("kde-breeze", ColorMode::Light)),
            Box::new(move |_, _| Switch::new("s").checked(checked).into_any_element()),
        );
        let r = resolved("kde-breeze", ColorMode::Light);
        let s = &r.switch;
        assert!(
            s.thumb_diameter > s.track_height,
            "kde-breeze's thumb no longer overhangs its track"
        );
        let track = bounds(cx, "native-switch-track");
        let thumb = bounds(cx, "native-switch-thumb");
        assert_eq!(track.size.height, px(s.track_height));
        assert_eq!(
            thumb.origin.y - track.origin.y,
            (px(s.track_height) - px(s.thumb_diameter)) / 2.,
            "checked {checked}: overhangs above and below alike"
        );
        if checked {
            assert_eq!(
                thumb.origin.x + thumb.size.width,
                track.origin.x + track.size.width,
                "on: flush with the right end"
            );
        } else {
            assert_eq!(
                thumb.origin.x, track.origin.x,
                "off: flush with the left end"
            );
        }
    }
}

/// docs/platform-facts.md §2.21: the track is outlined in the colour the
/// theme states for its state, `track_border_line_width` wide, and the thumb
/// in `thumb_border_color`, `thumb_border_line_width` wide; a part whose
/// colour or width is unset has no outline. Under kde-breeze (both tracks
/// and the thumb outlined), windows-11 and material-3 (the off track only).
#[gpui::test]
fn the_switch_draws_the_stated_outlines(cx: &mut TestAppContext) {
    for preset in ["kde-breeze", "windows-11", "material-3"] {
        for mode in [ColorMode::Light, ColorMode::Dark] {
            for (checked, disabled) in [(false, false), (true, false), (false, true), (true, true)]
            {
                switch_draws_the_stated_outlines(cx, preset, mode, checked, disabled);
            }
        }
    }
}

fn switch_draws_the_stated_outlines(
    cx: &mut TestAppContext,
    preset: &'static str,
    mode: ColorMode,
    checked: bool,
    disabled: bool,
) {
    let cx = window_with(
        cx,
        Some((preset, mode)),
        Box::new(move |_, _| {
            Switch::new("s")
                .checked(checked)
                .disabled(disabled)
                .into_any_element()
        }),
    );
    let r = resolved(preset, mode);
    let s = &r.switch;
    let state = format!("{preset} {mode:?} checked {checked} disabled {disabled}");
    assert_eq!(s.disabled_opacity, 1., "{state}: drawn unfaded");
    assert!(
        s.unchecked_track_border_color.is_some() && s.track_border_line_width.is_some(),
        "{state}: {preset} no longer states the off track's outline (docs/platform-facts.md §2.21)"
    );
    let track_colour = match (checked, disabled) {
        (true, false) => s.checked_track_border_color,
        (false, false) => s.unchecked_track_border_color,
        (true, true) => s
            .disabled_checked_track_border_color
            .or(s.checked_track_border_color),
        (false, true) => s
            .disabled_unchecked_track_border_color
            .or(s.unchecked_track_border_color),
    };
    let stated = |colour: Option<native_theme::color::Rgba>, width: Option<f32>| {
        colour
            .zip(width)
            .map(|(colour, width)| (px(width), c(colour)))
    };
    let edge = |cx: &mut VisualTestContext, b: Bounds<Pixels>| {
        cx.update(|window, _| {
            let b = b.scale(window.scale_factor());
            window
                .painted_quads()
                .iter()
                .find(|q| q.bounds == b && q.border_widths.top.0 > 0.)
                .map(|q| {
                    (
                        px(q.border_widths.top.0 / window.scale_factor()),
                        q.border_color,
                    )
                })
        })
    };
    let track = bounds(cx, "native-switch-track");
    assert_eq!(
        edge(cx, track),
        stated(track_colour, s.track_border_line_width),
        "{state}: the track"
    );
    let thumb = bounds(cx, "native-switch-thumb");
    let diameter = if checked {
        s.thumb_diameter
    } else {
        s.unchecked_thumb_diameter.unwrap_or(s.thumb_diameter)
    };
    assert_eq!(thumb.size.width, px(diameter), "{state}: the thumb's box");
    // The thumb lies over the track's outline, as qqc2-desktop-style's
    // handle lies over its track (SwitchIndicator.qml:38-79): gpui paints a
    // box's border after its children, so an outline on the track itself
    // would cross the thumb.
    let order = cx.update(|window, _| {
        let quads = window.painted_quads();
        let at = |b: Bounds<Pixels>, edge: bool| {
            let b = b.scale(window.scale_factor());
            quads
                .iter()
                .position(|q| q.bounds == b && (q.border_widths.top.0 > 0.) == edge)
        };
        (at(track, true), at(thumb, false))
    });
    if let (Some(edge), Some(fill)) = order {
        assert!(
            edge < fill,
            "{state}: the track's outline is painted over the thumb"
        );
    }
    // Placed as without an outline.
    let inset = (px(s.track_height) - px(diameter)) / 2.;
    assert_eq!(
        thumb.origin.y - track.origin.y,
        inset,
        "{state}: on the axis"
    );
    let end = inset.max(px(0.));
    if checked {
        assert_eq!(
            track.origin.x + track.size.width - (thumb.origin.x + thumb.size.width),
            end,
            "{state}: at the right end"
        );
    } else {
        assert_eq!(
            thumb.origin.x - track.origin.x,
            end,
            "{state}: at the left end"
        );
    }
    assert_eq!(
        edge(cx, thumb),
        stated(s.thumb_border_color, s.thumb_border_line_width),
        "{state}: the thumb"
    );
}

#[gpui::test]
fn the_slider_rail_and_thumb_are_the_stated_sizes(cx: &mut TestAppContext) {
    let state = cx.new(|_| SliderState::new().default_value(0.));
    let s2 = state.clone();
    let cx = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(move |_, _| Slider::new(&s2).w(px(140.)).into_any_element()),
    );
    let r = resolved("kde-breeze", ColorMode::Light);
    let s = &r.slider;
    let root = bounds(cx, "native-slider");
    assert_eq!(root.size.width, px(140.));
    assert_eq!(root.size.height, px(s.thumb_diameter.max(s.track_height)));
    let rail = bounds(cx, "native-slider-rail");
    assert_eq!(rail.size.height, px(s.track_height));
    assert_eq!(
        rail.size.width, root.size.width,
        "the rail spans the widget"
    );
    let thumb = bounds(cx, "native-slider-thumb");
    assert_eq!(thumb.size.width, px(s.thumb_diameter));
    assert_eq!(thumb.size.height, px(s.thumb_diameter));
    assert_eq!(
        thumb.origin.x, root.origin.x,
        "at 0 the thumb starts the widget"
    );
    assert_eq!(centre(thumb).y, centre(rail).y, "centred on the rail");
    drop(state);
}

#[gpui::test]
fn the_progress_bar_and_spinner_are_the_stated_sizes(cx: &mut TestAppContext) {
    let cx = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(|_, _| {
            div()
                .flex()
                .flex_col()
                .child(ProgressBar::new("p").value(40.))
                .child(Spinner::new("s"))
                .into_any_element()
        }),
    );
    let r = resolved("kde-breeze", ColorMode::Light);
    let bar = bounds(cx, "native-progress");
    assert_eq!(bar.size.height, px(r.progress_bar.track_height));
    let spinner = bounds(cx, "native-spinner");
    assert_eq!(spinner.size.width, px(r.spinner.diameter));
    assert_eq!(spinner.size.height, px(r.spinner.diameter));
}

/// A loading bar in a 200 px column, under kde-breeze, and its look.
fn loading_window(cx: &mut TestAppContext) -> (&mut VisualTestContext, ProgressBarLook) {
    let cx = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(|_, _| ProgressBar::new("p").loading(true).into_any_element()),
    );
    let look = ProgressBarLook::of(&resolved("kde-breeze", ColorMode::Light)).expect("finite");
    (cx, look)
}

/// The fill's bounds after `ms` milliseconds more, on a frame drawn then.
fn fill_after(cx: &mut VisualTestContext, ms: u64) -> Bounds<Pixels> {
    std::thread::sleep(std::time::Duration::from_millis(ms));
    redraw(cx);
    bounds(cx, "native-progress-fill")
}

/// M6-28: a loading bar sweeps its fill across the track on
/// gpui-component's motion (progress/progress.rs:149-160), over the theme's
/// track. The fill's colour is checked still, under reduced motion: while it
/// sweeps, it is under a pixel wide for part of each turn, and gpui drops a
/// quad that empty.
#[gpui::test]
fn a_loading_progress_bar_sweeps_its_fill_in_the_theme(cx: &mut TestAppContext) {
    let (cx, look) = loading_window(cx);
    let bar = bounds(cx, "native-progress");
    assert_eq!(bar.size.height, look.height);
    assert_eq!(painted_fill(cx, bar), Some(look.track), "the theme's track");
    let first = fill_after(cx, 0);
    assert_eq!(first.size.height, bar.size.height, "the track's height");
    // Frames a little apart until one lays the fill out elsewhere: the sweep
    // takes a second (progress/progress.rs:152), so 2 s of frames cover two
    // turns however slowly the runner draws.
    let start = std::time::Instant::now();
    let moved = loop {
        if fill_after(cx, 20) != first {
            break true;
        }
        if start.elapsed() > std::time::Duration::from_secs(2) {
            break false;
        }
    };
    assert!(moved, "the fill moves");
}

/// M6-28: under reduced motion the loading fill stands still in the middle,
/// gpui-component's `left(relative(0.325)).right(relative(0.325))`
/// (progress/progress.rs:161-164).
#[gpui::test]
fn a_loading_progress_bar_stands_still_under_reduced_motion(cx: &mut TestAppContext) {
    let (cx, look) = loading_window(cx);
    cx.update(|_, cx| cx.set_reduce_motion(true));
    let bar = bounds(cx, "native-progress");
    let first = fill_after(cx, 0);
    let inset = bar.size.width * 0.325;
    assert_eq!(first.origin.x, bar.origin.x + inset);
    assert_eq!(first.size.width, bar.size.width - inset * 2.);
    assert_eq!(painted_fill(cx, first), Some(look.fill), "the theme's fill");
    assert_eq!(fill_after(cx, 150), first, "still");
}

/// The spinner draws the icon set's indicator where the set has one, filling
/// `spinner.diameter`, and the arc for a set without one.
#[gpui::test]
fn the_spinner_is_the_icon_sets_indicator_or_an_arc(cx: &mut TestAppContext) {
    use native_theme::theme::IconSet;
    let cx = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(|_, _| {
            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .debug_selector(|| "set".into())
                        .child(Spinner::new("set").icon_set(IconSet::Lucide)),
                )
                .child(
                    div()
                        .debug_selector(|| "none".into())
                        .child(Spinner::new("none").icon_set(None)),
                )
                .into_any_element()
        }),
    );
    let r = resolved("kde-breeze", ColorMode::Light);
    let within = |outer: Bounds<Pixels>, inner: Bounds<Pixels>| {
        outer.origin.y <= inner.origin.y
            && inner.origin.y + inner.size.height <= outer.origin.y + outer.size.height
    };
    let arc = bounds(cx, "native-spinner-arc");
    assert!(within(bounds(cx, "none"), arc), "icon_set(None): the arc");
    assert_eq!(arc.size.width, px(r.spinner.diameter));
    #[cfg(feature = "lucide-icons")]
    {
        let indicator = bounds(cx, "native-spinner-indicator");
        assert!(
            within(bounds(cx, "set"), indicator),
            "Lucide: its indicator"
        );
        assert_eq!(indicator.size.width, px(r.spinner.diameter));
        assert_eq!(indicator.size.height, px(r.spinner.diameter));
    }
}

#[gpui::test]
fn without_a_native_theme_the_widgets_are_gpui_components(cx: &mut TestAppContext) {
    let state = cx.new(|_| SliderState::new());
    let cx = window_with(
        cx,
        None,
        Box::new(move |_, _| {
            div()
                .flex()
                .flex_col()
                .child(Checkbox::new("c").label("C"))
                .child(Radio::new("r").label("R"))
                .child(Switch::new("s").label("S"))
                .child(Slider::new(&state).w(px(100.)))
                .child(ProgressBar::new("p").value(10.))
                .child(Spinner::new("sp"))
                .into_any_element()
        }),
    );
    for selector in [
        "native-checkbox-indicator",
        "native-switch-track",
        "native-slider",
        "native-progress",
        "native-spinner",
    ] {
        assert!(
            cx.debug_bounds(selector).is_none(),
            "{selector}: drawn without a native theme"
        );
    }
}

/// A checkbox (or switch) wired as gpui-component's: controlled, the
/// requested value written back.
fn toggler(
    cx: &mut TestAppContext,
    disabled: bool,
    switch: bool,
) -> (&mut VisualTestContext, Rc<Cell<bool>>, Rc<Cell<usize>>) {
    let value = Rc::new(Cell::new(false));
    let calls = Rc::new(Cell::new(0usize));
    let (v, n) = (value.clone(), calls.clone());
    let cx = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(move |_, _| {
            let (v2, n2) = (v.clone(), n.clone());
            let on_change = move |checked: &bool, _: &mut Window, _: &mut gpui::App| {
                v2.set(*checked);
                n2.set(n2.get() + 1);
            };
            if switch {
                Switch::new("t")
                    .label("T")
                    .checked(v.get())
                    .disabled(disabled)
                    .on_change(on_change)
                    .into_any_element()
            } else {
                Checkbox::new("t")
                    .label("T")
                    .checked(v.get())
                    .disabled(disabled)
                    .on_change(on_change)
                    .into_any_element()
            }
        }),
    );
    (cx, value, calls)
}

#[gpui::test]
fn a_checkbox_toggles_on_click_enter_and_space(cx: &mut TestAppContext) {
    let (cx, value, calls) = toggler(cx, false, false);
    let at = centre(bounds(cx, "native-checkbox-indicator"));
    cx.simulate_click(at, Modifiers::default());
    assert_eq!((value.get(), calls.get()), (true, 1));
    cx.update(|window, cx| assert!(window.focused(cx).is_none(), "a press moves no focus"));
    redraw(cx);
    focus_next(cx);
    activate_key(cx, "enter");
    assert_eq!((value.get(), calls.get()), (false, 2));
    redraw(cx);
    activate_key(cx, "space");
    assert_eq!((value.get(), calls.get()), (true, 3));
}

#[gpui::test]
fn a_disabled_checkbox_is_inert(cx: &mut TestAppContext) {
    let (cx, value, calls) = toggler(cx, true, false);
    let at = centre(bounds(cx, "native-checkbox-indicator"));
    cx.simulate_click(at, Modifiers::default());
    cx.update(|window, cx| window.focus_next(cx));
    cx.update(|window, cx| assert!(window.focused(cx).is_none(), "not focusable"));
    assert_eq!((value.get(), calls.get()), (false, 0));
}

#[gpui::test]
fn a_switch_toggles_on_click_and_from_the_keyboard(cx: &mut TestAppContext) {
    let (cx, value, calls) = toggler(cx, false, true);
    let at = centre(bounds(cx, "native-switch-track"));
    cx.simulate_click(at, Modifiers::default());
    assert_eq!((value.get(), calls.get()), (true, 1));
    redraw(cx);
    focus_next(cx);
    activate_key(cx, "space");
    assert_eq!((value.get(), calls.get()), (false, 2));
}

/// K10: the switch's own focus handle is the base switch's one tab stop. In
/// this harness the switch is the only focusable element, so Tab from it
/// comes back to the same handle; a second tab stop would take the focus.
#[gpui::test]
fn a_switch_keeps_one_tab_stop(cx: &mut TestAppContext) {
    let (cx, value, calls) = toggler(cx, false, true);
    focus_next(cx);
    let first = cx.update(|window, cx| window.focused(cx));
    activate_key(cx, "space");
    assert_eq!((value.get(), calls.get()), (true, 1));
    redraw(cx);
    focus_next(cx);
    assert_eq!(
        cx.update(|window, cx| window.focused(cx)),
        first,
        "Tab from the only switch landed on a second tab stop"
    );
}

#[gpui::test]
fn a_disabled_switch_is_inert(cx: &mut TestAppContext) {
    let (cx, value, calls) = toggler(cx, true, true);
    let at = centre(bounds(cx, "native-switch-track"));
    cx.simulate_click(at, Modifiers::default());
    assert_eq!((value.get(), calls.get()), (false, 0));
}

#[gpui::test]
fn a_radio_group_reports_the_radio_clicked(cx: &mut TestAppContext) {
    let picked = Rc::new(Cell::new(None::<usize>));
    let p = picked.clone();
    let cx = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(move |_, _| {
            let radio = |ix: usize| {
                let p = p.clone();
                Radio::new(("radio", ix))
                    .label("R")
                    .checked(p.get() == Some(ix))
                    .on_change(move |_, _, _| p.set(Some(ix)))
            };
            RadioGroup::new("g")
                .children([radio(0), radio(1)])
                .into_any_element()
        }),
    );
    // Both indicators share a selector; the one drawn last is the second.
    let second = centre(bounds(cx, "native-checkbox-indicator"));
    cx.simulate_click(second, Modifiers::default());
    assert_eq!(picked.get(), Some(1));
}

/// M6-23: the caller's style is the group's own, so a layout style such as
/// `w_full()` reaches the element its parent lays out.
#[gpui::test]
fn a_radio_groups_style_reaches_its_root(cx: &mut TestAppContext) {
    let cx = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(|_, _| {
            div()
                .flex()
                .flex_row()
                .w(px(200.))
                .child(
                    RadioGroup::new("g")
                        .w_full()
                        .children([Radio::new("r").label("R")]),
                )
                .into_any_element()
        }),
    );
    assert_eq!(bounds(cx, "native-radio-group").size.width, px(200.));
}

/// What gpui writes into the accessibility node of each control `probe`
/// builds while a frame is laid out under kde-breeze, as gpui-base's own
/// tests read a control's node (checkbox.rs,
/// `accessibility_exposes_role_label_and_all_toggle_states`): the name each
/// announces.
fn announced(
    cx: &mut TestAppContext,
    probe: fn(&mut Window, &mut gpui::App) -> Vec<accesskit::Node>,
) -> Vec<Option<String>> {
    let captured = Rc::new(RefCell::new(Vec::new()));
    let c = captured.clone();
    window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(move |_, _| {
            let c = c.clone();
            gpui::canvas(
                move |_, window, cx| *c.borrow_mut() = probe(window, cx),
                |_, (), _, _| {},
            )
            .size_full()
            .into_any_element()
        }),
    );
    captured
        .take()
        .iter()
        .map(|node| node.label().map(str::to_owned))
        .collect()
}

/// The node `base`, a control's headless primitive, writes; an empty one
/// where the control is not theme-drawn (`None`).
fn a11y_node(
    role: accesskit::Role,
    base: Option<impl gpui::RenderOnce>,
    window: &mut Window,
    cx: &mut gpui::App,
) -> accesskit::Node {
    let mut node = accesskit::Node::new(role);
    if let Some(base) = base {
        base.render(window, cx)
            .into_element()
            .write_a11y_info(&mut node);
    }
    node
}

/// The names a probe's three controls announce: the first labelled and
/// named apart, the second labelled only, the third named only.
fn three_names() -> Vec<Option<String>> {
    vec![
        Some("Announced".to_owned()),
        Some("Shown".to_owned()),
        Some("Only announced".to_owned()),
    ]
}

/// M6-27: a checkbox announces its `accessibility_label`, or else its label.
#[gpui::test]
fn a_checkbox_announces_its_accessibility_label(cx: &mut TestAppContext) {
    let announced = announced(cx, |window, cx| {
        [
            Checkbox::new("a")
                .label("Shown")
                .accessibility_label("Announced"),
            Checkbox::new("b").label("Shown"),
            Checkbox::new("c").accessibility_label("Only announced"),
        ]
        .map(|checkbox| {
            let base = checkbox.row(window, cx).ok();
            a11y_node(accesskit::Role::CheckBox, base, window, cx)
        })
        .to_vec()
    });
    assert_eq!(announced, three_names());
}

/// M6-27: a radio announces its `accessibility_label`, or else its label.
#[gpui::test]
fn a_radio_announces_its_accessibility_label(cx: &mut TestAppContext) {
    let announced = announced(cx, |window, cx| {
        [
            Radio::new("a")
                .label("Shown")
                .accessibility_label("Announced"),
            Radio::new("b").label("Shown"),
            Radio::new("c").accessibility_label("Only announced"),
        ]
        .map(|radio| {
            let base = radio.row(window, cx).ok();
            a11y_node(accesskit::Role::RadioButton, base, window, cx)
        })
        .to_vec()
    });
    assert_eq!(announced, three_names());
}

/// M6-27: a switch announces its `accessibility_label`, or else its label.
#[gpui::test]
fn a_switch_announces_its_accessibility_label(cx: &mut TestAppContext) {
    let announced = announced(cx, |window, cx| {
        [
            Switch::new("a")
                .label("Shown")
                .accessibility_label("Announced"),
            Switch::new("b").label("Shown"),
            Switch::new("c").accessibility_label("Only announced"),
        ]
        .map(|switch| {
            let base = switch.row(window, cx).ok();
            a11y_node(accesskit::Role::Switch, base, window, cx)
        })
        .to_vec()
    });
    assert_eq!(announced, three_names());
}

/// M6-27: `focus_ring(false)` keeps a focused switch's track free of the
/// ring: with the theme's focus ring on, gpui-component's
/// `focus_ring_style` tints the track's border in `ThemeColor::ring`
/// (styled.rs:257-262), and the track sets no border colour of its own (its
/// outline is a box of its own, under the thumb).
#[gpui::test]
fn a_switch_told_not_to_draws_no_focus_ring(cx: &mut TestAppContext) {
    let cx = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(|_, _| div().into_any_element()),
    );
    let r = resolved("kde-breeze", ColorMode::Light);
    let look = SwitchLook::of(&r, false, false).expect("finite");
    let border = |cx: &mut VisualTestContext, focus_ring: bool| {
        cx.update(|window, cx| {
            let switch = Switch::new("s").focus_ring(focus_ring);
            let mut track = switch.track(&look, true, look.thumb_left(false), window, cx);
            track.style().border_color
        })
    };
    let ring = cx.update(|_, cx| {
        use gpui_component::ActiveTheme as _;
        assert!(cx.theme().focus_ring, "the theme draws focus rings");
        cx.theme().ring
    });
    assert_eq!(border(cx, true), Some(ring));
    assert_eq!(border(cx, false), None);
}

#[derive(Clone, Copy)]
enum Control {
    Checkbox,
    Radio,
    Switch,
}

/// The controls activated, in order, by three rounds of Tab and Space over
/// a column of three `control`s: "late" at Tab index 2, "early" at 1, and
/// "skipped", which is no tab stop.
fn activated_in_tab_order(cx: &mut TestAppContext, control: Control) -> Vec<&'static str> {
    let fired = Rc::new(RefCell::new(Vec::new()));
    let f = fired.clone();
    let cx = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(move |_, _| {
            let one = |id: &'static str, index: isize, stop: bool| {
                let f = f.clone();
                let on_change =
                    move |_: &bool, _: &mut Window, _: &mut gpui::App| f.borrow_mut().push(id);
                match control {
                    Control::Checkbox => Checkbox::new(id)
                        .label(id)
                        .tab_index(index)
                        .tab_stop(stop)
                        .on_change(on_change)
                        .into_any_element(),
                    Control::Radio => Radio::new(id)
                        .label(id)
                        .tab_index(index)
                        .tab_stop(stop)
                        .on_change(on_change)
                        .into_any_element(),
                    Control::Switch => Switch::new(id)
                        .label(id)
                        .tab_index(index)
                        .tab_stop(stop)
                        .on_change(on_change)
                        .into_any_element(),
                }
            };
            div()
                .flex()
                .flex_col()
                .child(one("late", 2, true))
                .child(one("early", 1, true))
                .child(one("skipped", 0, false))
                .into_any_element()
        }),
    );
    for _ in 0..3 {
        focus_next(cx);
        activate_key(cx, "space");
        redraw(cx);
    }
    fired.take()
}

/// M6-27: `tab_index` orders a control in its tab group, and
/// `tab_stop(false)` takes it out of the Tab order.
#[gpui::test]
fn a_checkbox_keeps_its_tab_order(cx: &mut TestAppContext) {
    assert_eq!(
        activated_in_tab_order(cx, Control::Checkbox),
        ["early", "late", "early"]
    );
}

#[gpui::test]
fn a_radio_keeps_its_tab_order(cx: &mut TestAppContext) {
    assert_eq!(
        activated_in_tab_order(cx, Control::Radio),
        ["early", "late", "early"]
    );
}

#[gpui::test]
fn a_switch_keeps_its_tab_order(cx: &mut TestAppContext) {
    assert_eq!(
        activated_in_tab_order(cx, Control::Switch),
        ["early", "late", "early"]
    );
}

/// M6-27: the caller's style refines the row after the widget's own, so
/// `w_full()` reaches it and a gap of the caller's replaces `label_gap`.
#[gpui::test]
fn the_callers_style_reaches_the_row(cx: &mut TestAppContext) {
    let cx = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(|_, _| {
            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .w(px(200.))
                        .child(Checkbox::new("c").label("C").w_full().gap(px(37.))),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .w(px(200.))
                        .child(Switch::new("s").label("S").w_full()),
                )
                .into_any_element()
        }),
    );
    assert_eq!(bounds(cx, "native-checkbox-row").size.width, px(200.));
    assert_eq!(bounds(cx, "native-switch-row").size.width, px(200.));
    let indicator = bounds(cx, "native-checkbox-indicator");
    let label = bounds(cx, "native-checkbox-label");
    assert_eq!(label.origin.x - indicator.right(), px(37.));
}

fn slider_window(
    cx: &mut TestAppContext,
    disabled: bool,
) -> (&mut VisualTestContext, Entity<SliderState>) {
    let state = cx.new(|_| {
        SliderState::new()
            .min(0.)
            .max(100.)
            .step(1.)
            .default_value(50.)
    });
    let s = state.clone();
    let cx = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(move |_, _| {
            Slider::new(&s)
                .disabled(disabled)
                .w(px(140.))
                .into_any_element()
        }),
    );
    (cx, state)
}

fn slider_value(cx: &mut VisualTestContext, state: &Entity<SliderState>) -> f32 {
    cx.update(|_, cx| state.read(cx).value().end())
}

#[gpui::test]
fn a_slider_steps_from_the_keyboard(cx: &mut TestAppContext) {
    let (cx, state) = slider_window(cx, false);
    cx.update(|window, cx| window.focus_next(cx));
    cx.update(|window, cx| assert!(window.focused(cx).is_some(), "the slider takes focus"));
    cx.simulate_keystrokes("right");
    assert_eq!(slider_value(cx, &state), 51.);
    cx.simulate_keystrokes("left left");
    assert_eq!(slider_value(cx, &state), 49.);
    cx.simulate_keystrokes("end");
    assert_eq!(slider_value(cx, &state), 100.);
    cx.simulate_keystrokes("up");
    assert_eq!(slider_value(cx, &state), 100., "no step past the end");
    cx.simulate_keystrokes("home");
    assert_eq!(slider_value(cx, &state), 0.);
}

#[gpui::test]
fn a_slider_follows_the_pointer_over_its_travel(cx: &mut TestAppContext) {
    let (cx, state) = slider_window(cx, false);
    let root = bounds(cx, "native-slider");
    let r = resolved("kde-breeze", ColorMode::Light);
    let radius = px(r.slider.thumb_diameter) / 2.;
    // A press at the travel's start is the minimum, one at its end the
    // maximum: the thumb's centre travels the width less a radius each end.
    let y = centre(root).y;
    cx.simulate_click(point(root.origin.x + radius, y), Modifiers::default());
    assert_eq!(slider_value(cx, &state), 0.);
    redraw(cx);
    cx.simulate_click(
        point(root.origin.x + root.size.width - radius, y),
        Modifiers::default(),
    );
    assert_eq!(slider_value(cx, &state), 100.);
}

fn range_window(cx: &mut TestAppContext) -> (&mut VisualTestContext, Entity<SliderState>) {
    let state = cx.new(|_| {
        SliderState::new()
            .min(0.)
            .max(100.)
            .step(1.)
            .default_value(20.0..80.0)
    });
    let s = state.clone();
    let cx = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(move |_, _| Slider::new(&s).w(px(140.)).into_any_element()),
    );
    (cx, state)
}

/// M6-20: a range state draws a thumb at each end, the fill between them,
/// as gpui-component's slider draws one (slider.rs:149-156, :305-308).
#[gpui::test]
fn a_range_slider_draws_a_thumb_at_each_end(cx: &mut TestAppContext) {
    let (cx, state) = range_window(cx);
    let r = resolved("kde-breeze", ColorMode::Light);
    let radius = px(r.slider.thumb_diameter) / 2.;
    let root = bounds(cx, "native-slider");
    // The travel the thumbs' centres cover: the width less a radius at each
    // end.
    let travel = root.size.width - radius * 2.;
    let at = |fraction: f32| root.origin.x + radius + travel * fraction;
    let start = bounds(cx, "native-slider-start-thumb");
    let end = bounds(cx, "native-slider-thumb");
    assert_eq!(centre(start).x, at(0.2), "the start thumb at 20");
    assert_eq!(centre(end).x, at(0.8), "the end thumb at 80");
    let fill = bounds(cx, "native-slider-fill");
    assert_eq!(fill.origin.x, at(0.2), "the fill from the start thumb");
    assert_eq!(fill.origin.x + fill.size.width, at(0.8), "to the end thumb");
    drop(state);
}

/// M6-20: the start of a range moves by pointer: a press on the rail nearer
/// the start moves the start, as gpui-base picks the nearer thumb
/// (slider.rs:677-689), and the start thumb follows; dragging that thumb
/// moves the start, not the end.
#[gpui::test]
fn a_range_slider_moves_its_start_by_pointer(cx: &mut TestAppContext) {
    use gpui_base::slider::SliderValue;
    let (cx, state) = range_window(cx);
    let r = resolved("kde-breeze", ColorMode::Light);
    let radius = px(r.slider.thumb_diameter) / 2.;
    let root = bounds(cx, "native-slider");
    let travel = root.size.width - radius * 2.;
    let at = |fraction: f32| root.origin.x + radius + travel * fraction;
    let y = centre(root).y;
    let value = |cx: &mut VisualTestContext| cx.update(|_, cx| state.read(cx).value());
    cx.simulate_click(point(at(0.05), y), Modifiers::default());
    assert_eq!(value(cx), SliderValue::Range(5., 80.), "the start moved");
    redraw(cx);
    let start = bounds(cx, "native-slider-start-thumb");
    assert_eq!(centre(start).x, at(0.05), "the start thumb follows");
    cx.simulate_mouse_down(centre(start), gpui::MouseButton::Left, Modifiers::default());
    // The first move starts the drag past gpui's threshold, the next moves
    // the dragged thumb.
    for x in [at(0.2), at(0.4)] {
        cx.simulate_mouse_move(
            point(x, y),
            Some(gpui::MouseButton::Left),
            Modifiers::default(),
        );
    }
    cx.simulate_mouse_up(
        point(at(0.4), y),
        gpui::MouseButton::Left,
        Modifiers::default(),
    );
    assert_eq!(
        value(cx),
        SliderValue::Range(40., 80.),
        "dragged, the start"
    );
}

/// M6-20: the keys move a range's end and keep its start, where a key turned
/// `Range(20, 80)` into `Single(81)`; the end never passes the start, as a
/// pointer keeps it (gpui-base slider.rs:378-382).
#[gpui::test]
fn a_range_slider_keeps_its_range_on_the_keys(cx: &mut TestAppContext) {
    use gpui_base::slider::SliderValue;
    let (cx, state) = range_window(cx);
    focus_next(cx);
    let value = |cx: &mut VisualTestContext| cx.update(|_, cx| state.read(cx).value());
    cx.simulate_keystrokes("right");
    assert_eq!(value(cx), SliderValue::Range(20., 81.));
    cx.simulate_keystrokes("left left");
    assert_eq!(value(cx), SliderValue::Range(20., 79.));
    cx.simulate_keystrokes("end");
    assert_eq!(value(cx), SliderValue::Range(20., 100.));
    cx.simulate_keystrokes("home");
    assert_eq!(value(cx), SliderValue::Range(20., 20.), "down to the start");
}

/// M6-20: along the vertical axis the rail stands up the widget, its minimum
/// at the bottom, as gpui-base maps the pointer (slider.rs:367-371), and the
/// thumb travels it.
#[gpui::test]
fn a_vertical_slider_lays_its_rail_out_vertically(cx: &mut TestAppContext) {
    let state = cx.new(|_| {
        SliderState::new()
            .min(0.)
            .max(100.)
            .step(1.)
            .default_value(25.)
    });
    let s = state.clone();
    let cx = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(move |_, _| {
            Slider::new(&s)
                .axis(gpui::Axis::Vertical)
                .h(px(140.))
                .into_any_element()
        }),
    );
    let r = resolved("kde-breeze", ColorMode::Light);
    let sl = &r.slider;
    let radius = px(sl.thumb_diameter) / 2.;
    let root = bounds(cx, "native-slider");
    assert_eq!(root.size.height, px(140.));
    assert_eq!(root.size.width, px(sl.thumb_diameter.max(sl.track_height)));
    let rail = bounds(cx, "native-slider-rail");
    assert_eq!(rail.size.width, px(sl.track_height), "the rail's thickness");
    assert_eq!(
        rail.size.height, root.size.height,
        "the rail spans the widget"
    );
    let thumb = bounds(cx, "native-slider-thumb");
    assert_eq!(thumb.size.width, px(sl.thumb_diameter));
    assert_eq!(centre(thumb).x, centre(rail).x, "centred on the rail");
    let travel = root.size.height - radius * 2.;
    assert_eq!(
        centre(thumb).y,
        root.origin.y + root.size.height - radius - travel * 0.25,
        "at 25 a quarter up from the bottom"
    );
    let fill = bounds(cx, "native-slider-fill");
    assert_eq!(
        fill.origin.y + fill.size.height,
        root.origin.y + root.size.height
    );
    assert_eq!(fill.origin.y, centre(thumb).y, "filled up to the thumb");
    // A press at the top of the travel is the maximum.
    cx.simulate_click(
        point(centre(root).x, root.origin.y + radius),
        Modifiers::default(),
    );
    assert_eq!(slider_value(cx, &state), 100.);
    focus_next(cx);
    cx.simulate_keystrokes("down");
    assert_eq!(slider_value(cx, &state), 99.);
}

/// The solid fill of the quad painted at `b`, given in window pixels.
fn painted_fill(cx: &mut VisualTestContext, b: Bounds<Pixels>) -> Option<gpui::Hsla> {
    cx.update(|window, _| {
        let b = b.scale(window.scale_factor());
        window
            .painted_quads()
            .iter()
            .find(|q| q.bounds == b && !q.background.is_transparent())
            .and_then(|q| q.background.as_solid())
    })
}

/// M6-19: a disabled adwaita slider is drawn faded as one widget over the
/// window, as libadwaita's whole-widget `filter: Opacity(..)` fades it
/// (docs/platform-facts.md §2.1.6): its fill is the fill over the rail over
/// the window, faded by `disabled_opacity` over the window, and opaque, where
/// fading each quad on its own let the rail show through the fill.
#[gpui::test]
fn a_disabled_slider_is_painted_faded_as_one(cx: &mut TestAppContext) {
    let state = cx.new(|_| SliderState::new().min(0.).max(100.).default_value(50.));
    let s = state.clone();
    let cx = window_with(
        cx,
        Some(("adwaita", ColorMode::Light)),
        Box::new(move |_, _| {
            Slider::new(&s)
                .disabled(true)
                .w(px(140.))
                .into_any_element()
        }),
    );
    let r = resolved("adwaita", ColorMode::Light);
    let sl = &r.slider;
    let a = sl.disabled_opacity;
    assert!(a < 1., "adwaita dims a disabled slider by opacity");
    let ground = c(r.defaults.background_color);
    let track = c(sl.disabled_track_color.unwrap_or(sl.track_color));
    let fill = c(sl.disabled_fill_color.unwrap_or(sl.fill_color));
    let rail = over(ground, track);
    let want_fill = faded(ground, rail, fill, a);
    assert_eq!(want_fill.a, 1., "the fill is opaque over an opaque window");
    let (fill_at, rail_at) = (
        bounds(cx, "native-slider-fill"),
        bounds(cx, "native-slider-rail"),
    );
    assert_eq!(
        painted_fill(cx, fill_at),
        Some(want_fill),
        "the fill over the rail, faded over the window"
    );
    assert_eq!(
        painted_fill(cx, rail_at),
        Some(faded(ground, ground, track, a)),
        "the rail faded over the window"
    );
    drop(state);
}

#[gpui::test]
fn a_disabled_slider_is_inert(cx: &mut TestAppContext) {
    let (cx, state) = slider_window(cx, true);
    let root = bounds(cx, "native-slider");
    cx.simulate_click(root.origin + point(px(4.), px(4.)), Modifiers::default());
    cx.update(|window, cx| window.focus_next(cx));
    cx.simulate_keystrokes("right");
    assert_eq!(slider_value(cx, &state), 50.);
}
