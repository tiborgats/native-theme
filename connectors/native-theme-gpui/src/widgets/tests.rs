//! Widget tests (docs/archive/todo_gpui-widgets-spec.md §4): every leaf a widget
//! paints is the one the spec names, the drawn boxes measure the stated
//! sizes, and the controls behave as gpui-component's.

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

/// The painted scene, read as the showcase's tests read it. These read no
/// underline.
#[allow(dead_code)]
#[path = "scene_read.rs"]
mod scene_read;
use scene_read::scene_quads;

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
        assert_eq!(idle.hover_border, k.hover_border_color.map(c), "{preset}");

        let checked = at(true, false);
        assert_eq!(checked.fill, c(k.checked_background), "{preset}");
        assert_eq!(checked.border, c(k.border.color), "{preset}");
        assert_eq!(
            checked.hover_fill, None,
            "{preset}: no stated checked hover"
        );
        assert_eq!(
            checked.hover_border,
            k.hover_border_color.map(c),
            "{preset}: the hover border frames a checked box too"
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
        assert_eq!(disabled.hover_border, None, "{preset}");
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

/// A theme that states no `checkbox.hover_border_color` gives the look no
/// hover border, so the indicator keeps its border at rest under the
/// pointer; kde-breeze states one, which is cleared here.
#[test]
fn an_unset_hover_border_keeps_the_border() {
    let mut r = resolved("kde-breeze", ColorMode::Light);
    assert!(
        r.checkbox.hover_border_color.is_some(),
        "kde-breeze states checkbox.hover_border_color, so clearing it proves something"
    );
    r.checkbox.hover_border_color = None;
    for look in [
        CheckboxLook::of(&r, false, false).expect("finite"),
        CheckboxLook::radio(&r, false, false).expect("finite"),
    ] {
        assert_eq!(look.hover_border, None);
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

/// An icon resolver that records every path it is asked for and keeps it.
fn recording_resolver(cx: &mut VisualTestContext) -> Rc<RefCell<Vec<String>>> {
    let record = Rc::new(RefCell::new(Vec::new()));
    let sink = record.clone();
    cx.update(|_, cx| {
        gpui_component::set_icon_resolver(cx, move |path, _| {
            sink.borrow_mut().push(path.to_string());
            None
        });
    });
    record
}

/// Where the theme states no check-mark stroke, the connector draws
/// gpui-component's check glyph by its asset path, and asks the
/// application's icon resolver for it first, as gpui-component's check box
/// does (gpui-component checkbox.rs:220-222). A stroked mark asks nothing.
#[gpui::test]
fn the_check_mark_asks_the_applications_icon_resolver(cx: &mut TestAppContext) {
    use gpui_component::{IconName, IconNamed as _};
    let look = CheckboxLook::of(&resolved("macos-monterey", ColorMode::Light), true, false);
    assert!(look.is_some(), "macos-monterey draws the connector's box");
    assert_eq!(look.and_then(|l| l.mark_stroke), None);
    let check = IconName::Check.path().to_string();

    let window = window_with(
        cx,
        Some(("macos-monterey", ColorMode::Light)),
        Box::new(|_, _| {
            Checkbox::new("c")
                .label("C")
                .checked(true)
                .into_any_element()
        }),
    );
    assert!(
        window.debug_bounds("native-checkbox-indicator").is_some(),
        "the connector's box, not gpui-component's"
    );
    let record = recording_resolver(window);
    redraw(window);
    assert!(
        record.borrow().contains(&check),
        "the resolver was asked for {check}: {:?}",
        record.borrow()
    );

    record.borrow_mut().clear();
    let window = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(|_, _| {
            Checkbox::new("c")
                .label("C")
                .checked(true)
                .into_any_element()
        }),
    );
    assert!(window.debug_bounds("native-checkbox-indicator").is_some());
    assert!(
        window.debug_bounds("native-checkbox-mark").is_some(),
        "kde-breeze's mark is the stroked check"
    );
    assert!(
        !record.borrow().contains(&check),
        "kde-breeze's stroked mark asks nothing: {:?}",
        record.borrow()
    );
}

/// A radio whose theme states neither a dot nor a check-mark stroke (nord)
/// draws the same check glyph, and asks the resolver for it too.
#[gpui::test]
fn the_radio_mark_asks_the_applications_icon_resolver(cx: &mut TestAppContext) {
    use gpui_component::{IconName, IconNamed as _};
    let look = CheckboxLook::radio(&resolved("nord", ColorMode::Light), true, false);
    assert!(look.is_some(), "nord draws the connector's radio");
    assert_eq!(look.and_then(|l| l.dot), None);
    assert_eq!(look.and_then(|l| l.mark_stroke), None);
    let check = IconName::Check.path().to_string();

    let window = window_with(
        cx,
        Some(("nord", ColorMode::Light)),
        Box::new(|_, _| Radio::new("r").label("R").checked(true).into_any_element()),
    );
    assert!(window.debug_bounds("native-checkbox-indicator").is_some());
    assert!(window.debug_bounds("native-radio-dot").is_none());
    let record = recording_resolver(window);
    redraw(window);
    assert!(
        record.borrow().contains(&check),
        "the resolver was asked for {check}: {:?}",
        record.borrow()
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
            let quads = scene_quads(window);
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

// ---------------------------------------------------------------------------
// The selected tab's outward corners: tab.active_outward_corner_radius
// ---------------------------------------------------------------------------

/// The light `windows-11` theme, resolved, with
/// `tab.active_outward_corner_radius` set on it to `radius` and `edit`
/// applied: the preset states 4 (G3 T15), and the tests set the radius on
/// the resolved theme, whose fields are public, to try it unstated and at
/// other values too.
fn windows_11(radius: Option<f32>, edit: impl FnOnce(&mut ResolvedTheme)) -> ResolvedTheme {
    let prefs = AccessibilityPreferences::default();
    let (_, mut r) = crate::from_preset("windows-11", false, &prefs).expect("windows-11 loads");
    r.tab.active_outward_corner_radius = radius;
    edit(&mut r);
    r
}

/// Each part's bounds, as a bar hands them to its observer, in order.
type Reported = Rc<RefCell<Vec<(Part, Bounds<Pixels>)>>>;

/// A window drawing three tabs, `two` selected, in a 320px bar under
/// [`windows_11`]'s theme, installed as `crate::apply` installs one; the bar
/// hands `parts` each tab's bounds.
fn folder_window(
    cx: &mut TestAppContext,
    radius: Option<f32>,
    edit: impl FnOnce(&mut ResolvedTheme),
    parts: Reported,
) -> (&mut VisualTestContext, ResolvedTheme) {
    folder_window_counted(cx, radius, edit, parts, Rc::new(Cell::new(0)))
}

/// [`folder_window`], counting in `renders` each time the window's view
/// renders the bar.
fn folder_window_counted(
    cx: &mut TestAppContext,
    radius: Option<f32>,
    edit: impl FnOnce(&mut ResolvedTheme),
    parts: Reported,
    renders: Rc<Cell<usize>>,
) -> (&mut VisualTestContext, ResolvedTheme) {
    let r = windows_11(radius, edit);
    cx.update(|cx| {
        gpui_component::init(cx);
        let prefs = AccessibilityPreferences::default();
        let (theme, _) = crate::from_preset("windows-11", false, &prefs).expect("windows-11 loads");
        crate::apply(theme, &r, &prefs, cx);
    });
    let build: Build = Box::new(move |_, _| {
        renders.set(renders.get() + 1);
        let parts = parts.clone();
        TabBar::new("t")
            .w(px(320.))
            .child(Tab::new("One").debug_selector(|| "one".into()))
            .child(Tab::new("Two").debug_selector(|| "two".into()))
            .child(Tab::new("Three").debug_selector(|| "three".into()))
            .selected_index(1)
            .on_part_bounds(Rc::new(move |part, b, _, _| {
                parts.borrow_mut().push((part, b))
            }))
            .into_any_element()
    });
    let (_, cx) = cx.add_window_view(|_, _| Harness { build });
    cx.update(|window, cx| window.draw(cx).clear(cx));
    (cx, r)
}

fn near(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-4
}

/// docs/platform-facts.md §2.11 `active_outward_corner_radius`: windows-11
/// with a radius of 4 and its outline of 1 gives the numbers every
/// connector's test shares -- the left curve about `(x0 - 3, y1 - 4)` in the
/// square `[x0 - 3, x0 + 1] x [y1 - 4, y1]`, the right one about
/// `(x1 + 3, y1 - 4)` in `[x1 - 1, x1 + 3] x [y1 - 4, y1]`; in each, the
/// fill an annulus from 3 to `4 * sqrt(2) + 1` in `tab.active_background`
/// and the outline one from 3 to 4 in `tab.border.color`, painted over the
/// fill's inner band -- with the top corners at 8. Without an outline the
/// fill begins at 4.
#[test]
fn the_flares_are_the_shapes_geometry() {
    let r = windows_11(Some(4.), |_| {});
    assert_eq!(r.tab.border.line_width, 1., "windows-11's outline");
    let look = TabLook::of(&r).expect("finite");
    assert_eq!(look.outward_radius, Some(px(4.)));
    assert_eq!(look.radius, px(8.), "the top corners");
    assert_eq!(look.active, c(r.tab.active_background));
    assert_eq!(look.border, c(r.tab.border.color));
    let (x0, x1, y1) = (20., 120., 42.);
    let tab = Bounds::new(point(px(x0), px(10.)), gpui::size(px(x1 - x0), px(32.)));
    let far = 4. * std::f32::consts::SQRT_2 + 1.;
    // (square's left edge, centre's x, outer radius, width, colour)
    let expected = [
        (x0 - 3., x0 - 3., far, far - 3., look.active),
        (x1 - 1., x1 + 3., far, far - 3., look.active),
        (x0 - 3., x0 - 3., 4., 1., look.border),
        (x1 - 1., x1 + 3., 4., 1., look.border),
    ];
    let quads = tab_bar::flare_quads(tab, &look);
    assert_eq!(quads.len(), expected.len(), "{quads:?}");
    for (q, (left, centre, outer, width, colour)) in quads.iter().zip(expected) {
        let at = |b: Bounds<Pixels>| {
            [
                f32::from(b.left()),
                f32::from(b.top()),
                f32::from(b.right()),
                f32::from(b.bottom()),
            ]
        };
        let square = [left, y1 - 4., left + 4., y1];
        let ring = [
            centre - outer,
            y1 - 4. - outer,
            centre + outer,
            y1 - 4. + outer,
        ];
        assert!(
            at(q.clip).iter().zip(square).all(|(a, b)| near(*a, b)),
            "{q:?}: the square {square:?}"
        );
        assert!(
            at(q.bounds).iter().zip(ring).all(|(a, b)| near(*a, b)),
            "{q:?}: the ring {ring:?}"
        );
        assert!(near(f32::from(q.radius), outer), "{q:?}");
        assert!(near(f32::from(q.width), width), "{q:?}");
        assert_eq!(q.color, colour, "{q:?}");
    }
    // No outline: the fills alone.
    let thin =
        TabLook::of(&windows_11(Some(4.), |r| r.tab.border.line_width = 0.)).expect("finite");
    let quads = tab_bar::flare_quads(tab, &thin);
    assert_eq!(quads.len(), 2, "{quads:?}");
    assert!(quads.iter().all(|q| q.color == thin.active));
    assert!(
        quads.iter().all(|q| near(f32::from(q.width), far - 4.)),
        "without an outline the fill begins at the radius: {quads:?}"
    );
    // Unstated or 0: today's shape, no flare.
    for radius in [None, Some(0.)] {
        let look = TabLook::of(&windows_11(radius, |_| {})).expect("finite");
        assert_eq!(look.outward_radius, None, "{radius:?}");
        assert!(tab_bar::flare_quads(tab, &look).is_empty(), "{radius:?}");
    }
    // Not finite: no look, so the bar is gpui-component's.
    for radius in [f32::NAN, f32::INFINITY, -1.] {
        assert!(
            TabLook::of(&windows_11(Some(radius), |_| {})).is_none(),
            "{radius}"
        );
    }
}

/// The quads the last frame painted in `flare`'s colour clipped to its
/// square, which only the flare's own two quads are clipped to.
fn painted_in(cx: &mut VisualTestContext, flare: &tab_bar::FlareQuad) -> Vec<gpui::Quad> {
    cx.update(|window, _| {
        let clip = flare.clip.scale(window.scale_factor());
        scene_quads(window)
            .into_iter()
            .filter(|q| q.content_mask.bounds == clip && q.border_color == flare.color)
            .collect()
    })
}

/// Each of `flares` painted once, as `flare_quads` gives it: an annulus
/// drawn as a transparent quad's border in its colour, about its centre,
/// clipped to its square.
fn assert_flares_painted(cx: &mut VisualTestContext, flares: &[tab_bar::FlareQuad], when: &str) {
    assert_eq!(
        flares.len(),
        4,
        "{when}: two flares, each a fill and an outline"
    );
    let s = cx.update(|window, _| window.scale_factor());
    for flare in flares {
        let painted = painted_in(cx, flare);
        assert_eq!(painted.len(), 1, "{when}: {flare:?} painted as {painted:?}");
        for q in painted {
            let centre = |b: Bounds<gpui::ScaledPixels>| {
                (
                    b.origin.x.0 + b.size.width.0 / 2.,
                    b.origin.y.0 + b.size.height.0 / 2.,
                )
            };
            let (want, got) = (centre(flare.bounds.scale(s)), centre(q.bounds));
            assert!(
                (want.0 - got.0).abs() <= 0.5 && (want.1 - got.1).abs() <= 0.5,
                "{when}: {flare:?} centred at {got:?}, not {want:?}"
            );
            assert!(q.background.is_transparent(), "{when}: {q:?}");
            assert_eq!(q.border_color, flare.color, "{when}: {q:?}");
            assert_eq!(
                q.corner_radii.top_left.0,
                f32::from(flare.radius) * s,
                "{when}"
            );
            let e = q.border_widths;
            let width = f32::from(flare.width) * s;
            assert!(
                [e.top.0, e.right.0, e.bottom.0, e.left.0]
                    .iter()
                    .all(|b| (b - width).abs() <= 0.5),
                "{when}: {q:?} is not {width} wide"
            );
        }
    }
}

/// The selected tab's outline quads, found by its bounds and colour: its
/// border widths, top, right, bottom and left, in scaled pixels.
fn selected_outline(
    cx: &mut VisualTestContext,
    tab: Bounds<Pixels>,
    look: &TabLook,
) -> Vec<[f32; 4]> {
    cx.update(|window, _| {
        let b = tab.scale(window.scale_factor());
        scene_quads(window)
            .into_iter()
            .filter(|q| q.bounds == b && q.border_color == look.border)
            .map(|q| {
                let e = q.border_widths;
                [e.top.0, e.right.0, e.bottom.0, e.left.0]
            })
            .collect()
    })
}

/// Under windows-11 with a radius of 4: the selected tab's two flares are
/// painted, each clipped to its square inside the row; the selected tab's
/// outline has no bottom edge and its other sides are `w`; the first tab
/// starts `r` inside the row, and the bar reports the selected tab's bounds
/// to its bottom edge, which has no border.
#[gpui::test]
fn the_selected_tab_curves_out_at_its_feet(cx: &mut TestAppContext) {
    let parts = Rc::new(RefCell::new(Vec::new()));
    let (cx, r) = folder_window(cx, Some(4.), |_| {}, parts.clone());
    let look = TabLook::of(&r).expect("finite");
    let (bar, one, two) = (
        bounds(cx, "native-tab-bar"),
        bounds(cx, "one"),
        bounds(cx, "two"),
    );
    assert_eq!(
        one.left() - bar.left(),
        px(4.),
        "the room at the row's start"
    );
    let flares = tab_bar::flare_quads(two, &look);
    assert_eq!(flares.len(), 4);
    assert!(
        flares.iter().all(|f| bar.contains(&f.clip.origin)
            && bar.contains(&point(f.clip.right() - px(0.5), f.clip.bottom() - px(0.5)))),
        "the squares lie inside the row"
    );
    assert_flares_painted(cx, &flares, "the first frame");
    let s = cx.update(|window, _| window.scale_factor());
    let w = f32::from(look.border_width) * s;
    let outline = selected_outline(cx, two, &look);
    assert!(!outline.is_empty(), "the selected tab's outline is painted");
    assert!(
        outline.iter().all(|e| *e == [w, w, 0., w]),
        "the outline has no bottom edge: {outline:?}"
    );
    assert_eq!(
        parts
            .borrow()
            .iter()
            .rev()
            .find(|(part, _)| *part == Part::Tab(1))
            .map(|(_, b)| *b),
        Some(two),
        "Part::Tab reaches the selected tab's bottom, which has no border"
    );
}

/// On gpui-pre, whose `painted_quads()` is the scene in paint order: every
/// quad of a tab, its indicator's included, comes before the flares' first.
#[cfg(not(feature = "gpui-fast"))]
#[gpui::test]
fn the_flares_are_painted_after_every_tab(cx: &mut TestAppContext) {
    let (cx, r) = folder_window(
        cx,
        Some(4.),
        |r| {
            r.tab.active_indicator_color = Some(r.defaults.accent_color);
            r.tab.active_indicator_width = Some(2.);
            r.tab.active_indicator_side = Some(TabIndicatorSide::Bottom);
        },
        Rc::new(RefCell::new(Vec::new())),
    );
    let look = TabLook::of(&r).expect("finite");
    let tabs = ["one", "two", "three", "native-tab-indicator"].map(|name| bounds(cx, name));
    let flares = tab_bar::flare_quads(tabs[1], &look);
    let order = cx.update(|window, _| {
        let s = window.scale_factor();
        let quads = window.painted_quads();
        let tabs = tabs.map(|b| b.scale(s));
        let clips: Vec<_> = flares.iter().map(|f| f.clip.scale(s)).collect();
        (
            quads.iter().rposition(|q| tabs.contains(&q.bounds)),
            quads
                .iter()
                .position(|q| clips.contains(&q.content_mask.bounds)),
        )
    });
    match order {
        (Some(last_tab), Some(first_flare)) => assert!(
            last_tab < first_flare,
            "a tab's quad at {last_tab} after a flare's at {first_flare}"
        ),
        _ => panic!("tabs and flares are not both painted: {order:?}"),
    }
}

/// A frame drawn after the first with nothing changed, without
/// `window.refresh()`, still paints the flares: GPUI Fast's reuse of an
/// unchanged view keeps them. On GPUI Fast the view does not render again
/// for that frame, so the frame is the reuse.
#[gpui::test]
fn the_flares_outlast_a_frame_with_nothing_changed(cx: &mut TestAppContext) {
    let renders = Rc::new(Cell::new(0));
    let (cx, r) = folder_window_counted(
        cx,
        Some(4.),
        |_| {},
        Rc::new(RefCell::new(Vec::new())),
        renders.clone(),
    );
    let look = TabLook::of(&r).expect("finite");
    let flares = tab_bar::flare_quads(bounds(cx, "two"), &look);
    assert_flares_painted(cx, &flares, "the first frame");
    let before = renders.get();
    assert!(before > 0, "the view rendered the bar");
    cx.update(|w, cx| w.draw(cx).clear(cx));
    #[cfg(feature = "gpui-fast")]
    assert_eq!(
        renders.get(),
        before,
        "the second frame rendered the view again, so it was no reuse"
    );
    assert_flares_painted(cx, &flares, "a second frame with nothing changed");
}

/// The selected tab's label does not move when the tab curves out: with
/// `tab.min_height` 0, so the label's line box and padding set the tab's
/// height, the selected tab is as tall with the radius stated as without,
/// its missing bottom edge made up in its bottom padding.
#[gpui::test]
fn a_tab_that_curves_out_keeps_its_labels_place(cx: &mut TestAppContext) {
    let height = |cx: &mut TestAppContext, radius| {
        let (cx, _) = folder_window(
            cx,
            radius,
            |r| r.tab.min_height = 0.,
            Rc::new(RefCell::new(Vec::new())),
        );
        bounds(cx, "two").size.height
    };
    let square = height(cx, None);
    let curved = height(cx, Some(4.));
    assert_eq!(curved, square, "the selected tab's height moved");
}

/// The quads the last frame painted in `flare`'s colour about its centre,
/// with its radius, found without its mask, which gpui widens to whole
/// device pixels at a fractional scale.
fn painted_about(cx: &mut VisualTestContext, flare: &tab_bar::FlareQuad) -> Vec<gpui::Quad> {
    cx.update(|window, _| {
        let s = window.scale_factor();
        let b = flare.bounds.scale(s);
        let want = (
            b.origin.x.0 + b.size.width.0 / 2.,
            b.origin.y.0 + b.size.height.0 / 2.,
        );
        scene_quads(window)
            .into_iter()
            .filter(|q| {
                let got = (
                    q.bounds.origin.x.0 + q.bounds.size.width.0 / 2.,
                    q.bounds.origin.y.0 + q.bounds.size.height.0 / 2.,
                );
                q.border_color == flare.color
                    && q.background.is_transparent()
                    && (q.corner_radii.top_left.0 - f32::from(flare.radius) * s).abs() < 1e-3
                    && (got.0 - want.0).abs() <= 1.
                    && (got.1 - want.1).abs() <= 1.
            })
            .collect()
    })
}

/// The signed distance of the device point `p` from the border of `q`, a
/// quad of one corner radius and one border width all round, as gpui's quad
/// shader computes it (gpui-pre-wgpu 0.3.12 `src/shaders.wgsl`, `fs_quad`,
/// `quad_sdf_impl`): negative inside the border, so a pixel whose centre
/// lies at or below 0 is at least half painted in the border's colour.
fn border_sdf(q: &gpui::Quad, p: (f32, f32)) -> f32 {
    let half = (q.bounds.size.width.0 / 2., q.bounds.size.height.0 / 2.);
    let centre_to_point = (
        p.0 - q.bounds.origin.x.0 - half.0,
        p.1 - q.bounds.origin.y.0 - half.1,
    );
    let radius = q.corner_radii.top_left.0;
    let border = q.border_widths.top.0;
    let corner_to_point = (
        centre_to_point.0.abs() - half.0,
        centre_to_point.1.abs() - half.1,
    );
    let corner_centre_to_point = (corner_to_point.0 + radius, corner_to_point.1 + radius);
    let outer = if radius == 0. {
        corner_centre_to_point.0.max(corner_centre_to_point.1)
    } else {
        corner_centre_to_point
            .0
            .max(0.)
            .hypot(corner_centre_to_point.1.max(0.))
            + corner_centre_to_point
                .0
                .max(corner_centre_to_point.1)
                .min(0.)
            - radius
    };
    let inner = if corner_centre_to_point.0 <= 0. || corner_centre_to_point.1 <= 0. {
        -(corner_to_point.0 + border).max(corner_to_point.1 + border)
    } else if corner_to_point.0 + border > 0. || corner_to_point.1 + border > 0. {
        -1.
    } else {
        -(outer + border)
    };
    inner.max(outer)
}

/// At fractional scales, where gpui snaps each quad's bounds and border width
/// to device pixels but not its corner radius, every device pixel of a
/// flare's square outside the curve, from a pixel beyond the outline's outer
/// edge at `r - w` on, is painted by the fill or the outline: no seam opens
/// between them. Read from the painted quads through the shader's distance.
#[gpui::test]
fn at_a_fractional_scale_no_seam_opens_between_fill_and_outline(cx: &mut TestAppContext) {
    let (cx, r) = folder_window(cx, Some(4.), |_| {}, Rc::new(RefCell::new(Vec::new())));
    let look = TabLook::of(&r).expect("finite");
    let (rr, w) = (4., f32::from(look.border_width));
    for scale in [1.25, 1.5, 1.75] {
        cx.simulate_scale_factor_change(scale);
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let s = cx.update(|window, _| window.scale_factor());
        assert_eq!(s, scale, "the window took the scale");
        let flares = tab_bar::flare_quads(bounds(cx, "two"), &look);
        assert_eq!(flares.len(), 4, "{scale}");
        let (fills, outlines) = flares.split_at(2);
        for (fill, outline) in fills.iter().zip(outlines) {
            let painted: Vec<gpui::Quad> = [fill, outline]
                .into_iter()
                .flat_map(|f| {
                    let quads = painted_about(cx, f);
                    assert_eq!(quads.len(), 1, "{scale}: {f:?} painted as {quads:?}");
                    quads
                })
                .collect();
            for q in &painted {
                let e = q.border_widths;
                assert!(
                    e.top == e.right && e.top == e.bottom && e.top == e.left,
                    "{scale}: {q:?}"
                );
            }
            let square = fill.clip.scale(s);
            let b = fill.bounds.scale(s);
            let centre = (
                b.origin.x.0 + b.size.width.0 / 2.,
                b.origin.y.0 + b.size.height.0 / 2.,
            );
            let mut bare = Vec::new();
            let (x0, y0) = (square.origin.x.0.floor(), square.origin.y.0.floor());
            let (x1, y1) = (
                (square.origin.x.0 + square.size.width.0).ceil(),
                (square.origin.y.0 + square.size.height.0).ceil(),
            );
            let mut y = y0 + 0.5;
            while y < y1 {
                let mut x = x0 + 0.5;
                while x < x1 {
                    let inside = x > square.origin.x.0
                        && x < square.origin.x.0 + square.size.width.0
                        && y > square.origin.y.0
                        && y < square.origin.y.0 + square.size.height.0;
                    let d = (x - centre.0).hypot(y - centre.1);
                    if inside && d >= (rr - w) * s + 1. {
                        let painted_here = painted.iter().any(|q| {
                            q.content_mask
                                .bounds
                                .contains(&point(gpui::ScaledPixels(x), gpui::ScaledPixels(y)))
                                && border_sdf(q, (x, y)) <= 0.
                        });
                        if !painted_here {
                            bare.push((x, y, d / s));
                        }
                    }
                    x += 1.;
                }
                y += 1.;
            }
            assert!(
                bare.is_empty(),
                "{scale}: pixels of {fill:?} painted by neither ring (x, y, distance): {bare:?}"
            );
        }
    }
}

/// With a bottom indicator and the radius both stated: the line spans the
/// tab's outer width at its bottom, which has no border to inset it by, and
/// the flares are painted over its ends as without it.
#[gpui::test]
fn a_bottom_indicator_reaches_the_bottom_of_a_tab_that_curves_out(cx: &mut TestAppContext) {
    let (cx, r) = folder_window(
        cx,
        Some(4.),
        |r| {
            r.tab.active_indicator_color = Some(r.defaults.accent_color);
            r.tab.active_indicator_width = Some(2.);
            r.tab.active_indicator_side = Some(TabIndicatorSide::Bottom);
        },
        Rc::new(RefCell::new(Vec::new())),
    );
    let look = TabLook::of(&r).expect("finite");
    let (two, line) = (bounds(cx, "two"), bounds(cx, "native-tab-indicator"));
    assert_eq!(line.bottom(), two.bottom(), "along the tab's bottom edge");
    assert_eq!(
        (line.left(), line.right()),
        (two.left(), two.right()),
        "across the tab"
    );
    assert_flares_painted(cx, &tab_bar::flare_quads(two, &look), "with a bottom line");
}

/// With the radius unstated: no flare, the selected tab's outline has its
/// bottom edge `w` wide, and the first tab starts at the row's start.
#[gpui::test]
fn without_the_radius_the_selected_tab_keeps_its_bottom_edge(cx: &mut TestAppContext) {
    let (cx, r) = folder_window(cx, None, |_| {}, Rc::new(RefCell::new(Vec::new())));
    let look = TabLook::of(&r).expect("finite");
    let (bar, one, two) = (
        bounds(cx, "native-tab-bar"),
        bounds(cx, "one"),
        bounds(cx, "two"),
    );
    assert_eq!(one.left(), bar.left(), "no room at the row's start");
    let mut stated = look;
    stated.outward_radius = Some(px(4.));
    for flare in tab_bar::flare_quads(two, &stated) {
        let painted = painted_in(cx, &flare);
        assert!(painted.is_empty(), "a flare painted: {painted:?}");
    }
    let s = cx.update(|window, _| window.scale_factor());
    let w = f32::from(look.border_width) * s;
    let outline = selected_outline(cx, two, &look);
    assert!(!outline.is_empty(), "the selected tab's outline is painted");
    assert!(
        outline.iter().all(|e| *e == [w, w, w, w]),
        "the outline keeps its bottom edge: {outline:?}"
    );
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

/// A line thinner than a device pixel -- macos-tahoe's `separator.line_width`
/// 0.5, on a 1x display -- is drawn one device pixel thick, where a box of
/// its height took no room (M9-165).
#[gpui::test]
fn a_separator_thinner_than_a_device_pixel_is_one_device_pixel(cx: &mut TestAppContext) {
    let r = resolved("macos-tahoe", ColorMode::Light);
    assert!(
        r.separator.line_width > 0.0 && r.separator.line_width < 1.0,
        "macos-tahoe's separator.line_width is {}, not under one pixel, so this case tests none",
        r.separator.line_width
    );
    let cx = window_with(
        cx,
        Some(("macos-tahoe", ColorMode::Light)),
        Box::new(|_, _| {
            div()
                .w(px(200.))
                .child(Separator::horizontal())
                .into_any_element()
        }),
    );
    let handle = cx.update(|window, _| window.window_handle());
    cx.simulate_window_scale_factor_change(handle, 1.0);
    redraw(cx);
    let line = bounds(cx, "native-separator");
    assert_eq!(
        (line.size.width, line.size.height),
        (px(200.), px(1.)),
        "the 0.5 px separator at scale 1 is not one device pixel thick"
    );
}

/// M9x CI item 11 (M9-165 reopened): at scale 1 the separator is painted as
/// a filled quad of its colour over its whole box, which every gpui renderer
/// draws. A border-only quad one device pixel tall, what a line drawn as a
/// border is there, is drawn by none: the Metal, wgpu and DirectX quad
/// shaders take a pixel's border by the half of the quad its centre lies in,
/// the bottom border where it lies on the middle (gpui-pre-apple 0.3.12
/// `src/shaders.metal:133-137`, gpui-pre-wgpu 0.3.12 `src/shaders.wgsl:603-611`,
/// gpui-pre-windows 0.3.12 `src/shaders.hlsl:590-593`), so the one row of such
/// a quad takes the zero bottom border and paints nothing, as the Screenshots
/// run 40218601365 showed on macOS and Windows. Under macos-tahoe (0.5 px)
/// and windows-11 (1 px), light.
#[cfg(not(feature = "gpui-fast"))]
#[gpui::test]
fn a_separator_is_painted_as_a_filled_line_at_scale_one(cx: &mut TestAppContext) {
    for preset in ["macos-tahoe", "windows-11"] {
        let r = resolved(preset, ColorMode::Light);
        let colour = crate::colors::rgba_to_hsla(r.separator.line_color);
        let cx = window_with(
            cx,
            Some((preset, ColorMode::Light)),
            Box::new(|_, _| {
                div()
                    .w(px(200.))
                    .child(Separator::horizontal())
                    .into_any_element()
            }),
        );
        let handle = cx.update(|window, _| window.window_handle());
        cx.simulate_window_scale_factor_change(handle, 1.0);
        redraw(cx);
        let line = bounds(cx, "native-separator");
        let filled = cx.update(|window, _| {
            let line = line.scale(window.scale_factor());
            window.painted_quads().iter().any(|q| {
                q.bounds == line
                    && line.size.height.0 >= 1.0
                    && q.background.as_solid() == Some(colour)
            })
        });
        assert!(
            filled,
            "{preset}: the separator at {line:?} is not painted as a filled quad of {colour:?}"
        );
    }
}

/// Part B's review, item 14(g): at scale 2 the 0.5 px separator of
/// macos-tahoe is its stated thickness, 0.5 logical px, one device pixel:
/// the device-pixel floor takes the line no thicker where the theme's
/// thickness is a whole device pixel already.
#[gpui::test]
fn a_hairline_separator_is_its_stated_thickness_at_scale_two(cx: &mut TestAppContext) {
    let cx = window_with(
        cx,
        Some(("macos-tahoe", ColorMode::Light)),
        Box::new(|_, _| {
            div()
                .w(px(200.))
                .child(Separator::horizontal())
                .into_any_element()
        }),
    );
    let handle = cx.update(|window, _| window.window_handle());
    cx.simulate_window_scale_factor_change(handle, 2.0);
    redraw(cx);
    let line = bounds(cx, "native-separator");
    assert_eq!(
        line.size.height,
        px(0.5),
        "the 0.5 px separator at scale 2 is not 0.5 logical px, one device pixel"
    );
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
/// says (tab/tab_bar.rs:138-142).
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

/// [`window_with`] under `preset` with `edit` applied to the resolved theme
/// before it is installed.
fn window_with_resolved<'a>(
    cx: &'a mut TestAppContext,
    preset: &str,
    mode: ColorMode,
    edit: impl FnOnce(&mut ResolvedTheme),
    build: Build,
) -> &'a mut VisualTestContext {
    cx.update(|cx| {
        gpui_component::init(cx);
        let mut r = resolved(preset, mode);
        edit(&mut r);
        let prefs = AccessibilityPreferences::default();
        let theme = crate::to_theme(&r, preset, mode == ColorMode::Dark, &prefs);
        crate::apply(theme, &r, &prefs, cx);
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
    cx.update(|window, cx| window.draw(cx).clear(cx));
}

/// A key pressed and released, as gpui-component's own tests activate a
/// control (checkbox.rs, `activate_key`).
#[allow(clippy::unwrap_used)]
fn activate_key(cx: &mut VisualTestContext, key: &str) {
    let keystroke = cx
        .update(|_, cx| gpui::Keystroke::parse_with_os(key, cx.platform_info().os))
        .unwrap();
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
            scene_quads(window)
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
    // would cross the thumb. Read from `painted_quads()`, which is in paint
    // order where `scene_quads` is not.
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
    if stated(track_colour, s.track_border_line_width).is_some() {
        let (Some(edge), Some(fill)) = order else {
            panic!("{state}: the track's outline and the thumb are not both painted: {order:?}");
        };
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
/// The animation reads the executor's clock (gpui-pre 0.3.12
/// `src/elements/animation.rs:271`, gpui-fast 0.1.15 the same line), which a
/// test steps rather than waits out. Each draw that lays the running
/// animation out asks for its next frame (`Window::request_animation_frame`,
/// which queues a notify of its view), and `simulate_next_frame` delivers
/// that notify, which tests have no frame loop to deliver: a call delivers
/// the request the draw before it queued, the first call `window_with`'s.
fn fill_after(cx: &mut VisualTestContext, ms: u64) -> Bounds<Pixels> {
    cx.executor()
        .advance_clock(std::time::Duration::from_millis(ms));
    cx.update(|window, cx| window.simulate_next_frame(cx));
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
    // Frames 20 ms apart on the test clock until one lays the fill out
    // elsewhere: the sweep takes a second (progress/progress.rs:152), so 100
    // frames, 2 s, cover two turns.
    let moved = (0..100).any(|_| fill_after(cx, 20) != first);
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
/// requested value written back and the view notified.
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
        Box::new(move |_, cx| {
            let (v2, n2, harness) = (v.clone(), n.clone(), cx.entity_id());
            let on_change = move |checked: &bool, _: &mut Window, cx: &mut gpui::App| {
                v2.set(*checked);
                n2.set(n2.get() + 1);
                cx.notify(harness);
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

/// What gpui-kit's test API finds of sliders drawn one under another, each
/// over a state of its own and built by its `build`, under `theme` (with no
/// native theme where `None`): each looked up by gpui-base's slider id,
/// `("slider", entity_id)` (gpui-base slider.rs:610-612).
fn slider_snapshots(
    cx: &mut TestAppContext,
    theme: Option<(&str, ColorMode)>,
    builds: &[fn(Slider) -> Slider],
) -> Vec<Option<gpui_kit::test::ElementSnapshot>> {
    let builds = builds.to_vec();
    let states: Vec<Entity<SliderState>> = builds
        .iter()
        .map(|_| cx.new(|_| SliderState::new()))
        .collect();
    let drawn = states.clone();
    let cx = window_with(
        cx,
        theme,
        Box::new(move |_, _| {
            div()
                .flex()
                .flex_col()
                .children(
                    drawn
                        .iter()
                        .zip(&builds)
                        .map(|(state, build)| build(Slider::new(state).w(px(100.)))),
                )
                .into_any_element()
        }),
    );
    // A query by id alone would find gpui-component's slider just as well.
    assert_eq!(
        cx.debug_bounds("native-slider").is_some(),
        theme.is_some(),
        "the connector draws its slider exactly under a native theme"
    );
    cx.update(|window, _| {
        states
            .iter()
            .map(|state| {
                gpui_kit::test::TestWindowExt::try_find(window, ("slider", state.entity_id()))
            })
            .collect()
    })
}

/// G3 T7 fix round 1: a themed slider registers for gpui-kit's test API
/// under gpui-base's slider id, as gpui-component's slider does, so a test
/// finds it by id, role and label under a native theme too; an enabled one's
/// focus binding is observed (`focused()` does not panic).
#[gpui::test]
fn a_themed_slider_registers_for_gpui_kits_test_api(cx: &mut TestAppContext) {
    let found = slider_snapshots(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        &[|slider| slider.accessibility_label("Volume")],
    );
    let slider = found
        .into_iter()
        .next()
        .flatten()
        .expect("the themed slider is registered");
    assert_eq!(slider.role(), Some(accesskit::Role::Slider));
    assert_eq!(slider.label(), Some("Volume"));
    assert_eq!(slider.focused(), Some(false));
}

/// G3 T7: a themed slider announces its `accessibility_label`, as
/// gpui-component's does (slider.rs:126-130, :269-273), and no name without
/// one.
#[gpui::test]
fn a_slider_announces_its_accessibility_label(cx: &mut TestAppContext) {
    let found = slider_snapshots(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        &[
            |slider| slider.accessibility_label("Announced"),
            |slider| slider,
        ],
    );
    let labels: Vec<Option<String>> = found
        .iter()
        .map(|slider| {
            let slider = slider.as_ref().expect("the themed slider is registered");
            slider.label().map(str::to_owned)
        })
        .collect();
    assert_eq!(labels, vec![Some("Announced".to_owned()), None]);
}

/// G3 T7: a disabled themed slider's node is disabled, as gpui-base marks
/// its own slider's (a11y_disabled.rs:130-135, slider.rs:656); an enabled
/// one's is not.
#[gpui::test]
fn a_disabled_slider_reports_itself_disabled(cx: &mut TestAppContext) {
    let found = slider_snapshots(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        &[|slider| slider.disabled(true), |slider| slider],
    );
    let disabled: Vec<Option<bool>> = found
        .iter()
        .map(|slider| {
            slider
                .as_ref()
                .expect("the themed slider is registered")
                .disabled()
        })
        .collect();
    assert_eq!(disabled, [Some(true), None]);
}

/// G3 T7: without a native theme the slider passes its label and its
/// disabled state on to gpui-component's, whose gpui-base slider registers
/// them for gpui-kit's test API (gpui-base slider.rs:610-612,
/// test_support.rs:497-509).
#[gpui::test]
fn without_a_native_theme_a_slider_passes_on_its_label_and_disabled_state(cx: &mut TestAppContext) {
    let found = slider_snapshots(
        cx,
        None,
        &[
            |slider| slider.accessibility_label("Enabled"),
            |slider| slider.disabled(true).accessibility_label("Disabled"),
        ],
    );
    let found: Vec<(Option<String>, Option<bool>)> = found
        .iter()
        .map(|slider| {
            let slider = slider.as_ref().expect("gpui-base's slider is registered");
            (slider.label().map(str::to_owned), slider.disabled())
        })
        .collect();
    assert_eq!(
        found,
        [
            (Some("Enabled".to_owned()), None),
            (Some("Disabled".to_owned()), Some(true)),
        ]
    );
}

/// M6-27: `focus_ring(false)` keeps a focused switch's track free of the
/// ring: told to, the focused track draws the theme's ring
/// ([`FocusRing::draw`], kde-breeze's across the track's edge, from 1 px
/// inside it to 1 px outside), and told not to, none.
#[gpui::test]
fn a_switch_told_not_to_draws_no_focus_ring(cx: &mut TestAppContext) {
    let r = resolved("kde-breeze", ColorMode::Light);
    let look = SwitchLook::of(&r, false, false).expect("finite");
    for (focus_ring, drawn) in [(true, true), (false, false)] {
        let cx = window_with(
            cx,
            Some(("kde-breeze", ColorMode::Light)),
            Box::new(move |window, cx| {
                Switch::new("s")
                    .focus_ring(focus_ring)
                    .track(&look, true, look.thumb_left(false), window, cx)
                    .into_any_element()
            }),
        );
        redraw(cx);
        assert_eq!(
            cx.debug_bounds(FOCUS_RING_SELECTOR).is_some(),
            drawn,
            "focus_ring({focus_ring})"
        );
    }
}

/// A key no control in the window binds, pressed and released: the last
/// input is then a key, and the focus has not moved.
fn a_key_that_moves_no_focus(cx: &mut VisualTestContext) {
    let before = cx.update(|window, cx| window.focused(cx));
    activate_key(cx, "f1");
    let after = cx.update(|window, cx| window.focused(cx));
    assert_eq!(before, after, "the key moved the focus");
}

/// A focused Checkbox, Radio, Switch and Slider each draw the theme's ring
/// (C-06): kde-breeze's after a key press, as the theme says keyboard only,
/// across the control's edge, from 1 px inside it to 1 px outside; adwaita's
/// inset one as a band inside the edge of the borderless row, track and thumb.
#[gpui::test]
fn every_focused_control_draws_the_ring(cx: &mut TestAppContext) {
    type Build = Box<dyn Fn() -> AnyElement>;
    for preset in ["kde-breeze", "adwaita"] {
        let controls: [(&str, Build); 3] = [
            (
                "checkbox",
                Box::new(|| Checkbox::new("c").label("C").into_any_element()),
            ),
            (
                "radio",
                Box::new(|| Radio::new("r").label("R").into_any_element()),
            ),
            ("switch", Box::new(|| Switch::new("s").into_any_element())),
        ];
        for (name, build) in controls {
            let cx = window_with(
                cx,
                Some((preset, ColorMode::Light)),
                Box::new(move |_, _| build()),
            );
            assert!(
                cx.debug_bounds(FOCUS_RING_SELECTOR).is_none(),
                "{preset} {name}: a ring before the focus"
            );
            if preset == "kde-breeze" {
                a_key_that_moves_no_focus(cx);
            }
            focus_next(cx);
            redraw(cx);
            assert!(
                cx.debug_bounds(FOCUS_RING_SELECTOR).is_some(),
                "{preset} {name}: no ring once focused"
            );
        }
    }
    let (cx, _state) = slider_window(cx, false);
    a_key_that_moves_no_focus(cx);
    focus_next(cx);
    redraw(cx);
    assert!(
        cx.debug_bounds(FOCUS_RING_SELECTOR).is_some(),
        "kde-breeze slider: no ring once focused"
    );
}

/// The three theme-drawn controls the D4 tests focus, built in a window's
/// harness: a Checkbox, a Switch, and a Slider over `state`.
fn d4_controls(state: &Entity<SliderState>) -> [(&'static str, Build); 3] {
    let s = state.clone();
    [
        (
            "checkbox",
            Box::new(|_, _| Checkbox::new("c").label("C").into_any_element()),
        ),
        (
            "switch",
            Box::new(|_, _| Switch::new("s").into_any_element()),
        ),
        (
            "slider",
            Box::new(move |_, _| Slider::new(&s).w(px(140.)).into_any_element()),
        ),
    ]
}

fn d4_slider_state(cx: &mut TestAppContext) -> Entity<SliderState> {
    cx.new(|_| {
        SliderState::new()
            .min(0.)
            .max(100.)
            .step(1.)
            .default_value(50.)
    })
}

/// D4: under kde-breeze, which says the platform shows its ring for keyboard
/// focus only, a Checkbox, a Switch and a Slider the program focuses with no
/// key before draw no ring; the same theme with
/// `focus_ring_keyboard_only` unset rings the same focus.
#[gpui::test]
fn a_control_focused_without_a_key_shows_no_ring_where_the_theme_says_keyboard_only(
    cx: &mut TestAppContext,
) {
    assert_eq!(
        resolved("kde-breeze", ColorMode::Light)
            .defaults
            .focus_ring_keyboard_only,
        Some(true)
    );
    for (unset, drawn) in [(false, false), (true, true)] {
        let state = d4_slider_state(cx);
        for (name, build) in d4_controls(&state) {
            let cx = window_with_resolved(
                cx,
                "kde-breeze",
                ColorMode::Light,
                |r| {
                    if unset {
                        r.defaults.focus_ring_keyboard_only = None;
                    }
                },
                build,
            );
            focus_next(cx);
            redraw(cx);
            assert_eq!(
                cx.debug_bounds(FOCUS_RING_SELECTOR).is_some(),
                drawn,
                "{name}, focus_ring_keyboard_only unset: {unset}"
            );
        }
    }
}

/// D4: a ring that followed a key stays through a mouse move and ends with
/// the focus; the next focus, with no key before, draws none. The frame
/// between the blur and the refocus lets the control see itself unfocused
/// (`ring_shows` samples at render).
#[gpui::test]
fn the_ring_outlasts_a_mouse_move_and_ends_with_the_focus(cx: &mut TestAppContext) {
    let state = d4_slider_state(cx);
    for (name, build) in d4_controls(&state) {
        let cx = window_with(cx, Some(("kde-breeze", ColorMode::Light)), build);
        a_key_that_moves_no_focus(cx);
        focus_next(cx);
        redraw(cx);
        let ring = bounds(cx, FOCUS_RING_SELECTOR);
        assert!(
            cx.debug_bounds(FOCUS_RING_SELECTOR).is_some(),
            "{name}: ringed after a key"
        );
        cx.simulate_mouse_move(centre(ring), None, Modifiers::default());
        redraw(cx);
        assert!(
            cx.debug_bounds(FOCUS_RING_SELECTOR).is_some(),
            "{name}: the ring left at a mouse move"
        );
        cx.update(|window, cx| window.blur(cx));
        redraw(cx);
        assert!(
            cx.debug_bounds(FOCUS_RING_SELECTOR).is_none(),
            "{name}: a ring without the focus"
        );
        focus_next(cx);
        redraw(cx);
        assert!(
            cx.debug_bounds(FOCUS_RING_SELECTOR).is_none(),
            "{name}: a ring for a focus that followed no key"
        );
    }
}

/// A theme's stated ring, `width` wide `offset` off the element's edge,
/// drawn round a 40 px square with a 1 px border and 4 px corners.
fn ringed(
    cx: &mut TestAppContext,
    width: f32,
    offset: f32,
) -> (&mut VisualTestContext, ResolvedTheme) {
    ringed_square(cx, width, offset, true)
}

/// [`ringed`], the square with a 1 px border where `bordered`, and none
/// where not.
fn ringed_square(
    cx: &mut TestAppContext,
    width: f32,
    offset: f32,
    bordered: bool,
) -> (&mut VisualTestContext, ResolvedTheme) {
    let mut r = resolved("kde-breeze", ColorMode::Light);
    r.defaults.focus_ring_width = width;
    r.defaults.focus_ring_offset = offset;
    let ring = FocusRing::of(&r);
    let cx = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(move |window, _| {
            use gpui::prelude::FluentBuilder as _;
            let square = div()
                .debug_selector(|| "ringed".into())
                .relative()
                .size(px(40.))
                .when(bordered, |square| square.border(px(1.)))
                .rounded(px(4.));
            match ring {
                Some(ring) => ring.draw(square, window).into_any_element(),
                None => square.into_any_element(),
            }
        }),
    );
    redraw(cx);
    (cx, r)
}

/// The quads painted over `b` grown by `grow` on every side, scaled.
fn quads_round(cx: &mut VisualTestContext, b: Bounds<Pixels>, grow: f32) -> Vec<gpui::Quad> {
    cx.update(|window, _| {
        let scale = window.scale_factor();
        let want = b.dilate(px(grow)).scale(scale);
        scene_quads(window)
            .into_iter()
            .filter(|q| q.bounds == want)
            .collect()
    })
}

/// The ring is the theme's (C-06): `defaults.focus_ring_color` as stated, a
/// band `focus_ring_width` wide whose outer edge is the element's bounds
/// grown by `offset + width`, its corners the element's grown as much --
/// for a ring outside the edge and for one that straddles it; where it lies
/// inset at the edge the element's border takes the colour; where the width
/// is not positive there is none.
#[gpui::test]
fn the_focus_ring_is_the_themes(cx: &mut TestAppContext) {
    for (width, offset) in [(1.0_f32, 2.0_f32), (3.0, -1.0)] {
        let (cx, r) = ringed(cx, width, offset);
        let ring = color(r.defaults.focus_ring_color);
        let square = bounds(cx, "ringed");
        let grow = offset + width;
        let band = quads_round(cx, square, grow);
        let scale = cx.update(|window, _| window.scale_factor());
        assert!(
            band.iter().any(|q| q.border_color == ring
                && (q.border_widths.top.0 - width * scale).abs() < 0.01
                && (q.corner_radii.top_left.0 - (4.0 + grow) * scale).abs() < 0.01),
            "width {width}, offset {offset}: no ring at {square:?} grown by {grow}: {band:?}"
        );
        assert_eq!(
            cx.debug_bounds(FOCUS_RING_SELECTOR).map(|b| b.size),
            Some(square.dilate(px(grow)).size),
            "width {width}, offset {offset}"
        );
    }
    // Inset at the edge: the square's own border is the ring.
    let (cx, r) = ringed(cx, 2.0, -2.0);
    let ring = color(r.defaults.focus_ring_color);
    let square = bounds(cx, "ringed");
    assert!(
        cx.debug_bounds(FOCUS_RING_SELECTOR).is_none(),
        "an inset ring adds no band"
    );
    assert!(
        quads_round(cx, square, 0.0)
            .iter()
            .any(|q| q.border_color == ring && q.border_widths.top.0 > 0.0),
        "an inset ring is not the square's border"
    );
    // Inset at the edge of an element with no border of its own (adwaita's
    // ring on the checkbox row, the switch track, the slider thumb): the band
    // inside the edge, at the element's corners.
    let (cx, r) = ringed_square(cx, 2.0, -2.0, false);
    let ring = color(r.defaults.focus_ring_color);
    let square = bounds(cx, "ringed");
    let scale = cx.update(|window, _| window.scale_factor());
    assert_eq!(
        cx.debug_bounds(FOCUS_RING_SELECTOR).map(|b| b.size),
        Some(square.size),
        "a borderless inset ring is a band at the edge"
    );
    assert!(
        quads_round(cx, square, 0.0)
            .iter()
            .any(|q| q.border_color == ring
                && (q.border_widths.top.0 - 2.0 * scale).abs() < 0.01
                && (q.corner_radii.top_left.0 - 4.0 * scale).abs() < 0.01),
        "no 2 px band inside the borderless square's edge"
    );
    // No ring at all.
    for (width, offset) in [(0.0_f32, 1.0_f32), (f32::NAN, 1.0), (1.0, f32::INFINITY)] {
        let (cx, r) = ringed(cx, width, offset);
        let ring = color(r.defaults.focus_ring_color);
        let square = bounds(cx, "ringed");
        assert!(
            cx.debug_bounds(FOCUS_RING_SELECTOR).is_none(),
            "{width}, {offset}"
        );
        assert!(
            !quads_round(cx, square, 0.0)
                .iter()
                .any(|q| q.border_color == ring),
            "{width}, {offset}: the square's border took the ring's colour"
        );
    }
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
/// as gpui-component's slider draws one (slider.rs:157-164, :316-319).
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
/// (slider.rs:789-801), and the start thumb follows; dragging that thumb
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
/// pointer keeps it (gpui-base slider.rs:392-396).
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
/// at the bottom, as gpui-base maps the pointer (slider.rs:381-385), and the
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
        scene_quads(window)
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

// ---------------------------------------------------------------------------
// The tab bar's keyboard focus
// ---------------------------------------------------------------------------

/// A window drawing a kde-breeze tab bar of [`TAB_NAMES`]' eight tabs across
/// the harness's 200px column, the tab at `selected` selected; a click on a
/// tab, or Enter on the focused one, pushes its index onto `activated`.
fn keyboard_tab_window(
    cx: &mut TestAppContext,
    selected: usize,
    activated: Rc<RefCell<Vec<usize>>>,
) -> &mut VisualTestContext {
    edited_tab_window(cx, selected, activated, |_| {})
}

/// [`keyboard_tab_window`] under kde-breeze with `edit` applied to the
/// resolved theme ([`window_with_resolved`]).
fn edited_tab_window(
    cx: &mut TestAppContext,
    selected: usize,
    activated: Rc<RefCell<Vec<usize>>>,
    edit: impl FnOnce(&mut ResolvedTheme),
) -> &mut VisualTestContext {
    let cx = window_with_resolved(
        cx,
        "kde-breeze",
        ColorMode::Light,
        edit,
        Box::new(move |_, _| {
            let activated = activated.clone();
            TabBar::new("t")
                .w_full()
                .children(TAB_NAMES.map(|name| Tab::new(name).debug_selector(|| name.into())))
                .selected_index(selected)
                .on_click(move |ix, _, _| activated.borrow_mut().push(*ix))
                .into_any_element()
        }),
    );
    redraw(cx);
    redraw(cx);
    cx
}

/// The tab Enter activates, which is the focused one, or none.
fn focused_tab(cx: &mut VisualTestContext, activated: &Rc<RefCell<Vec<usize>>>) -> Option<usize> {
    activated.borrow_mut().clear();
    activate_key(cx, "enter");
    redraw(cx);
    let tab = activated.borrow().last().copied();
    activated.borrow_mut().clear();
    tab
}

fn anything_focused(cx: &mut VisualTestContext) -> bool {
    cx.update(|window, cx| window.focused(cx).is_some())
}

/// T4: a click activates a tab and leaves the focus where it was, none or
/// another tab; Tab enters the list on the selected tab, and the arrow keys,
/// Home and End move the focus to the next, previous, first and last tab.
#[gpui::test]
fn a_click_keeps_the_focus_and_the_keys_move_it(cx: &mut TestAppContext) {
    let activated: Rc<RefCell<Vec<usize>>> = Rc::default();
    let cx = keyboard_tab_window(cx, 2, activated.clone());
    assert!(!anything_focused(cx));
    let tab1 = bounds(cx, "tab1");
    cx.simulate_click(centre(tab1), Modifiers::default());
    redraw(cx);
    assert_eq!(*activated.borrow(), [1], "the click activated tab 1");
    assert!(!anything_focused(cx), "the click moved no focus");

    focus_next(cx);
    redraw(cx);
    assert_eq!(
        focused_tab(cx, &activated),
        Some(2),
        "Tab enters on the selected tab"
    );
    for (keys, tab) in [("right", 3), ("left", 2), ("end", 7), ("home", 0)] {
        cx.simulate_keystrokes(keys);
        redraw(cx);
        assert_eq!(focused_tab(cx, &activated), Some(tab), "{keys}");
    }

    let tab1 = bounds(cx, "tab1");
    cx.simulate_click(centre(tab1), Modifiers::default());
    redraw(cx);
    assert_eq!(*activated.borrow(), [1], "the click activated tab 1");
    assert_eq!(
        focused_tab(cx, &activated),
        Some(0),
        "the keyboard focus stayed on tab 0"
    );
}

/// The shadows the last frame painted over the bar `bar`.
#[cfg(feature = "gpui-fast")]
fn shadows_over(cx: &mut VisualTestContext, bar: Bounds<Pixels>) -> Vec<gpui::Shadow> {
    cx.update(|window, _| {
        let bar = bar.scale(window.scale_factor());
        window
            .painted_scene()
            .shadows
            .iter()
            .filter(|s| s.element_bounds.intersects(&bar))
            .cloned()
            .collect()
    })
}

/// T4 (b'): no tab draws a ring at rest; the tab the keyboard focuses draws
/// the theme's, one inset shadow in `defaults.focus_ring_color` whose
/// spread is `defaults.focus_ring_width`, inside the tab; after a click on
/// another tab, which moves no focus, it stays (D4). It reads the painted
/// shadows, which GPUI Fast's scene holds.
#[cfg(feature = "gpui-fast")]
#[gpui::test]
fn the_keyboard_focused_tab_draws_the_ring_inside(cx: &mut TestAppContext) {
    let r = resolved("kde-breeze", ColorMode::Light);
    let activated: Rc<RefCell<Vec<usize>>> = Rc::default();
    let cx = keyboard_tab_window(cx, 0, activated);
    let bar = bounds(cx, "native-tab-bar");
    let painted = shadows_over(cx, bar);
    assert!(painted.is_empty(), "no ring at rest: {painted:?}");

    focus_next(cx);
    cx.simulate_keystrokes("right");
    redraw(cx);
    let tab = bounds(cx, "tab1");
    let painted = shadows_over(cx, bar);
    let [ring] = painted.as_slice() else {
        panic!("one ring on the focused tab: {painted:?}");
    };
    let scale = cx.update(|window, _| window.scale_factor());
    let want = tab.scale(scale);
    assert_eq!(ring.inset, 1, "inset");
    assert_eq!(ring.color, c(r.defaults.focus_ring_color));
    assert!(
        ring.element_bounds.contains(&want.center()) && ring.element_bounds.size == want.size,
        "on the focused tab: {:?}, tab {want:?}",
        ring.element_bounds
    );
    let spread = r.defaults.focus_ring_width * scale;
    let e = ring.element_bounds;
    let hole = ring.bounds;
    assert_eq!(
        [
            hole.origin.x.0 - e.origin.x.0,
            hole.origin.y.0 - e.origin.y.0,
            e.bottom_right().x.0 - hole.bottom_right().x.0,
            e.bottom_right().y.0 - hole.bottom_right().y.0,
        ],
        [spread; 4],
        "spread by the ring's width"
    );

    let tab2 = bounds(cx, "tab2");
    cx.simulate_click(centre(tab2), Modifiers::default());
    redraw(cx);
    let painted = shadows_over(cx, bar);
    let tab = bounds(cx, "tab1").scale(scale);
    assert!(
        painted.len() == 1
            && painted
                .iter()
                .all(|s| s.element_bounds.contains(&tab.center())),
        "after the click, which moved no focus, one ring on tab 1: {painted:?}"
    );
}

/// D4: under kde-breeze, which says keyboard only, the tab the program
/// focuses with no key before draws no ring; an arrow key's move rings the
/// tab it reaches, and a mouse move leaves that ring. With
/// `focus_ring_keyboard_only` unset, the same focus rings the selected tab.
#[cfg(feature = "gpui-fast")]
#[gpui::test]
fn a_tab_focused_without_a_key_draws_no_ring_where_the_theme_says_keyboard_only(
    cx: &mut TestAppContext,
) {
    let one_ring_on = |cx: &mut VisualTestContext, bar: Bounds<Pixels>, name: &'static str| {
        let painted = shadows_over(cx, bar);
        let scale = cx.update(|window, _| window.scale_factor());
        let tab = bounds(cx, name).scale(scale);
        painted.len() == 1
            && painted
                .iter()
                .all(|s| s.element_bounds.contains(&tab.center()))
    };
    let activated: Rc<RefCell<Vec<usize>>> = Rc::default();
    let cx = keyboard_tab_window(cx, 0, activated.clone());
    let bar = bounds(cx, "native-tab-bar");
    focus_next(cx);
    redraw(cx);
    let painted = shadows_over(cx, bar);
    assert!(painted.is_empty(), "no ring without a key: {painted:?}");
    cx.simulate_keystrokes("right");
    redraw(cx);
    assert!(
        one_ring_on(cx, bar, "tab1"),
        "one ring on tab 1 after Right"
    );
    cx.simulate_mouse_move(centre(bar), None, Modifiers::default());
    redraw(cx);
    assert!(
        one_ring_on(cx, bar, "tab1"),
        "the ring on tab 1 after a mouse move"
    );

    let cx = edited_tab_window(cx, 0, activated, |r| {
        r.defaults.focus_ring_keyboard_only = None;
    });
    let bar = bounds(cx, "native-tab-bar");
    focus_next(cx);
    redraw(cx);
    assert!(
        one_ring_on(cx, bar, "tab0"),
        "unset: one ring on the selected tab"
    );
}

/// The strip scrolled to its last tab, an element round the bar takes the
/// focus: every tab's probe is then within the focus, and the strip stays
/// where it is (two frames).
#[gpui::test]
fn focusing_an_element_round_the_bar_keeps_the_strip_where_it_is(cx: &mut TestAppContext) {
    let handle = gpui::ScrollHandle::new();
    let outer = cx.update(|cx| cx.focus_handle());
    let (s, o) = (handle.clone(), outer.clone());
    let cx = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(move |_, _| {
            div()
                .w_full()
                .track_focus(&o)
                .child(
                    TabBar::new("t")
                        .w_full()
                        .children(
                            TAB_NAMES.map(|name| Tab::new(name).debug_selector(|| name.into())),
                        )
                        .selected_index(3)
                        .track_scroll(&s),
                )
                .into_any_element()
        }),
    );
    redraw(cx);
    redraw(cx);
    handle.scroll_to_item(TAB_NAMES.len() - 1);
    redraw(cx);
    redraw(cx);
    let before = handle.offset().x;
    assert!(before < px(0.), "the strip scrolled: {before:?}");
    cx.update(|window, cx| outer.focus(window, cx));
    redraw(cx);
    redraw(cx);
    assert!(
        cx.update(|window, _| outer.is_focused(window)),
        "the element round the bar has the focus"
    );
    assert_eq!(handle.offset().x, before, "the strip stayed where it was");
}

/// A window drawing [`keyboard_tab_window`]'s bar inside an element that
/// counts the right and the middle presses that reach it, as a context menu
/// round the bar would open on them.
fn pressed_tab_window(
    cx: &mut TestAppContext,
    activated: Rc<RefCell<Vec<usize>>>,
    right: Rc<Cell<usize>>,
    middle: Rc<Cell<usize>>,
) -> &mut VisualTestContext {
    let cx = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(move |_, _| {
            let activated = activated.clone();
            let (right, middle) = (right.clone(), middle.clone());
            div()
                .w_full()
                .on_mouse_down(gpui::MouseButton::Right, move |_, _, _| {
                    right.set(right.get() + 1)
                })
                .on_mouse_down(gpui::MouseButton::Middle, move |_, _, _| {
                    middle.set(middle.get() + 1)
                })
                .child(
                    TabBar::new("t")
                        .w_full()
                        .children(
                            TAB_NAMES.map(|name| Tab::new(name).debug_selector(|| name.into())),
                        )
                        .on_click(move |ix, _, _| activated.borrow_mut().push(*ix)),
                )
                .into_any_element()
        }),
    );
    redraw(cx);
    redraw(cx);
    cx
}

/// gpui-base's `Tab` keeps only the left button from focusing it; the bar
/// keeps a right and a middle press from it too, neither activates a tab,
/// and both reach the element round the bar.
#[gpui::test]
fn a_right_or_a_middle_press_moves_no_focus(cx: &mut TestAppContext) {
    let activated: Rc<RefCell<Vec<usize>>> = Rc::default();
    let (right, middle): (Rc<Cell<usize>>, Rc<Cell<usize>>) = Default::default();
    let cx = pressed_tab_window(cx, activated.clone(), right.clone(), middle.clone());
    for (button, count) in [
        (gpui::MouseButton::Right, &right),
        (gpui::MouseButton::Middle, &middle),
    ] {
        let at = centre(bounds(cx, "tab1"));
        cx.simulate_mouse_down(at, button, Modifiers::default());
        cx.simulate_mouse_up(at, button, Modifiers::default());
        redraw(cx);
        assert!(
            !anything_focused(cx),
            "{button:?}: the press moved the focus"
        );
        assert!(
            activated.borrow().is_empty(),
            "{button:?}: the press activated {:?}",
            activated.borrow()
        );
        assert_eq!(
            count.get(),
            1,
            "{button:?}: the press reached the element round the bar"
        );
    }
}

/// A press of any button on the pixel of a tab's scroll probe, the focus
/// handle inside the tab at its padding box's top left corner, moves no
/// focus either; a left press there still activates the tab.
#[gpui::test]
fn a_press_on_a_tabs_scroll_probe_moves_no_focus(cx: &mut TestAppContext) {
    let look = TabLook::of(&resolved("kde-breeze", ColorMode::Light));
    assert!(look.is_some(), "kde-breeze's tab look resolves");
    let Some(look) = look else {
        return;
    };
    let activated: Rc<RefCell<Vec<usize>>> = Rc::default();
    let cx = keyboard_tab_window(cx, 0, activated.clone());
    let inset = f32::from(look.border_width) + 0.5;
    for button in [
        gpui::MouseButton::Left,
        gpui::MouseButton::Right,
        gpui::MouseButton::Middle,
    ] {
        activated.borrow_mut().clear();
        let at = bounds(cx, "tab1").origin + point(px(inset), px(inset));
        cx.simulate_mouse_down(at, button, Modifiers::default());
        cx.simulate_mouse_up(at, button, Modifiers::default());
        redraw(cx);
        assert!(
            !anything_focused(cx),
            "{button:?}: the press on the probe moved the focus"
        );
        let want: &[usize] = if button == gpui::MouseButton::Left {
            &[1]
        } else {
            &[]
        };
        assert_eq!(
            activated.borrow().as_slice(),
            want,
            "{button:?}: the tabs the press activated"
        );
    }
}

/// T4: where the focus moves to a tab outside the strip, the strip scrolls
/// it in: the tab the bar's edge cuts, then by End the last, by Home the
/// first.
#[gpui::test]
fn the_strip_scrolls_the_focused_tab_into_view(cx: &mut TestAppContext) {
    let activated: Rc<RefCell<Vec<usize>>> = Rc::default();
    let cx = keyboard_tab_window(cx, 0, activated.clone());
    let bar = bounds(cx, "native-tab-bar");
    let inside = |b: Bounds<Pixels>| b.left() >= bar.left() && b.right() <= bar.right();
    let (ix, name) = half_hidden_tab(cx);
    assert!(
        !inside(bounds(cx, "tab7")),
        "the last tab starts out of view"
    );

    focus_next(cx);
    cx.simulate_keystrokes(&vec!["right"; ix].join(" "));
    redraw(cx);
    redraw(cx);
    // Read before Enter, which reveals the tab it activates by itself.
    assert!(inside(bounds(cx, name)), "tab {ix} scrolled into view");
    assert_eq!(focused_tab(cx, &activated), Some(ix));
    for (key, name) in [("end", "tab7"), ("home", "tab0")] {
        cx.simulate_keystrokes(key);
        redraw(cx);
        redraw(cx);
        assert!(inside(bounds(cx, name)), "{key}: {name} scrolled into view");
    }
}

/// T6: right to left, the tabs run from the bar's right edge, each the
/// stated gap to the left of the one before it, as
/// `the_tab_bar_outlines_only_the_selected_tab` finds them to the right left
/// to right; the arrow keys step as gpui-base's `Tabs` steps them, Left to
/// the next tab, which lies to the left; overflowing, the strip scrolls from
/// its origin, the last tab at its left edge, as gpui-component's tab bar
/// lays them out, and the focused tab scrolls into view; wrapped, each row
/// starts at the right.
#[gpui::test]
fn right_to_left_the_tab_bar_runs_from_the_right(cx: &mut TestAppContext) {
    cx.update(|cx| gpui_base::LayoutDirection::Rtl.set_global(cx));
    let r = resolved("kde-breeze", ColorMode::Light);
    {
        let cx = window_with(
            cx,
            Some(("kde-breeze", ColorMode::Light)),
            Box::new(|_, _| {
                TabBar::new("t")
                    .w_full()
                    .child(Tab::new("One").debug_selector(|| "one".into()))
                    .child(Tab::new("Two").debug_selector(|| "two".into()))
                    .selected_index(1)
                    .into_any_element()
            }),
        );
        let bar = bounds(cx, "native-tab-bar");
        let (one, two) = (bounds(cx, "one"), bounds(cx, "two"));
        assert_eq!(one.right(), bar.right(), "the first tab at the right edge");
        assert_eq!(
            Some(one.left() - two.right()),
            r.tab.item_gap.map(px),
            "the second tab tab.item_gap to the first's left"
        );
    }
    {
        let cx = window_with(
            cx,
            Some(("kde-breeze", ColorMode::Light)),
            Box::new(|_, _| {
                TabBar::new("t")
                    .w_full()
                    .wrap()
                    .children(TAB_NAMES.map(|name| Tab::new(name).debug_selector(|| name.into())))
                    .into_any_element()
            }),
        );
        let bar = bounds(cx, "native-tab-bar");
        let (first, last) = (bounds(cx, "tab0"), bounds(cx, "tab7"));
        assert_eq!(
            first.right(),
            bar.right(),
            "wrapped, the first row from the right"
        );
        assert!(last.top() > first.top(), "the tabs wrapped");
    }

    let activated: Rc<RefCell<Vec<usize>>> = Rc::default();
    let cx = keyboard_tab_window(cx, 0, activated.clone());
    let bar = bounds(cx, "native-tab-bar");
    assert_eq!(
        bounds(cx, "tab7").left(),
        bar.left(),
        "the strip scrolls from its origin"
    );
    assert!(bounds(cx, "tab0").right() > bar.right());
    focus_next(cx);
    redraw(cx);
    redraw(cx);
    let tab0 = bounds(cx, "tab0");
    assert!(
        tab0.left() >= bar.left() && tab0.right() <= bar.right(),
        "the focused tab scrolled into view"
    );
    assert!(
        bounds(cx, "tab1").right() <= tab0.left(),
        "tab 1 to the left"
    );
    for (keys, tab) in [
        ("left", 1),
        ("left", 2),
        ("right", 1),
        ("end", 7),
        ("home", 0),
    ] {
        cx.simulate_keystrokes(keys);
        redraw(cx);
        assert_eq!(focused_tab(cx, &activated), Some(tab), "{keys}");
    }
}

// ---------------------------------------------------------------------------
// The check box's mixed state and right-to-left row
// ---------------------------------------------------------------------------

/// T3 row 3: an indeterminate checkbox, checked or not, is drawn in the
/// checked look with gpui-component's dash in place of the check: the box
/// is `checked_background` edged in `border.color`, and the dash, stroked at
/// `check_mark_stroke_width`, is `indicator_color`, its two round ends on the
/// box's middle line; disabled under adwaita, the box and the dash are
/// faded as one over the window ([`CheckboxLook::faded_over`]).
#[gpui::test]
fn an_indeterminate_checkbox_draws_the_checked_box_and_a_dash(cx: &mut TestAppContext) {
    for (preset, checked, disabled) in [
        ("kde-breeze", false, false),
        ("kde-breeze", true, false),
        ("adwaita", false, true),
    ] {
        let r = resolved(preset, ColorMode::Light);
        let ground = c(r.defaults.background_color);
        let look = CheckboxLook::of(&r, true, disabled)
            .expect("finite")
            .faded_over(ground);
        let stroke = look.mark_stroke.expect("a stated check-mark stroke");
        let cx = window_with(
            cx,
            Some((preset, ColorMode::Light)),
            Box::new(move |_, _| {
                Checkbox::new("m")
                    .label("M")
                    .checked(checked)
                    .indeterminate(true)
                    .disabled(disabled)
                    .into_any_element()
            }),
        );
        let indicator = bounds(cx, "native-checkbox-indicator");
        let mark = bounds(cx, "native-checkbox-mark");
        let (fill, edge, ends) = cx.update(|window, _| {
            let scale = window.scale_factor();
            let indicator = indicator.scale(scale);
            let quads = scene_quads(window);
            // gpui paints a box's fill and its border as two quads.
            let fill = quads
                .iter()
                .find(|q| q.bounds == indicator && !q.background.is_transparent())
                .and_then(|q| q.background.as_solid());
            let edge = quads
                .iter()
                .find(|q| q.bounds == indicator && q.border_widths.top.0 > 0.)
                .map(|q| q.border_color);
            let ends: Vec<_> = quads
                .iter()
                .filter(|q| {
                    indicator.contains(&q.bounds.center())
                        && q.bounds.size != indicator.size
                        && (q.bounds.size.width.0 - f32::from(stroke) * scale).abs() <= 1.
                })
                .map(|q| (q.background.as_solid(), q.bounds.center()))
                .collect();
            (fill, edge, ends)
        });
        let at = format!("{preset} checked({checked}) disabled({disabled})");
        assert_eq!(
            (fill, edge),
            (Some(look.fill), Some(look.border)),
            "{at}: the checked box"
        );
        let [(left, a), (right, b)] = ends.as_slice() else {
            panic!("{at}: the dash's two round ends: {ends:?}");
        };
        assert_eq!((*left, *right), (Some(look.mark), Some(look.mark)), "{at}");
        let scale = cx.update(|window, _| window.scale_factor());
        let middle = mark.scale(scale).center().y.0;
        assert!(
            (a.y.0 - middle).abs() <= 1. && (b.y.0 - middle).abs() <= 1.,
            "{at}: on the middle line, {a:?} {b:?}, {middle}"
        );
        let length = f32::from(mark.size.width) * scale * (19. - 5.) / 24.;
        assert!(
            ((a.x.0 - b.x.0).abs() - length).abs() <= 1.,
            "{at}: {a:?} {b:?} {length}"
        );
    }
}

/// Where the theme states no check-mark stroke, an indeterminate checkbox's
/// dash is gpui-component's `IconName::Minus` by its asset path, which the
/// application's icon resolver is asked for in place of the check, as
/// gpui-component's check box asks it (gpui-component 0.10.0 checkbox.rs,
/// `an_indeterminate_checkbox_draws_a_dash`).
#[gpui::test]
fn the_dash_asks_the_applications_icon_resolver(cx: &mut TestAppContext) {
    use gpui_component::{IconName, IconNamed as _};
    let look = CheckboxLook::of(&resolved("macos-monterey", ColorMode::Light), true, false);
    assert_eq!(look.and_then(|l| l.mark_stroke), None);
    let window = window_with(
        cx,
        Some(("macos-monterey", ColorMode::Light)),
        Box::new(|_, _| {
            Checkbox::new("m")
                .label("M")
                .checked(true)
                .indeterminate(true)
                .into_any_element()
        }),
    );
    assert!(window.debug_bounds("native-checkbox-indicator").is_some());
    let record = recording_resolver(window);
    redraw(window);
    let asked = record.borrow();
    assert!(
        asked.contains(&IconName::Minus.path().to_string()),
        "{asked:?}"
    );
    assert!(
        !asked.contains(&IconName::Check.path().to_string()),
        "the dash replaces the check: {asked:?}"
    );
}

/// T3 row 3: an indeterminate checkbox reports gpui-base's mixed state,
/// checked or not; one only checked reports checked.
#[gpui::test]
fn an_indeterminate_checkbox_reports_the_mixed_state(cx: &mut TestAppContext) {
    let captured = Rc::new(RefCell::new(Vec::new()));
    let sink = captured.clone();
    window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(move |_, _| {
            let sink = sink.clone();
            gpui::canvas(
                move |_, window, cx| {
                    *sink.borrow_mut() = [
                        Checkbox::new("a").checked(true).indeterminate(true),
                        Checkbox::new("b").indeterminate(true),
                        Checkbox::new("c").checked(true),
                    ]
                    .map(|checkbox| {
                        let base = checkbox.row(window, cx).ok();
                        a11y_node(accesskit::Role::CheckBox, base, window, cx).toggled()
                    })
                    .to_vec();
                },
                |_, (), _, _| {},
            )
            .size_full()
            .into_any_element()
        }),
    );
    assert_eq!(
        captured.take(),
        [
            Some(accesskit::Toggled::Mixed),
            Some(accesskit::Toggled::Mixed),
            Some(accesskit::Toggled::True),
        ]
    );
}

/// T3 row 3: activating an indeterminate checkbox requests `true`, even
/// where it is also checked: the mixed state wins.
#[gpui::test]
fn activating_an_indeterminate_checkbox_requests_checked(cx: &mut TestAppContext) {
    let requested = Rc::new(Cell::new(None));
    let sink = requested.clone();
    let cx = window_with(
        cx,
        Some(("kde-breeze", ColorMode::Light)),
        Box::new(move |_, _| {
            let sink = sink.clone();
            Checkbox::new("m")
                .label("M")
                .checked(true)
                .indeterminate(true)
                .on_change(move |checked, _, _| sink.set(Some(*checked)))
                .into_any_element()
        }),
    );
    let at = centre(bounds(cx, "native-checkbox-indicator"));
    cx.simulate_click(at, Modifiers::default());
    assert_eq!(requested.get(), Some(true));
}

/// T6: right to left, a checkbox's and a radio's indicator leads on the
/// right, the label `label_gap` to its left, as `the_indicator_is_the_stated_size`
/// finds it to the right left to right.
#[gpui::test]
fn right_to_left_the_indicator_leads_from_the_right(cx: &mut TestAppContext) {
    cx.update(|cx| gpui_base::LayoutDirection::Rtl.set_global(cx));
    let r = resolved("kde-breeze", ColorMode::Light);
    let controls: [Build; 2] = [
        Box::new(|_, _| Checkbox::new("c").label("Label").into_any_element()),
        Box::new(|_, _| Radio::new("r").label("Label").into_any_element()),
    ];
    for build in controls {
        let cx = window_with(cx, Some(("kde-breeze", ColorMode::Light)), build);
        let b = bounds(cx, "native-checkbox-indicator");
        assert_eq!(b.size.width, px(r.checkbox.indicator_width));
        let label = bounds(cx, "native-checkbox-label");
        assert_eq!(
            b.origin.x - label.right(),
            px(r.checkbox.label_gap),
            "the label sits label_gap to the indicator's left"
        );
    }
}

/// T6 (b): right to left, a horizontal slider runs from the right, where
/// gpui-base maps the pointer from (gpui-base 0.10.0 slider.rs,
/// `offset_along`): a range's thumbs and the fill between them stand where
/// its left-to-right test has them, mirrored, a press lands where it is
/// drawn, and Left and Right swap, as gpui-base's keys (`key_delta`); Up and
/// Down do not.
#[gpui::test]
fn right_to_left_the_slider_runs_from_the_right(cx: &mut TestAppContext) {
    use gpui_base::slider::SliderValue;
    cx.update(|cx| gpui_base::LayoutDirection::Rtl.set_global(cx));
    let (cx, state) = range_window(cx);
    let r = resolved("kde-breeze", ColorMode::Light);
    let radius = px(r.slider.thumb_diameter) / 2.;
    let root = bounds(cx, "native-slider");
    let travel = root.size.width - radius * 2.;
    let at = |fraction: f32| root.right() - radius - travel * fraction;
    let start = bounds(cx, "native-slider-start-thumb");
    let end = bounds(cx, "native-slider-thumb");
    assert_eq!(centre(start).x, at(0.2), "the start thumb at 20");
    assert_eq!(centre(end).x, at(0.8), "the end thumb at 80");
    let fill = bounds(cx, "native-slider-fill");
    assert_eq!(fill.right(), at(0.2), "the fill from the start thumb");
    assert_eq!(fill.left(), at(0.8), "to the end thumb");

    let value = |cx: &mut VisualTestContext| cx.update(|_, cx| state.read(cx).value());
    cx.simulate_click(point(at(0.05), centre(root).y), Modifiers::default());
    assert_eq!(value(cx), SliderValue::Range(5., 80.), "the start moved");
    redraw(cx);
    let start = bounds(cx, "native-slider-start-thumb");
    assert_eq!(centre(start).x, at(0.05), "the start thumb follows");

    focus_next(cx);
    cx.simulate_keystrokes("left");
    assert_eq!(value(cx), SliderValue::Range(5., 81.), "Left steps up");
    cx.simulate_keystrokes("right right");
    assert_eq!(value(cx), SliderValue::Range(5., 79.), "Right steps down");
    cx.simulate_keystrokes("up");
    assert_eq!(value(cx), SliderValue::Range(5., 80.), "Up steps up");
    cx.simulate_keystrokes("down");
    assert_eq!(value(cx), SliderValue::Range(5., 79.), "Down steps down");
}

/// T6 (b): right to left, the switch is laid out from the right, as
/// gpui-component's (gpui-component 0.10.0 switch.rs, `label_side`,
/// `inset_s`): the thumb where its left-to-right test has it, mirrored, off
/// flush with the track's right end and on with its left, and the label
/// `LABEL_GAP` to the track's left.
#[gpui::test]
fn right_to_left_the_switch_runs_from_the_right(cx: &mut TestAppContext) {
    cx.update(|cx| gpui_base::LayoutDirection::Rtl.set_global(cx));
    for checked in [false, true] {
        let cx = window_with(
            cx,
            Some(("kde-breeze", ColorMode::Light)),
            Box::new(move |_, _| {
                Switch::new("s")
                    .label("On")
                    .checked(checked)
                    .into_any_element()
            }),
        );
        let track = bounds(cx, "native-switch-track");
        let thumb = bounds(cx, "native-switch-thumb");
        if checked {
            assert_eq!(thumb.left(), track.left(), "on: flush with the left end");
        } else {
            assert_eq!(
                thumb.right(),
                track.right(),
                "off: flush with the right end"
            );
        }
        let gap = cx.update(|window, _| super::switch::LABEL_GAP.to_pixels(window.rem_size()));
        let label = bounds(cx, "native-switch-label");
        assert_eq!(
            track.left() - label.right(),
            gap,
            "checked {checked}: the label LABEL_GAP to the track's left"
        );
    }
}

/// T6 (b): right to left, the fill runs from the bar's right end, as
/// gpui-component's (gpui-component 0.10.0 progress/progress.rs, `inset_s`):
/// a value's fill as wide as the value from it, and a loading fill sweeping
/// from it, its start held at the right end over the first half turn
/// (`loading_span`).
#[gpui::test]
fn right_to_left_the_progress_bar_fills_from_the_right(cx: &mut TestAppContext) {
    cx.update(|cx| gpui_base::LayoutDirection::Rtl.set_global(cx));
    {
        let value = 40.;
        let cx = window_with(
            cx,
            Some(("kde-breeze", ColorMode::Light)),
            Box::new(move |_, _| ProgressBar::new("p").value(value).into_any_element()),
        );
        let bar = bounds(cx, "native-progress");
        let fill = bounds(cx, "native-progress-fill");
        assert_eq!(fill.right(), bar.right(), "from the right end");
        assert_eq!(
            fill.size.width,
            bar.size.width * (value / 100.),
            "as wide as the value"
        );
    }
    let (cx, _) = loading_window(cx);
    let bar = bounds(cx, "native-progress");
    fill_after(cx, 0);
    let mut grown = false;
    for _ in 0..10 {
        let fill = fill_after(cx, 20);
        assert_eq!(
            fill.right(),
            bar.right(),
            "the sweep starts at the right end"
        );
        grown |= fill.left() < bar.right();
    }
    assert!(grown, "the fill grows leftward");
}

// ---------------------------------------------------------------------------
// gpui-component's own band under the connector's `Root` plugin (D4)
// ---------------------------------------------------------------------------

/// gpui-component's focus band: `FOCUS_RING_WIDTH` outside a focused
/// control, in `ring` with its alpha set to `FOCUS_RING_OPACITY`
/// (gpui-component 0.10.0 `src/styled.rs:11-12`).
#[cfg(feature = "gpui-fast")]
const UPSTREAM_BAND_WIDTH: f32 = 3.;
#[cfg(feature = "gpui-fast")]
const UPSTREAM_BAND_ALPHA: f32 = 0.5;

/// An upstream `Input`, `Button` and `Switch`, one under another; the view
/// calls nothing of the connector's but the button's geometry.
struct UpstreamControls {
    input: Entity<gpui_component::input::InputState>,
    button: gpui::FocusHandle,
}

impl Render for UpstreamControls {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        use crate::ActiveNativeTheme as _;
        use gpui_component::StyledExt as _;
        let Some(n) = cx.native() else {
            return div().into_any_element();
        };
        let button = gpui_component::button::Button::new("band-button")
            .track_focus(&self.button)
            .refine_style(&crate::geometry::button(n))
            .child(
                div()
                    .refine_style(&crate::geometry::button_label(n))
                    .child("Apply"),
            );
        div()
            .size_full()
            .p(px(40.))
            .flex()
            .flex_col()
            .items_start()
            .gap(px(40.))
            .child(
                div()
                    .debug_selector(|| "band-field".into())
                    .w(px(160.))
                    .child(gpui_component::input::Input::new(&self.input)),
            )
            .child(div().debug_selector(|| "band-button".into()).child(button))
            .child(
                div()
                    .debug_selector(|| "band-switch".into())
                    .child(gpui_component::switch::Switch::new("band-switch").checked(false)),
            )
            .into_any_element()
    }
}

/// A window of [`UpstreamControls`] inside gpui-component's `Root`, made
/// when this is called, and the field's and the button's focus handles.
struct BandWindow {
    window: gpui::AnyWindowHandle,
    field: gpui::FocusHandle,
    button: gpui::FocusHandle,
}

fn band_window(cx: &mut TestAppContext) -> Option<BandWindow> {
    let button = cx.update(|cx| cx.focus_handle());
    let b = button.clone();
    let field = Rc::new(RefCell::new(None));
    let f = field.clone();
    let handle = cx.add_window(move |window, cx| {
        let input = cx.new(|cx| gpui_component::input::InputState::new(window, cx));
        *f.borrow_mut() = Some(gpui::Focusable::focus_handle(input.read(cx), cx));
        let view = cx.new(|_| UpstreamControls { input, button: b });
        gpui_component::Root::new(view, window, cx)
    });
    let field = field.borrow_mut().take()?;
    Some(BandWindow {
        window: handle.into(),
        field,
        button,
    })
}

/// `preset`'s light variant installed, as an application installs it.
fn apply_light(cx: &mut TestAppContext, preset: &str) {
    cx.update(|cx| {
        gpui_component::init(cx);
        let r = resolved(preset, ColorMode::Light);
        let prefs = AccessibilityPreferences::default();
        let theme = crate::to_theme(&r, preset, false, &prefs);
        crate::apply(theme, &r, &prefs, cx);
    });
}

fn activate(cx: &mut VisualTestContext) {
    cx.update(|window, _| window.activate_window());
    cx.run_until_parked();
}

fn band_flag(cx: &mut VisualTestContext) -> bool {
    cx.update(|_, cx| gpui_component::Theme::global(cx).focus_ring)
}

/// The bounds, in logical pixels, of every quad the last frame painted as
/// gpui-component's band: bordered `UPSTREAM_BAND_WIDTH` on every side in
/// `ring` with its alpha set to `UPSTREAM_BAND_ALPHA`.
#[cfg(feature = "gpui-fast")]
fn painted_bands(cx: &mut VisualTestContext) -> Vec<Bounds<Pixels>> {
    use gpui_component::ActiveTheme as _;
    cx.update(|window, cx| {
        let scale = window.scale_factor();
        let rgba8 = |colour: gpui::Hsla| {
            let rgba = gpui::Rgba::from(colour);
            let channel = |v: f32| (v.clamp(0., 1.) * 255.).round() as u8;
            [
                channel(rgba.r),
                channel(rgba.g),
                channel(rgba.b),
                channel(rgba.a),
            ]
        };
        let band = rgba8(cx.theme().ring.alpha(UPSTREAM_BAND_ALPHA));
        scene_quads(window)
            .into_iter()
            .filter(|q| {
                let w = &q.border_widths;
                [w.top, w.right, w.bottom, w.left]
                    .iter()
                    .all(|w| near(w.0 / scale, UPSTREAM_BAND_WIDTH))
                    && rgba8(q.border_color) == band
            })
            .map(|q| Bounds {
                origin: point(
                    px(q.bounds.origin.x.0 / scale),
                    px(q.bounds.origin.y.0 / scale),
                ),
                size: gpui::size(
                    px(q.bounds.size.width.0 / scale),
                    px(q.bounds.size.height.0 / scale),
                ),
            })
            .collect()
    })
}

/// After `step`: `Theme::focus_ring` is `on`, and the last frame painted
/// the band as [`assert_painted_band`] says.
fn assert_band(cx: &mut VisualTestContext, on: bool, round: Option<&'static str>, step: &str) {
    assert_eq!(band_flag(cx), on, "{step}: Theme::focus_ring");
    assert_painted_band(cx, round, step);
}

/// On GPUI Fast, whose scene holds the painted quads: the last frame painted
/// the band round the control `round` names and nowhere else, or none where
/// `round` is `None`.
fn assert_painted_band(cx: &mut VisualTestContext, round: Option<&'static str>, step: &str) {
    #[cfg(feature = "gpui-fast")]
    {
        let bands = painted_bands(cx);
        match round {
            None => assert!(bands.is_empty(), "{step}: a band: {bands:?}"),
            Some(selector) => {
                let want = bounds(cx, selector).dilate(px(UPSTREAM_BAND_WIDTH));
                let edges = |b: &Bounds<Pixels>| {
                    [b.origin.x, b.origin.y, b.size.width, b.size.height].map(f32::from)
                };
                let at = |b: &Bounds<Pixels>| {
                    edges(b)
                        .iter()
                        .zip(edges(&want))
                        .all(|(got, want)| near(*got, want))
                };
                assert!(
                    !bands.is_empty() && bands.iter().all(at),
                    "{step}: the band round {selector} ({want:?}): {bands:?}"
                );
            }
        }
    }
    #[cfg(not(feature = "gpui-fast"))]
    let _ = (cx, round, step);
}

fn focused_is(cx: &mut VisualTestContext, handle: &gpui::FocusHandle) -> bool {
    cx.update(|window, cx| window.focused(cx).as_ref() == Some(handle))
}

/// The pointer moved onto the centre of `selector` and its left button
/// pressed there.
fn press(cx: &mut VisualTestContext, selector: &'static str) -> gpui::Point<Pixels> {
    let at = centre(bounds(cx, selector));
    cx.simulate_mouse_move(at, None, Modifiers::default());
    cx.simulate_mouse_down(at, gpui::MouseButton::Left, Modifiers::default());
    at
}

fn release(cx: &mut VisualTestContext, at: gpui::Point<Pixels>) {
    cx.simulate_mouse_up(at, gpui::MouseButton::Left, Modifiers::default());
}

fn click_on(cx: &mut VisualTestContext, selector: &'static str) {
    let at = press(cx, selector);
    release(cx, at);
}

/// D4, decision 18: under kde-breeze, which says keyboard only, the plugin
/// `apply` registers on gpui-base's `Root` turns gpui-component's band on
/// for a focus that followed a key and off for a click's, in a `Root` made
/// after `apply` whose view calls nothing; gpui-component's `Root` binds Tab
/// and Shift+Tab (gpui-base 0.10.0 `src/root.rs:44-45`). Under adwaita,
/// which leaves the field unset, the plugin writes nothing: the band stays
/// as `apply` set it, or as the application wrote it after.
#[gpui::test]
fn upstreams_band_shows_after_a_key_only_where_the_theme_says_so(cx: &mut TestAppContext) {
    apply_light(cx, "kde-breeze");
    let opened = band_window(cx);
    assert!(opened.is_some(), "the field has a focus handle");
    let Some(w) = opened else {
        return;
    };
    let cx = &mut VisualTestContext::from_window(w.window, cx);
    activate(cx);
    assert_band(cx, false, None, "start");

    let at = press(cx, "band-field");
    assert_band(cx, false, None, "the mouse-down into the field");
    release(cx, at);
    assert!(focused_is(cx, &w.field), "the click focused the field");
    assert_band(cx, false, None, "a click into the field");

    cx.simulate_keystrokes("tab");
    assert!(focused_is(cx, &w.button), "Tab focused the button");
    assert_band(cx, true, Some("band-button"), "Tab");

    cx.simulate_keystrokes("tab");
    assert!(
        !focused_is(cx, &w.field) && !focused_is(cx, &w.button),
        "Tab moved the focus on to the switch"
    );
    assert_band(cx, true, Some("band-switch"), "Tab to the switch");

    let over_button = centre(bounds(cx, "band-button"));
    cx.simulate_mouse_move(over_button, None, Modifiers::default());
    assert_band(cx, true, Some("band-switch"), "a mouse move");

    click_on(cx, "band-field");
    assert_band(cx, false, None, "a click into the field again");

    cx.simulate_keystrokes("tab");
    assert_band(cx, true, Some("band-button"), "Tab from the field");
    click_on(cx, "band-switch");
    assert_band(cx, false, None, "a click on the switch");

    cx.simulate_keystrokes("shift-tab");
    assert!(focused_is(cx, &w.button), "Shift+Tab focused the button");
    assert_band(cx, true, Some("band-button"), "Shift+Tab");

    apply_light(cx, "adwaita");
    let opened = band_window(cx);
    assert!(opened.is_some(), "the field has a focus handle");
    let Some(w) = opened else {
        return;
    };
    let cx = &mut VisualTestContext::from_window(w.window, cx);
    activate(cx);
    assert_band(cx, true, None, "adwaita: start");
    click_on(cx, "band-field");
    assert_band(
        cx,
        true,
        Some("band-field"),
        "adwaita: a click into the field",
    );
    cx.simulate_keystrokes("tab");
    assert_band(cx, true, Some("band-button"), "adwaita: Tab");

    apply_light(cx, "adwaita");
    cx.update(|_, cx| gpui_component::Theme::global_mut(cx).focus_ring = false);
    let opened = band_window(cx);
    assert!(opened.is_some(), "the field has a focus handle");
    let Some(w) = opened else {
        return;
    };
    let cx = &mut VisualTestContext::from_window(w.window, cx);
    activate(cx);
    cx.simulate_keystrokes("tab");
    assert_band(
        cx,
        false,
        None,
        "adwaita, the band written off after apply: Tab",
    );
}

/// Decision 18: each window's `Root` sets `Theme::focus_ring` for its own
/// window when it renders, so two windows under kde-breeze paint each its
/// own answer. The flag, which is the window's whose `Root` rendered last, is
/// asserted only right after a window's own render, never between the two.
#[gpui::test]
fn each_windows_root_sets_the_band_for_its_window(cx: &mut TestAppContext) {
    apply_light(cx, "kde-breeze");
    let (one, two) = (band_window(cx), band_window(cx));
    assert!(
        one.is_some() && two.is_some(),
        "the fields have focus handles"
    );
    let (Some(one), Some(two)) = (one, two) else {
        return;
    };
    let mut v1 = VisualTestContext::from_window(one.window, cx);
    let mut v2 = VisualTestContext::from_window(two.window, cx);

    activate(&mut v1);
    click_on(&mut v1, "band-field");
    v1.simulate_keystrokes("tab");
    assert!(
        focused_is(&mut v1, &one.button),
        "window 1: Tab focused the button"
    );
    assert_band(
        &mut v1,
        true,
        Some("band-button"),
        "window 1: a click, then Tab",
    );

    activate(&mut v2);
    click_on(&mut v2, "band-field");
    assert!(
        focused_is(&mut v2, &two.field),
        "window 2: the click focused the field"
    );
    assert_band(&mut v2, false, None, "window 2: a click into the field");

    redraw(&mut v1);
    assert!(
        band_flag(&mut v1),
        "window 1 drawn again: Theme::focus_ring"
    );
    assert_painted_band(&mut v1, Some("band-button"), "window 1 drawn again");
    redraw(&mut v2);
    assert!(
        !band_flag(&mut v2),
        "window 2 drawn again: Theme::focus_ring"
    );
    assert_painted_band(&mut v2, None, "window 2 drawn again");
}

// ---------------------------------------------------------------------------
// The bordered button against the Default one
// ---------------------------------------------------------------------------

/// A colour as 8-bit sRGBA channels: the precision a frame is drawn in.
fn rgba8(colour: gpui::Hsla) -> [u8; 4] {
    let rgba = gpui::Rgba::from(colour);
    let channel = |v: f32| (v.clamp(0., 1.) * 255.).round() as u8;
    [
        channel(rgba.r),
        channel(rgba.g),
        channel(rgba.b),
        channel(rgba.a),
    ]
}

fn hex(c: [u8; 4]) -> String {
    format!("#{:02x}{:02x}{:02x}{:02x}", c[0], c[1], c[2], c[3])
}

/// What one quad paints, its bounds relative to the button's origin.
#[derive(Clone)]
struct QuadLook {
    at: [f32; 4],
    fill: String,
    raw_fill: Option<gpui::Hsla>,
    border: [u8; 4],
    raw_border: gpui::Hsla,
    widths: [f32; 4],
    radii: [f32; 4],
}

impl QuadLook {
    fn of(q: &gpui::Quad, origin: gpui::Point<gpui::ScaledPixels>) -> Self {
        let raw_fill = q.background.as_solid();
        Self {
            at: [
                q.bounds.origin.x.0 - origin.x.0,
                q.bounds.origin.y.0 - origin.y.0,
                q.bounds.size.width.0,
                q.bounds.size.height.0,
            ],
            fill: raw_fill.map_or_else(|| format!("{:?}", q.background), |f| hex(rgba8(f))),
            raw_fill,
            border: rgba8(q.border_color),
            raw_border: q.border_color,
            widths: [
                q.border_widths.top.0,
                q.border_widths.right.0,
                q.border_widths.bottom.0,
                q.border_widths.left.0,
            ],
            radii: [
                q.corner_radii.top_left.0,
                q.corner_radii.top_right.0,
                q.corner_radii.bottom_right.0,
                q.corner_radii.bottom_left.0,
            ],
        }
    }

    fn line(&self) -> String {
        format!(
            "at={:?} fill={} border={} widths={:?} radii={:?}",
            self.at,
            self.fill,
            hex(self.border),
            self.widths,
            self.radii
        )
    }
}

/// The quads of the last frame whose bounds are the button's, and those
/// inside its bounds grown by 12 px (a focus band, a focus line), each in
/// paint order.
type ButtonQuads = (Vec<QuadLook>, Vec<QuadLook>);

fn button_quads(cx: &mut VisualTestContext, selector: &'static str) -> ButtonQuads {
    let b = bounds(cx, selector);
    cx.update(|window, _| {
        let scale = window.scale_factor();
        let b = b.scale(scale);
        let pad = 12. * scale;
        let mut quads = scene_quads(window);
        quads.sort_by_key(|q| q.order);
        let (mut own, mut near) = (Vec::new(), Vec::new());
        for q in &quads {
            let look = QuadLook::of(q, b.origin);
            let inside = q.bounds.origin.x.0 >= b.origin.x.0 - pad
                && q.bounds.origin.y.0 >= b.origin.y.0 - pad
                && q.bounds.origin.x.0 + q.bounds.size.width.0
                    <= b.origin.x.0 + b.size.width.0 + pad
                && q.bounds.origin.y.0 + q.bounds.size.height.0
                    <= b.origin.y.0 + b.size.height.0 + pad;
            if q.bounds == b {
                own.push(look);
            } else if inside {
                near.push(look);
            }
        }
        (own, near)
    })
}

/// The button under `selector` hovered, then pressed, each frame read
/// before the pointer moves on; the pointer is left away from both.
fn hover_and_press(
    cx: &mut VisualTestContext,
    selector: &'static str,
    away: gpui::Point<Pixels>,
) -> (ButtonQuads, ButtonQuads) {
    let at = centre(bounds(cx, selector));
    cx.simulate_mouse_move(at, None, Modifiers::default());
    redraw(cx);
    let hovered = button_quads(cx, selector);
    cx.simulate_mouse_down(at, gpui::MouseButton::Left, Modifiers::default());
    redraw(cx);
    let pressed = button_quads(cx, selector);
    cx.simulate_mouse_up(at, gpui::MouseButton::Left, Modifiers::default());
    cx.simulate_mouse_move(away, None, Modifiers::default());
    redraw(cx);
    (hovered, pressed)
}

/// `handle` given the focus by `focus_next`, at most four steps; whether it
/// took it.
fn focus_next_to(cx: &mut VisualTestContext, handle: &gpui::FocusHandle) -> bool {
    for _ in 0..4 {
        if cx.update(|window, _| handle.is_focused(window)) {
            return true;
        }
        focus_next(cx);
    }
    cx.update(|window, _| handle.is_focused(window))
}

/// Whether the quad has a border width above 0.
fn has_border(q: &QuadLook) -> bool {
    q.widths.iter().any(|w| *w > 0.)
}

/// Whether the quad's fill is its own border colour at alpha 0, as gpui
/// paints a border (gpui-pre 0.3.12 src/style.rs:746-756).
fn carries_border(q: &QuadLook) -> bool {
    q.fill == hex([q.border[0], q.border[1], q.border[2], 0])
}

/// How the two buttons are built: at rest (hovered, pressed and focused
/// from there), selected or disabled.
#[derive(Clone, Copy)]
enum BuiltAs {
    Rest,
    Selected,
    Disabled,
}

/// The `Default` button and the `bordered_button` one, both reading "Apply"
/// and refined by `geometry::button`, as `built` asks, 40 px apart. Nothing
/// is drawn where no native theme or no hover border is installed.
fn default_and_bordered(built: BuiltAs, handles: (gpui::FocusHandle, gpui::FocusHandle)) -> Build {
    use crate::ActiveNativeTheme as _;
    use gpui_component::button::{Button, ButtonVariants as _};
    use gpui_component::{ActiveTheme as _, Disableable as _, Selectable as _, StyledExt as _};
    Box::new(move |_, cx| {
        let Some(n) = cx.native() else {
            return div().into_any_element();
        };
        let Some(variant) = crate::variants::bordered_button(n, cx) else {
            return div().into_any_element();
        };
        let label = || {
            div()
                .min_w_0()
                .whitespace_nowrap()
                .text_ellipsis()
                .refine_style(&crate::geometry::button_label(n))
                .child("Apply")
        };
        let default = Button::new("default")
            .debug_selector(|| "default-button".into())
            .track_focus(&handles.0)
            .refine_style(&crate::geometry::button(n));
        let bordered = Button::new("bordered")
            .debug_selector(|| "bordered-button".into())
            .track_focus(&handles.1)
            .custom(variant)
            .refine_style(&crate::geometry::button(n));
        let (default, bordered) = match built {
            BuiltAs::Rest => (default, bordered.bg(cx.theme().button)),
            BuiltAs::Selected => (
                default
                    .selected(true)
                    .refine_style(&crate::geometry::button_checked(n)),
                bordered
                    .selected(true)
                    .refine_style(&crate::geometry::button_checked(n)),
            ),
            BuiltAs::Disabled => (
                default
                    .disabled(true)
                    .refine_style(&crate::geometry::button_disabled(n)),
                bordered
                    .bg(cx.theme().button)
                    .disabled(true)
                    .refine_style(&crate::geometry::button_disabled(n)),
            ),
        };
        div()
            .pt(px(40.))
            .pl(px(20.))
            .flex()
            .flex_col()
            .gap(px(40.))
            .child(default.child(label()).accessibility_label("Apply"))
            .child(bordered.child(label()).accessibility_label("Apply"))
            .into_any_element()
    })
}

/// docs/todo_v0.7.2_kde-focus-ring-and-button-outlines-spec.md, 3F.2 and
/// 3F.3: in every state the `bordered_button` paints what the `Default`
/// button paints, but the hovered and pressed border, which is the theme's
/// `button.hover_border_color` where the `Default` one draws `input`.
#[gpui::test]
fn bordered_button_matches_the_default_button_but_its_hover_border(cx: &mut TestAppContext) {
    use gpui_component::ActiveTheme as _;
    use gpui_component::button::ButtonCustomVariant;
    let red = native_theme::color::Rgba::rgb(255, 0, 0);
    let away = point(px(500.), px(400.));
    let (default_sel, bordered_sel) = ("default-button", "bordered-button");
    let mut unexpected: Vec<String> = Vec::new();
    let mut expected_seen: Vec<String> = Vec::new();
    let mut vanished: Vec<String> = Vec::new();
    let mut border_fills_checked = 0usize;
    // The third field: whether gpui-component's band is on for any focus
    // (`Theme::focus_ring`); kde-breeze and windows-11 say keyboard only, so
    // outside a `Root` the band stays off there.
    for (preset, set, band_on) in [
        ("kde-breeze", None, false),
        ("windows-11", Some(red), false),
        ("macos-tahoe", Some(red), true),
    ] {
        for mode in [ColorMode::Light, ColorMode::Dark] {
            let combo = format!("{preset} {mode:?}");
            // Each state's (Default, bordered) quads.
            let mut states: Vec<(&str, ButtonQuads, ButtonQuads)> = Vec::new();
            let mut hover = None;
            let mut input = None;
            for built in [BuiltAs::Rest, BuiltAs::Selected, BuiltAs::Disabled] {
                let handles = cx.update(|cx| (cx.focus_handle(), cx.focus_handle()));
                let edit = move |r: &mut ResolvedTheme| {
                    if let Some(c) = set {
                        r.button.hover_border_color = Some(c);
                    }
                };
                let vcx = window_with_resolved(
                    cx,
                    preset,
                    mode,
                    edit,
                    default_and_bordered(built, handles.clone()),
                );
                vcx.simulate_mouse_move(away, None, Modifiers::default());
                redraw(vcx);
                let name = match built {
                    BuiltAs::Rest => "rest",
                    BuiltAs::Selected => "selected",
                    BuiltAs::Disabled => "disabled",
                };
                if !matches!(built, BuiltAs::Rest) {
                    let d = button_quads(vcx, default_sel);
                    let b = button_quads(vcx, bordered_sel);
                    states.push((name, d, b));
                    continue;
                }
                let read = vcx.update(|_, cx| {
                    use crate::ActiveNativeTheme as _;
                    let n = cx.native()?;
                    let theme = cx.theme();
                    let h = crate::variants::hover_border(n)?;
                    let want = ButtonCustomVariant::new(cx)
                        .color(h)
                        .foreground(theme.button_foreground)
                        .hover(theme.button_hover)
                        .active(theme.button_active)
                        .shadow(false);
                    let got = crate::variants::bordered_button(n, cx);
                    let other_fg = want.foreground(theme.muted_foreground);
                    let distinct =
                        theme.muted_foreground == theme.button_foreground || got != Some(other_fg);
                    Some((
                        h,
                        theme.input,
                        got == Some(want),
                        distinct,
                        theme.focus_ring,
                    ))
                });
                let Some((h, i, text_ok, text_distinct, band)) = read else {
                    unexpected.push(format!(
                        "{combo}: no native theme or no hover border on the resolved theme"
                    ));
                    continue;
                };
                hover = Some(h);
                input = Some(i);
                if band != band_on {
                    unexpected.push(format!(
                        "{combo}: Theme::focus_ring is {band}, the combination's band is {}",
                        if band_on { "on" } else { "off" }
                    ));
                }
                if !text_ok {
                    unexpected.push(format!(
                        "{combo}: the variant is not color(hover_border).foreground(button_foreground).hover(button_hover).active(button_active).shadow(false)"
                    ));
                }
                if !text_distinct {
                    unexpected.push(format!(
                        "{combo}: the foreground is not part of the variant's equality"
                    ));
                }
                println!(
                    "{combo} | hover_border_color={} input={} button_foreground: {} | Theme::focus_ring={band}",
                    hex(rgba8(h)),
                    hex(rgba8(i)),
                    if text_ok { "equal" } else { "NOT equal" }
                );
                let d = button_quads(vcx, default_sel);
                let b = button_quads(vcx, bordered_sel);
                states.push(("rest", d, b));

                let (d_hovered, d_pressed) = hover_and_press(vcx, default_sel, away);
                let (b_hovered, b_pressed) = hover_and_press(vcx, bordered_sel, away);
                states.push(("hovered", d_hovered, b_hovered));
                states.push(("pressed", d_pressed, b_pressed));

                let before = vcx.update(|window, cx| window.focused(cx));
                activate_key(vcx, "x");
                let after = vcx.update(|window, cx| window.focused(cx));
                assert!(before == after, "{combo}: the key moved the focus");
                assert!(
                    focus_next_to(vcx, &handles.0),
                    "{combo}: focus_next never reached the Default button"
                );
                redraw(vcx);
                let d_focused = button_quads(vcx, default_sel);
                assert!(
                    focus_next_to(vcx, &handles.1),
                    "{combo}: focus_next never reached the bordered button"
                );
                redraw(vcx);
                let b_focused = button_quads(vcx, bordered_sel);
                states.push(("focused", d_focused, b_focused));
            }
            let hover8 = hover.map(rgba8);
            let input8 = input.map(rgba8);
            for (state, d, b) in &states {
                for (name, (own, near)) in [("default", d), ("bordered", b)] {
                    for (i, q) in own.iter().enumerate() {
                        println!("{combo} | {state:8} | {name:8} | own[{i}]  {}", q.line());
                    }
                    for (i, q) in near.iter().enumerate() {
                        println!("{combo} | {state:8} | {name:8} | near[{i}] {}", q.line());
                    }
                }
                let border_state = matches!(*state, "hovered" | "pressed");
                for (part, dq, bq) in [("own", &d.0, &b.0), ("near", &d.1, &b.1)] {
                    if dq.len() != bq.len() {
                        unexpected.push(format!(
                            "{combo} | {state} | {part}: Default paints {} quads, bordered {}",
                            dq.len(),
                            bq.len()
                        ));
                    }
                    if part == "own" && border_state {
                        let shown = bq
                            .iter()
                            .any(|q| Some(q.border) == hover8 && q.widths.iter().any(|w| *w > 0.));
                        if !shown {
                            unexpected.push(format!(
                                "{combo} | {state} | own: the bordered button does not draw hover_border_color"
                            ));
                        }
                    }
                    // Exception 2: a quad with a border is painted filled with
                    // its own border colour at alpha 0; asserted on every such
                    // quad of both buttons, and its fill is not compared.
                    for (name, quads) in [("Default", dq), ("bordered", bq)] {
                        for (i, q) in quads.iter().enumerate() {
                            if !has_border(q) {
                                continue;
                            }
                            border_fills_checked += 1;
                            if !carries_border(q) {
                                unexpected.push(format!(
                                    "{combo} | {state} | {part}[{i}] {name}: a quad with a border is filled {}, not its border colour {} at alpha 0",
                                    q.fill,
                                    hex(q.border)
                                ));
                            }
                        }
                    }
                    for (i, (x, y)) in dq.iter().zip(bq.iter()).enumerate() {
                        let at = format!("{combo} | {state} | {part}[{i}]");
                        let mut item = |what: &str, dv: String, bv: String| {
                            unexpected.push(format!("{at} {what}: Default {dv}, bordered {bv}"));
                        };
                        if x.at != y.at {
                            item("bounds", format!("{:?}", x.at), format!("{:?}", y.at));
                        }
                        let fill_compared = !(has_border(x) && has_border(y));
                        if fill_compared && x.fill != y.fill {
                            item("fill", x.fill.clone(), y.fill.clone());
                        }
                        if x.widths != y.widths {
                            item(
                                "border widths",
                                format!("{:?}", x.widths),
                                format!("{:?}", y.widths),
                            );
                        }
                        if x.radii != y.radii {
                            item("radii", format!("{:?}", x.radii), format!("{:?}", y.radii));
                        }
                        // Exception 1: hovered and pressed, the border quads'
                        // colour is asserted on both sides.
                        let designed =
                            part == "own" && border_state && has_border(x) && has_border(y);
                        if designed {
                            let default_ok = Some(x.border) == input8;
                            let bordered_ok = Some(y.border) == hover8;
                            if default_ok && bordered_ok {
                                expected_seen.push(format!(
                                    "{at} border: Default {} (input), bordered {} (hover_border_color)",
                                    hex(x.border),
                                    hex(y.border)
                                ));
                            } else {
                                item(
                                    "hover and press border (want Default input, bordered hover_border_color)",
                                    format!(
                                        "{} (input {})",
                                        hex(x.border),
                                        input8.map_or_else(|| "unread".into(), hex)
                                    ),
                                    format!(
                                        "{} (hover_border_color {})",
                                        hex(y.border),
                                        hover8.map_or_else(|| "unread".into(), hex)
                                    ),
                                );
                            }
                        } else if x.border != y.border {
                            item("border colour", hex(x.border), hex(y.border));
                        } else if x.raw_border != y.raw_border {
                            vanished.push(format!(
                                "{at} border: {:?} against {:?}, equal at 8 bits",
                                x.raw_border, y.raw_border
                            ));
                        }
                        if fill_compared && x.fill == y.fill && x.raw_fill != y.raw_fill {
                            vanished.push(format!(
                                "{at} fill: {:?} against {:?}, equal at 8 bits",
                                x.raw_fill, y.raw_fill
                            ));
                        }
                    }
                }
            }
        }
    }
    println!(
        "--- the hover and press border, as designed ({}):",
        expected_seen.len()
    );
    for line in &expected_seen {
        println!("{line}");
    }
    println!(
        "--- differences that vanish at 8 bits ({}):",
        vanished.len()
    );
    for line in &vanished {
        println!("{line}");
    }
    println!("--- other differences ({}):", unexpected.len());
    for line in &unexpected {
        println!("{line}");
    }
    println!(
        "--- quads with a border whose fill was asserted to be their border colour at alpha 0: {border_fills_checked}"
    );
    assert!(
        unexpected.is_empty(),
        "{} differences beyond the hover and press border:\n{}",
        unexpected.len(),
        unexpected.join("\n")
    );
}

/// A child that records `window.text_style().color` when it renders, which is
/// the moment gpui-component's `Icon` reads the colour it paints in where it
/// states none of its own (gpui-component 0.10.0 `icon.rs:260-266`, through
/// `into_svg`'s `self.text_color.unwrap_or(..)`, `:213-214`).
#[derive(IntoElement)]
struct TextColourProbe {
    rendered: Rc<RefCell<Vec<gpui::Hsla>>>,
}

impl gpui::RenderOnce for TextColourProbe {
    fn render(self, window: &mut Window, _: &mut gpui::App) -> impl IntoElement {
        self.rendered.borrow_mut().push(window.text_style().color);
        div().size(px(1.))
    }
}

/// A checked toolbar Button's icon takes the checked text (decision 14 of the
/// steps 4–5 specification): a selected Button on `variants::toolbar_button`,
/// refined with `geometry::toolbar_button_rest` and then
/// `geometry::toolbar_button_checked` as the showcase's Edit button is,
/// holds an `Icon` with no colour of its own and a probe that records the
/// colour at render, the one the icon is painted in (a test cannot read an
/// icon's painted colour: gpui's test API gives quads and underlines only).
/// Under windows-11 light and dark, each recorded colour is
/// `button.checked_text_color`.
#[gpui::test]
fn a_checked_toolbar_buttons_icon_takes_the_checked_text(cx: &mut TestAppContext) {
    use gpui_component::button::{Button, ButtonVariants as _};
    use gpui_component::{Icon, IconName, Selectable as _, StyledExt as _};

    let mut lines = Vec::new();
    let mut wrong = Vec::new();
    for mode in [ColorMode::Light, ColorMode::Dark] {
        let r = resolved("windows-11", mode);
        let checked_text = r.button.checked_text_color.map(c);
        assert_ne!(
            checked_text,
            Some(c(r.button.font.color)),
            "windows-11 {mode:?}: the checked text equals the variant's foreground \
             (button.font.color), so this case proves nothing"
        );
        let rest = crate::geometry::toolbar_button_rest(crate::Native::unscaled(&r));
        let checked = crate::geometry::toolbar_button_checked(crate::Native::unscaled(&r));
        let rendered = Rc::new(RefCell::new(Vec::new()));
        let into_render = rendered.clone();
        let build: Build = Box::new(move |_, cx| {
            Button::new("edit")
                .custom(crate::variants::toolbar_button(cx))
                .selected(true)
                .refine_style(&rest)
                .refine_style(&checked)
                .child(Icon::new(IconName::Copy))
                .child(TextColourProbe {
                    rendered: into_render.clone(),
                })
                .into_any_element()
        });
        let _ = window_with(cx, Some(("windows-11", mode)), build);
        let recorded = rendered.borrow().last().copied();
        let line = format!(
            "windows-11 {mode:?}: recorded {recorded:?}; checked text {checked_text:?}; renders {}",
            rendered.borrow().len()
        );
        if checked_text.is_none() || recorded != checked_text {
            wrong.push(line.clone());
        }
        lines.push(line);
    }
    for line in &lines {
        println!("{line}");
    }
    assert!(
        wrong.is_empty(),
        "the icon of a checked toolbar button is not lettered button.checked_text_color:\n{}",
        wrong.join("\n")
    );
}
