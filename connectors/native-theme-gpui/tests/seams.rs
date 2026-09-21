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

use std::num::NonZeroU32;

use gpui::{
    AbsoluteLength, AnyElement, AppContext as _, Bounds, Context, DefiniteLength, ImageSource,
    InteractiveElement as _, IntoElement, Length, ParentElement as _, Pixels, Render, SharedString,
    Size, StyleRefinement, Styled as _, TestAppContext, Window, div, px,
};
use gpui_component::{
    StyledExt as _,
    button::Button,
    combobox::{Combobox, ComboboxState},
    input::{Input, InputState},
    list::ListItem,
    progress::Progress,
    select::{SearchableVec, Select, SelectState},
    tooltip::Tooltip,
};
use native_theme::theme::{AnimatedIcon, ColorMode, IconData, ResolvedTheme, Theme};
use native_theme::{AccessibilityPreferences, ResolutionContext};
use native_theme_gpui::{ActiveNativeTheme as _, Native, apply, geometry, icons, to_theme};

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

/// Install `preset` as the native theme, lay `build` out, and report where each
/// of `selectors` landed.
fn laid_out<const N: usize>(
    cx: &mut TestAppContext,
    preset: &str,
    style: Option<StyleRefinement>,
    build: Build,
    selectors: [&'static str; N],
) -> [Bounds<Pixels>; N] {
    let prefs = scaled();
    let resolved = resolved(preset);
    let theme = to_theme(&resolved, preset, false, &prefs);
    cx.update(|cx| {
        gpui_component::init(cx);
        apply(theme, &resolved, &prefs, cx);
    });
    let (_, cx) = cx.add_window_view(|_, _| Harness { style, build });
    cx.update(|window, cx| window.draw(cx).clear(cx));
    selectors.map(|s| {
        cx.debug_bounds(s)
            .unwrap_or_else(|| panic!("{s} was not laid out"))
    })
}

fn measure(
    cx: &mut TestAppContext,
    preset: &str,
    style: Option<StyleRefinement>,
    build: Build,
) -> Size<Pixels> {
    let [probe] = laid_out(cx, preset, style, build, ["probe"]);
    probe.size
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

// --- Icons: what an animated icon shows on its first frame -----------------

/// The size one animation frame is rasterised at here; a size this file names,
/// so the expected bounds are that number and not a default.
const FRAME_SIZE: u32 = 32;
const FRAME_SVG: &[u8] = b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'>\
                           <circle cx='12' cy='12' r='10' fill='red'/></svg>";

/// The first frame of a frame-based animation, converted the way the showcase
/// converts the ones it caches.
fn first_animation_frame() -> Option<ImageSource> {
    let frame = IconData::Svg(std::borrow::Cow::Borrowed(FRAME_SVG));
    let anim = AnimatedIcon::frames(
        vec![frame.clone(), frame.clone(), frame],
        NonZeroU32::new(80)?,
    )
    .ok()?;
    icons::animated_frames_to_image_sources(&anim, None, Some(FRAME_SIZE))?
        .sources
        .into_iter()
        .next()
}

/// A view that shows the frame only once it is asked to.
///
/// `add_window_view` runs the executor to a standstill before it returns
/// (gpui-pre `src/app/test_context.rs:303-328`), so an element built in the
/// opening frame has had every asynchronous decode finish behind it. Holding
/// the frame back means the draw the test asks for is the first one that ever
/// needs the image, which is the frame the maintainer sees blink.
struct FirstDraw {
    showing: bool,
}

impl Render for FirstDraw {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let frame = match (self.showing, first_animation_frame()) {
            (true, Some(source)) => gpui::img(source).into_any_element(),
            _ => div().into_any_element(),
        };
        div().size(px(600.)).flex().items_start().child(
            div()
                .debug_selector(|| "probe".into())
                .flex_none()
                .child(frame),
        )
    }
}

/// An animated icon is on the screen in the frame it first appears in.
///
/// An `img` whose size is auto takes the image's own size, and it can only do
/// that once the image data is in hand (gpui-pre
/// `src/elements/img.rs:348-380`); with no data it lays out at nothing and
/// paints nothing. `ImageSource::Image` is answered by
/// `window.use_asset`, which returns `None` until a background decode finishes
/// (`src/elements/img.rs:534-553`), so every frame of an animation is blank
/// the first time it comes up -- the blink. `ImageSource::Render` is answered
/// from the value itself, in the same frame.
#[gpui::test]
fn an_animation_frame_is_there_on_its_first_draw(cx: &mut TestAppContext) {
    let (view, cx) = cx.add_window_view(|_, _| FirstDraw { showing: false });
    cx.update(|_window, cx| {
        view.update(cx, |this, cx| {
            this.showing = true;
            cx.notify();
        });
    });
    cx.update(|window, cx| window.draw(cx).clear(cx));
    let probe = cx.debug_bounds("probe");
    let raster = px(FRAME_SIZE as f32);
    assert_eq!(
        probe.map(|b| b.size),
        Some(gpui::size(raster, raster)),
        "the frame laid out at {probe:?} on the draw it first appeared in, so gpui \
         had no image to measure and painted nothing"
    );
}

/// kde-breeze states a 300 px tooltip with a 3 px horizontal padding, so a
/// sentence overruns it by a wide margin.
const TOOLTIP_PRESET: &str = "kde-breeze";
/// One line that no bundled preset's `tooltip.max_width` holds unwrapped.
const LONG_TOOLTIP: &str = "This popup carries the platform's max width, padding, radius, text size \
                            and text colour, and it is long enough to need more than one line.";
/// Short enough to sit well inside the same width.
const SHORT_TOOLTIP: &str = "Save";
/// The debug selector on the element the application hands the tooltip.
const TOOLTIP_TEXT: &str = "tooltip-text";
/// Upstream draws the bubble with a one-pixel border on every side
/// (`src/tooltip.rs:117`, `border_1()`), which the platform's outer width pays
/// for along with the two paddings.
const TOOLTIP_BORDER: f32 = 1.0;

/// An application-built tooltip: `geometry::tooltip` on the bubble, and
/// `geometry::tooltip_content` on the element the application itself passes to
/// `Tooltip::element` — reached through the installed theme, the way an
/// application reaches it (`geometry`'s module doc).
fn tooltip_of(
    text: &'static str,
    style: Option<&StyleRefinement>,
    window: &mut Window,
    cx: &mut Context<Harness>,
) -> AnyElement {
    let content = cx
        .native_theme()
        .and_then(|nt| nt.native(cx))
        .map(geometry::tooltip_content);
    let tooltip = Tooltip::element(move |_, _| {
        let body = div().debug_selector(|| TOOLTIP_TEXT.into()).child(text);
        match &content {
            Some(style) => body.refine_style(style),
            None => body,
        }
    });
    let tooltip = match style {
        Some(style) => tooltip.refine_style(style),
        None => tooltip,
    };
    tooltip.build(window, cx).into_any_element()
}

fn long_tooltip(
    s: Option<&StyleRefinement>,
    w: &mut Window,
    cx: &mut Context<Harness>,
) -> AnyElement {
    tooltip_of(LONG_TOOLTIP, s, w, cx)
}
fn short_tooltip(
    s: Option<&StyleRefinement>,
    w: &mut Window,
    cx: &mut Context<Harness>,
) -> AnyElement {
    tooltip_of(SHORT_TOOLTIP, s, w, cx)
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
// `select`'s metrics on a different upstream seam (`combobox.rs`); only the
// text colour separates the two builders, and the height is what is measured.
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

/// The tooltip's seam is a width, and the text has to keep to it.
///
/// A tooltip's text sits in a bare `div()` inside upstream's `h_flex()`
/// (`src/tooltip.rs:126-132`), so it is a flex item with an automatic minimum
/// size, and gpui measures text under `AvailableSpace::MinContent` without
/// wrapping it — the wrap width is taken only from a *definite* available
/// width (`gpui-pre-0.3.5/src/elements/text.rs:649-656`). The item's minimum is
/// therefore the whole unwrapped line: a max width on the bubble alone clamps
/// the bubble and not the text, and the text runs out of it.
#[gpui::test]
fn tooltip_text_keeps_inside_the_bubble(cx: &mut TestAppContext) {
    let t = resolved(TOOLTIP_PRESET).tooltip;
    let inner = px(t.max_width - 2.0 * t.border.padding_horizontal - 2.0 * TOOLTIP_BORDER);
    let style = native_style(TOOLTIP_PRESET, geometry::tooltip);

    let [text, probe] = laid_out(
        cx,
        TOOLTIP_PRESET,
        Some(style.clone()),
        long_tooltip,
        [TOOLTIP_TEXT, "probe"],
    );
    assert!(
        text.size.width <= inner,
        "a long tooltip text laid out {:?} wide, past the {inner:?} the platform's \
         {:?} max width leaves between the paddings and the border",
        text.size.width,
        px(t.max_width),
    );
    assert!(
        text.right() <= probe.right(),
        "the text ends at {:?}, past the {:?} the tooltip itself ends at",
        text.right(),
        probe.right(),
    );

    // The same width must not stretch a short tooltip: `max_w` is a ceiling.
    let [short, _] = laid_out(
        cx,
        TOOLTIP_PRESET,
        Some(style),
        short_tooltip,
        [TOOLTIP_TEXT, "probe"],
    );
    assert!(
        short.size.width < text.size.width && short.size.width < inner,
        "a short tooltip text was stretched to {:?}",
        short.size.width,
    );
}
