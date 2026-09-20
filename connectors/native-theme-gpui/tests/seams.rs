//! Rendered seam tests (spec v0.5.9 §6).
//!
//! Each `geometry` builder rests on one upstream behaviour: the widget applies
//! its own size defaults and *then* the caller's style. These tests lay real
//! gpui-component widgets out on GPUI's headless test platform, once without
//! and once with the builder's refinement, and compare the measured height
//! with the refinement's own field. When an upstream release stops honouring
//! a seam, this file fails (the nightly dependency canary runs it).
//!
//! Every test: `u` (unstyled) must differ from `e` (expected), otherwise the
//! input does not discriminate; then `s` (styled) must equal `e`. Text scale
//! 1.5 is the input that takes `control_height` past the presets' minimum.
//!
//! `button`, `input` and `progress` set the same `size.height` their widget
//! sets for itself, so they also prove the ordering of the two writes.
//! `select` and `combobox` set `min_size.height` and `list_item` a height the
//! widget leaves content-driven, so those three prove that the refinement
//! reaches the widget's root box and wins there, not an ordering claim.
//! Every other builder rests on the source citation in its doc comment.

use gpui::{
    AbsoluteLength, AnyElement, AppContext as _, Context, DefiniteLength, InteractiveElement as _,
    IntoElement, Length, ParentElement as _, Pixels, Render, SharedString, Size, StyleRefinement,
    Styled as _, TestAppContext, Window, div, px,
};
use gpui_component::{
    StyledExt as _,
    button::Button,
    combobox::{Combobox, ComboboxState},
    input::{Input, InputState},
    list::ListItem,
    progress::Progress,
    select::{SearchableVec, Select, SelectState},
};
use native_theme::theme::{ColorMode, ResolvedTheme, Theme};
use native_theme::{AccessibilityPreferences, ResolutionContext};
use native_theme_gpui::{Native, apply, geometry, to_theme};

fn scaled() -> AccessibilityPreferences {
    AccessibilityPreferences {
        text_scaling_factor: 1.5,
        ..AccessibilityPreferences::default()
    }
}

/// Resolve a preset without consulting the machine. `ResolutionContext::for_tests`
/// pins the font DPI at 96, so the point sizes the presets declare convert to
/// the same pixels everywhere; `from_preset` would resolve through
/// `ResolutionContext::from_system`, whose DPI is read from the desktop
/// (`native-theme/src/resolve/context.rs:62-83`) and would make the expected
/// height machine-dependent.
fn resolved(preset: &str) -> ResolvedTheme {
    Theme::preset(preset)
        .expect("preset loads")
        .into_variant(ColorMode::Light)
        .expect("light variant")
        .into_resolved(&ResolutionContext::for_tests())
        .expect("preset resolves")
}

type Build = fn(Option<&StyleRefinement>, &mut Window, &mut Context<Harness>) -> AnyElement;

struct Harness {
    style: Option<StyleRefinement>,
    build: Build,
}

impl Render for Harness {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let widget = (self.build)(self.style.as_ref(), window, cx);
        div().size(px(600.)).flex().items_start().child(
            div()
                .debug_selector(|| "probe".into())
                .flex_none()
                .child(widget),
        )
    }
}

fn native_style(preset: &str, build: fn(Native<'_>) -> StyleRefinement) -> StyleRefinement {
    let prefs = scaled();
    let resolved = resolved(preset);
    build(Native {
        resolved: &resolved,
        accessibility: &prefs,
    })
}

fn measure(
    cx: &mut TestAppContext,
    preset: &str,
    style: Option<StyleRefinement>,
    build: Build,
) -> Size<Pixels> {
    let prefs = scaled();
    let resolved = resolved(preset);
    let theme = to_theme(&resolved, preset, false, &prefs);
    cx.update(|cx| {
        gpui_component::init(cx);
        apply(theme, &resolved, &prefs, cx);
    });
    let (_, cx) = cx.add_window_view(|_, _| Harness { style, build });
    cx.update(|window, cx| window.draw(cx).clear(cx));
    cx.debug_bounds("probe").expect("probe was laid out").size
}

fn px_of(length: Option<Length>) -> Pixels {
    match length {
        Some(Length::Definite(DefiniteLength::Absolute(AbsoluteLength::Pixels(p)))) => p,
        other => panic!("the builder no longer sets an absolute length: {other:?}"),
    }
}

fn assert_seam(u: Pixels, s: Pixels, e: Pixels, what: &str) {
    assert_ne!(
        u, e,
        "{what}: the unstyled widget already measures the native value; this input proves nothing, pick another preset"
    );
    assert_eq!(
        s, e,
        "{what}: upstream no longer honours the caller's style"
    );
}

fn styled<T: gpui::Styled + IntoElement>(w: T, style: Option<&StyleRefinement>) -> AnyElement {
    match style {
        Some(style) => w.refine_style(style).into_any_element(),
        None => w.into_any_element(),
    }
}

fn button(s: Option<&StyleRefinement>, _: &mut Window, _: &mut Context<Harness>) -> AnyElement {
    styled(Button::new("b").label("OK"), s)
}
fn input(s: Option<&StyleRefinement>, w: &mut Window, cx: &mut Context<Harness>) -> AnyElement {
    let state = cx.new(|cx| InputState::new(w, cx));
    styled(Input::new(&state), s)
}
fn select(s: Option<&StyleRefinement>, w: &mut Window, cx: &mut Context<Harness>) -> AnyElement {
    let items: Vec<SharedString> = vec!["a".into(), "b".into()];
    let state = cx.new(|cx| SelectState::new(SearchableVec::new(items), None, w, cx));
    styled(Select::new(&state), s)
}
fn combobox(s: Option<&StyleRefinement>, w: &mut Window, cx: &mut Context<Harness>) -> AnyElement {
    let items: Vec<SharedString> = vec!["a".into(), "b".into()];
    let state = cx.new(|cx| ComboboxState::new(SearchableVec::new(items), vec![], w, cx));
    styled(Combobox::new(&state), s)
}
fn list_item(s: Option<&StyleRefinement>, _: &mut Window, _: &mut Context<Harness>) -> AnyElement {
    styled(ListItem::new("i").child("x"), s)
}
fn progress(s: Option<&StyleRefinement>, _: &mut Window, _: &mut Context<Harness>) -> AnyElement {
    styled(Progress::new("p"), s)
}

macro_rules! seam {
    ($name:ident, $preset:literal, $build:expr, $geom:expr, $field:ident . $sub:ident) => {
        #[gpui::test]
        fn $name(cx: &mut TestAppContext) {
            let style = native_style($preset, $geom);
            let e = px_of(style.$field.$sub);
            let u = measure(cx, $preset, None, $build).height;
            let s = measure(cx, $preset, Some(style), $build).height;
            assert_seam(u, s, e, stringify!($name));
        }
    };
}
seam!(
    button_takes_the_native_height,
    "kde-breeze",
    button,
    geometry::button,
    size.height
);
seam!(
    input_takes_the_native_height,
    "kde-breeze",
    input,
    geometry::input,
    size.height
);
// kde-breeze's combo box minimum equals upstream's own height; adwaita's is larger.
seam!(
    select_takes_the_native_min_height,
    "adwaita",
    select,
    geometry::select,
    min_size.height
);
// Same refinement as `select`, but a different upstream seam (`combobox.rs`).
seam!(
    combobox_takes_the_native_min_height,
    "adwaita",
    combobox,
    geometry::combobox,
    min_size.height
);
seam!(
    list_item_takes_the_native_height,
    "kde-breeze",
    list_item,
    geometry::list_item,
    size.height
);
seam!(
    progress_takes_the_native_height,
    "kde-breeze",
    progress,
    geometry::progress,
    size.height
);
