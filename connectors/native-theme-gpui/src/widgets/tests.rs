//! Widget tests (docs/todo_gpui-widgets-spec.md §4): every leaf a widget
//! paints is the one the spec names, the drawn boxes measure the stated
//! sizes, and the controls behave as gpui-component's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

use std::cell::Cell;
use std::rc::Rc;

use gpui::{
    AnyElement, AppContext as _, Bounds, Context, Entity, InteractiveElement as _, IntoElement,
    Modifiers, ParentElement as _, Pixels, Render, Styled as _, TestAppContext, VisualTestContext,
    Window, div, point, px,
};
use gpui_base::slider::SliderState;
use native_theme::theme::{ColorMode, ResolvedTheme, Theme};
use native_theme::{AccessibilityPreferences, ResolutionContext};

use super::*;

/// The two presets the tests draw under: a light desktop theme and a dark
/// one whose sizes all differ from it.
const PRESETS: [(&str, ColorMode); 2] = [
    ("kde-breeze", ColorMode::Light),
    ("material", ColorMode::Dark),
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
            Some(unchecked.blend(c(k.hover_background.unwrap_or(k.background_color)))),
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

        let disabled = at(true, true);
        assert_eq!(
            disabled.fill,
            c(k.disabled_background.unwrap_or(k.background_color)),
            "{preset}"
        );
        assert_eq!(disabled.mark, c(k.disabled_text_color), "{preset}");
        assert_eq!(disabled.label, c(k.disabled_text_color), "{preset}");
        assert_eq!(disabled.hover_fill, None, "{preset}");
    }
}

#[test]
fn the_checkbox_look_is_the_presets_values() {
    // The values the audit measured against (SC AUDIT-gpui, the dumps
    // kbl.flat / md.flat): the mapping reaches the stated numbers.
    let kbl = CheckboxLook::of(&resolved("kde-breeze", ColorMode::Light), false, false).unwrap();
    assert_eq!(kbl.indicator, px(20.));
    let md = CheckboxLook::of(&resolved("material", ColorMode::Dark), false, false).unwrap();
    assert_eq!(md.indicator, px(18.));
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
            Some(c(s.checked_background).blend(c(
                s.hover_checked_background.unwrap_or(s.checked_background)
            ))),
            "{preset}"
        );
        assert_eq!(on.thumb_color, c(s.thumb_background), "{preset}");
        assert_eq!(on.label, c(r.defaults.text_color), "{preset}");
        assert_eq!(
            on.thumb_left(true),
            on.track_width - on.inset - on.thumb,
            "{preset}"
        );
        assert_eq!(on.thumb_left(false), on.inset, "{preset}");

        let off = SwitchLook::of(&r, false, false).expect("finite");
        assert_eq!(off.track, c(s.unchecked_background), "{preset}");

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
    }
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
            Some(c(s.thumb_color).blend(c(s.thumb_hover_color.unwrap_or(s.thumb_color)))),
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
    cx.update(|window, cx| window.draw(cx).clear(cx));
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
        Some(("material", ColorMode::Dark)),
        Box::new(|_, _| Radio::new("r").label("A").into_any_element()),
    );
    let r = resolved("material", ColorMode::Dark);
    let b = bounds(cx, "native-checkbox-indicator");
    assert_eq!(b.size.width, px(r.checkbox.indicator_width));
    assert_eq!(b.size.height, px(r.checkbox.indicator_width));
}

#[gpui::test]
fn the_switch_track_and_thumb_are_the_stated_sizes(cx: &mut TestAppContext) {
    let cx = window_with(
        cx,
        Some(("material", ColorMode::Dark)),
        Box::new(|_, _| {
            Switch::new("s")
                .label("On")
                .checked(true)
                .into_any_element()
        }),
    );
    let r = resolved("material", ColorMode::Dark);
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

#[gpui::test]
fn a_disabled_slider_is_inert(cx: &mut TestAppContext) {
    let (cx, state) = slider_window(cx, true);
    let root = bounds(cx, "native-slider");
    cx.simulate_click(root.origin + point(px(4.), px(4.)), Modifiers::default());
    cx.update(|window, cx| window.focus_next(cx));
    cx.simulate_keystrokes("right");
    assert_eq!(slider_value(cx, &state), 50.);
}
