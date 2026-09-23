//! Rendered seam tests (spec v0.5.9 §6).
//!
//! Each `geometry` builder rests on one upstream behaviour: the widget applies
//! its own size defaults and *then* the caller's style. These tests lay real
//! gpui-component widgets out on GPUI's headless test platform, once without
//! and once with the builder's refinement, and compare the measured height
//! with the refinement's own field. When an upstream release stops honouring
//! a seam, this file fails (the nightly dependency canary runs it).
//!
//! Every seam test: `u` (unstyled) must differ from `e` (expected), otherwise
//! the input does not discriminate; then `s` (styled) must equal `e`. They
//! run at text scale 1, where the control-height rule gives each control its
//! stated height.
//!
//! `button`, `input` and `progress` set the same `size.height` their widget
//! sets for itself, so they also prove the ordering of the two writes.
//! `select` and `combobox` set `min_size.height` and `list_item` a height the
//! widget leaves content-driven, so those three prove that the refinement
//! reaches the widget's root box and wins there, not an ordering claim.
//!
//! Four sweeps run every native preset at its own font DPI: the drawn content
//! inset of an Input, a Select and a Combobox is the stated left padding
//! side; an Input with no suffix is laid out with the stated right side; a
//! Textarea whose refinement has its padding cleared draws none; and six
//! single-line controls are their stated height at text scale 1, no shorter
//! at 1.1 and at 2, with their text inside them at all three.
//! Every other builder rests on the source citation in its doc comment.

use std::num::NonZeroU32;

use gpui::{
    AbsoluteLength, AnyElement, AppContext as _, Bounds, Context, DefiniteLength, ImageSource,
    InteractiveElement as _, IntoElement, Length, ParentElement as _, Pixels, Render, SharedString,
    Size, StyleRefinement, Styled as _, TestAppContext, Window, div, px,
};
use gpui_component::{
    ActiveTheme as _, IndexPath, StyledExt as _,
    button::Button,
    combobox::{Combobox, ComboboxState},
    input::{Input, InputState, Textarea, TextareaState},
    list::ListItem,
    progress::Progress,
    select::{SearchableVec, Select, SelectItem, SelectState},
    tooltip::Tooltip,
};
use native_theme::theme::{AnimatedIcon, ColorMode, IconData, ResolvedTheme, Theme};
use native_theme::{AccessibilityPreferences, ResolutionContext};
use native_theme_gpui::{ActiveNativeTheme as _, Native, apply, geometry, icons, to_theme};

fn scaled_by(factor: f32) -> AccessibilityPreferences {
    AccessibilityPreferences {
        text_scaling_factor: factor,
        ..AccessibilityPreferences::default()
    }
}

/// The text scale the seam tests run at.
fn scaled() -> AccessibilityPreferences {
    scaled_by(1.0)
}

/// Resolve a preset without consulting the machine. `ResolutionContext::for_tests`
/// pins the font DPI at 96, so the point sizes the presets declare convert to
/// the same pixels everywhere; `from_preset` would resolve through
/// `ResolutionContext::from_system`, whose DPI is read from the desktop
/// (`native-theme/src/resolve/context.rs:62-83`) and would make the expected
/// height machine-dependent.
fn resolved(preset: &str) -> ResolvedTheme {
    resolved_at(preset, ResolutionContext::for_tests().font_dpi)
}

/// `preset` resolved at `dpi`: the point sizes it states become pixels at
/// that DPI.
fn resolved_at(preset: &str, dpi: f32) -> ResolvedTheme {
    Theme::preset(preset)
        .expect("preset loads")
        .into_variant(ColorMode::Light)
        .expect("light variant")
        .into_resolved(&ResolutionContext {
            font_dpi: dpi,
            ..ResolutionContext::for_tests()
        })
        .expect("preset resolves")
}

/// The native presets, each with the font DPI its platform resolves at:
/// macOS at 72, where a point is a pixel (native-theme `detect.rs`,
/// `detect_system_font_dpi`), the others at the 96 `for_tests` pins.
const NATIVE: [(&str, f32); 4] = [
    ("kde-breeze", 96.0),
    ("adwaita", 96.0),
    ("macos-sonoma", 72.0),
    ("windows-11", 96.0),
];

type Build = fn(Option<&StyleRefinement>, &mut Window, &mut Context<Harness>) -> AnyElement;

struct Harness {
    style: Option<StyleRefinement>,
    build: Build,
}

impl Render for Harness {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // What gpui-component's `Root` does for an application's window
        // (`root.rs:582`, `:590`): the installed font size is the rem, which
        // the connector scales by the text-scaling factor, and the installed
        // family is the text's.
        window.set_rem_size(cx.theme().font_size);
        let family = cx.theme().font_family.clone();
        let widget = (self.build)(self.style.as_ref(), window, cx);
        div()
            .size(px(600.))
            .flex()
            .items_start()
            .font_family(family)
            .child(
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
    laid_out_as(
        cx,
        preset,
        &resolved(preset),
        &scaled(),
        style,
        build,
        selectors,
    )
}

/// [`laid_out`] with the resolved theme and the preferences given.
fn laid_out_as<const N: usize>(
    cx: &mut TestAppContext,
    preset: &str,
    resolved: &ResolvedTheme,
    prefs: &AccessibilityPreferences,
    style: Option<StyleRefinement>,
    build: Build,
    selectors: [&'static str; N],
) -> [Bounds<Pixels>; N] {
    let theme = to_theme(resolved, preset, false, prefs);
    cx.update(|cx| {
        gpui_component::init(cx);
        apply(theme, resolved, prefs, cx);
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

/// kde-breeze states a 300 px tooltip with a 3 px padding on the left and the
/// right, so a sentence overruns it by a wide margin.
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
    let (Some(left), Some(right)) = (t.border.padding.left, t.border.padding.right) else {
        panic!(
            "{TOOLTIP_PRESET} no longer states both horizontal tooltip sides; pick another preset"
        );
    };
    let inner = px(t.max_width - left - right - 2.0 * TOOLTIP_BORDER);
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

// --- The drawn content inset and the control-height rule ----------------------

/// The debug selector on the element around a sample's text.
const TEXT: &str = "text";
/// The sample text.
const SAMPLE: &str = "Ag";

fn text_probe() -> gpui::Div {
    div().debug_selector(|| TEXT.into()).child(SAMPLE)
}

/// An item whose trigger title is the text probe, so a Select's title can be
/// measured.
#[derive(Clone)]
struct Probed(SharedString);

impl SelectItem for Probed {
    type Value = SharedString;
    fn title(&self) -> SharedString {
        self.0.clone()
    }
    fn display_title(&self) -> Option<AnyElement> {
        Some(
            div()
                .debug_selector(|| TEXT.into())
                .child(self.0.clone())
                .into_any_element(),
        )
    }
    fn value(&self) -> &SharedString {
        &self.0
    }
}

/// A Button whose label is the text probe.
fn probed_button(
    s: Option<&StyleRefinement>,
    _: &mut Window,
    _: &mut Context<Harness>,
) -> AnyElement {
    styled(Button::new("b").child(text_probe()), s)
}
/// An Input whose prefix is the text probe: the prefix is the root's first
/// child (`input/input.rs:724`), so its left edge is where the root's
/// padding ends, and it lays out one line of the root's text style.
fn probed_input(
    s: Option<&StyleRefinement>,
    w: &mut Window,
    cx: &mut Context<Harness>,
) -> AnyElement {
    let state = cx.new(|cx| InputState::new(w, cx));
    styled(Input::new(&state).prefix(text_probe()), s)
}
/// A Select showing a selected item whose title is the text probe.
fn probed_select(
    s: Option<&StyleRefinement>,
    w: &mut Window,
    cx: &mut Context<Harness>,
) -> AnyElement {
    let items = vec![Probed(SAMPLE.into())];
    let state =
        cx.new(|cx| SelectState::new(SearchableVec::new(items), Some(IndexPath::default()), w, cx));
    styled(Select::new(&state), s)
}
/// A Combobox whose trigger body is the text probe (`Combobox::render_trigger`).
fn probed_combobox(
    s: Option<&StyleRefinement>,
    w: &mut Window,
    cx: &mut Context<Harness>,
) -> AnyElement {
    let items: Vec<SharedString> = vec!["a".into(), "b".into()];
    let state = cx.new(|cx| ComboboxState::new(SearchableVec::new(items), vec![], w, cx));
    styled(
        Combobox::new(&state).render_trigger(|_, _, _| text_probe()),
        s,
    )
}
/// A ListItem whose content is the text probe.
fn probed_list_item(
    s: Option<&StyleRefinement>,
    _: &mut Window,
    _: &mut Context<Harness>,
) -> AnyElement {
    styled(ListItem::new("i").child(text_probe()), s)
}
/// A menu row the application draws, as the showcase's `demo::menu_rows`
/// does: gpui-component's own `MenuItemElement` is crate-private
/// (`menu/menu_item.rs:10-11`), so `geometry::menu_item` lands on the
/// application's row, a flex `div` of items centred on the cross axis.
fn probed_menu_row(
    s: Option<&StyleRefinement>,
    _: &mut Window,
    _: &mut Context<Harness>,
) -> AnyElement {
    styled(div().flex().items_center().child(text_probe()), s)
}

/// Upstream draws a one-pixel border round the Select and Combobox triggers
/// (`select.rs:535`, `combobox.rs:986`, `border_1()`), which the drawn inset
/// includes; `geometry::select`/`combobox` set no border width.
const TRIGGER_BORDER: f32 = 1.0;

/// Under every native preset, at its own DPI, the drawn content inset of an
/// Input, a Select and a Combobox is the border and the stated padding side:
/// upstream pads each root before the refinement (`input/input.rs:701` then
/// `:719`, `select.rs:544` then `:546`, `combobox.rs:995` then `:997`), so the
/// platform's side is the one drawn.
///
/// The left side is measured: it is where the first child starts. At least
/// one preset per widget states a side upstream does not draw on its own, or
/// the sweep proves nothing.
#[gpui::test]
fn input_select_and_combobox_draw_the_stated_padding(cx: &mut TestAppContext) {
    type Case = (&'static str, Build, fn(Native<'_>) -> StyleRefinement);
    let cases: [Case; 3] = [
        ("input", probed_input, geometry::input),
        ("select", probed_select, geometry::select),
        ("combobox", probed_combobox, geometry::combobox),
    ];
    for (widget, build, geom) in cases {
        let mut discriminated = false;
        for (preset, dpi) in NATIVE {
            let r = resolved_at(preset, dpi);
            let prefs = scaled_by(1.0);
            let (stated, border) = match widget {
                "input" => (r.input.border.padding.left, r.input.border.line_width),
                _ => (r.combo_box.border.padding.left, TRIGGER_BORDER),
            };
            let Some(stated) = stated else {
                continue;
            };
            let style = geom(Native {
                resolved: &r,
                accessibility: &prefs,
            });
            let inset = |style: Option<StyleRefinement>, cx: &mut TestAppContext| {
                let [text, probe] =
                    laid_out_as(cx, preset, &r, &prefs, style, build, [TEXT, "probe"]);
                text.left() - probe.left()
            };
            let u = inset(None, cx);
            let s = inset(Some(style), cx);
            let e = px(border + stated);
            assert_eq!(
                s, e,
                "{preset} {widget}: the content starts {s:?} in, not at the {border}px border \
                 plus the stated left padding {stated}px"
            );
            discriminated |= u != e;
        }
        assert!(
            discriminated,
            "{widget}: no native preset states a left padding upstream does not draw already"
        );
    }
}

/// A Textarea: it renders as a multi-line `Input` (`input/textarea.rs:162-165`).
fn textarea(s: Option<&StyleRefinement>, w: &mut Window, cx: &mut Context<Harness>) -> AnyElement {
    let state = cx.new(|cx| TextareaState::new(w, cx));
    styled(Textarea::new(&state), s)
}

/// `style` with its right padding side 0.
fn without_right(style: &StyleRefinement) -> StyleRefinement {
    let mut style = style.clone();
    style.padding.right = Some(px(0.).into());
    style
}

/// Under every native preset, at its own DPI, an Input with no suffix is laid
/// out with the stated right padding side: upstream pads a single-line root
/// (`input/input.rs:700-702`) before the refinement (`:719`), and only a
/// suffix pads the right side again after it (`:736`). The right side has no
/// child to measure it by, so it is measured as the width it adds: the
/// Input's width under the refinement, less its width under the same
/// refinement with a right side of 0. At least one preset states a right side
/// that is not 0, or the sweep proves nothing.
#[gpui::test]
fn an_input_without_a_suffix_draws_the_stated_right_padding(cx: &mut TestAppContext) {
    let mut discriminated = false;
    for (preset, dpi) in NATIVE {
        let r = resolved_at(preset, dpi);
        let prefs = scaled_by(1.0);
        let Some(stated) = r.input.border.padding.right else {
            continue;
        };
        let style = geometry::input(Native {
            resolved: &r,
            accessibility: &prefs,
        });
        let mut width = |style: StyleRefinement| {
            laid_out_as(cx, preset, &r, &prefs, Some(style), input, ["probe"])[0]
                .size
                .width
        };
        let drawn = width(style.clone()) - width(without_right(&style));
        assert_eq!(
            drawn,
            px(stated),
            "{preset} input: the right side adds {drawn:?}, not the stated {stated}px"
        );
        discriminated |= stated != 0.0;
    }
    assert!(
        discriminated,
        "input: no native preset states a right padding other than 0"
    );
}

/// `style` with every padding side 0.
fn without_padding(style: &StyleRefinement) -> StyleRefinement {
    let mut style = style.clone();
    style.padding.top = Some(px(0.).into());
    style.padding.right = Some(px(0.).into());
    style.padding.bottom = Some(px(0.).into());
    style.padding.left = Some(px(0.).into());
    style
}

/// Under every native preset, at its own DPI, a Textarea refined as
/// `geometry::input`'s doc says -- the padding sides cleared -- draws no
/// padding: upstream pads only a single-line root (`input/input.rs:700-702`),
/// so a multi-line root's inset is 0, and the refinement's padding must not
/// reach it. The inset has no child to measure it by, so it is measured as
/// the size it adds: the Textarea's size with the padding cleared equals its
/// size with every side 0. The height the rule sets is removed, as the
/// showcase replaces it with its own, so the height is the content's.
///
/// Left in, the refinement's padding would reach the root (`:719`): at least
/// one preset states sides that then add to the width, or the sweep proves
/// nothing about the clearing.
#[gpui::test]
fn a_textarea_draws_no_padding(cx: &mut TestAppContext) {
    let mut discriminated = false;
    for (preset, dpi) in NATIVE {
        let r = resolved_at(preset, dpi);
        let prefs = scaled_by(1.0);
        let mut style = geometry::input(Native {
            resolved: &r,
            accessibility: &prefs,
        });
        style.size.height = None;
        let mut cleared = style.clone();
        cleared.padding = StyleRefinement::default().padding;
        let mut size = |style: StyleRefinement| {
            laid_out_as(cx, preset, &r, &prefs, Some(style), textarea, ["probe"])[0].size
        };
        let none = size(without_padding(&style));
        let drawn = size(cleared);
        assert_eq!(
            drawn, none,
            "{preset} textarea: with the padding cleared it is {drawn:?}, not {none:?}, the \
             size with no padding, so something pads the multi-line root"
        );
        let stated = r.input.border.padding;
        let sides = stated.left.unwrap_or(0.0) + stated.right.unwrap_or(0.0);
        let reached = size(style).width - none.width;
        discriminated |= sides != 0.0 && reached == px(sides);
    }
    assert!(
        discriminated,
        "textarea: under no native preset does the refinement's padding, left in, add its \
         stated left and right sides, so clearing it proves nothing"
    );
}

/// Where upstream's own Select or Combobox trigger is taller than the
/// platform's stated minimum, `geometry::select`/`combobox` (through `min_h`,
/// as they always have) leave upstream's height: upstream's `input_size` sets
/// `h_8`, 2 rem, for `Size::Medium` (`sizing.rs:236-237`, `:261-264`), and a
/// minimum below it changes nothing. These are the (preset, widget) pairs
/// where that happens at text scale 1; the stated heights themselves are the
/// follow-up plan's (spec v0.5.9 unstated-sizes rationale §7).
const TRIGGER_TALLER_THAN_STATED: &[(&str, &str)] =
    &[("macos-sonoma", "select"), ("macos-sonoma", "combobox")];

/// Under every native preset, at its own DPI: at text scale 1 each single-line
/// control is its stated height and its text lies inside it; at text scale 2
/// it is at least that tall, and its text still lies inside it. Growth is the
/// means, not the requirement: a control with no stated vertical padding whose
/// text exactly fills it at scale 2 (windows-11's list row) is correct.
#[gpui::test]
fn single_line_controls_are_their_stated_height_and_fit_the_text_at_every_scale(
    cx: &mut TestAppContext,
) {
    type Case = (
        &'static str,
        Build,
        fn(Native<'_>) -> StyleRefinement,
        fn(&ResolvedTheme) -> f32,
    );
    let cases: [Case; 6] = [
        ("button", probed_button, geometry::button, |r| {
            r.button.min_height
        }),
        ("input", probed_input, geometry::input, |r| {
            r.input.min_height
        }),
        ("select", probed_select, geometry::select, |r| {
            r.combo_box.min_height
        }),
        ("combobox", probed_combobox, geometry::combobox, |r| {
            r.combo_box.min_height
        }),
        ("menu row", probed_menu_row, geometry::menu_item, |r| {
            r.menu.row_height
        }),
        ("list item", probed_list_item, geometry::list_item, |r| {
            r.list.row_height
        }),
    ];
    let mut taller = Vec::new();
    for (widget, build, geom, stated_of) in cases {
        for (preset, dpi) in NATIVE {
            let r = resolved_at(preset, dpi);
            let stated = px(stated_of(&r));
            let at = |factor: f32, cx: &mut TestAppContext| {
                let prefs = scaled_by(factor);
                let style = geom(Native {
                    resolved: &r,
                    accessibility: &prefs,
                });
                let [text, probe] =
                    laid_out_as(cx, preset, &r, &prefs, Some(style), build, [TEXT, "probe"]);
                let own = laid_out_as(cx, preset, &r, &prefs, None, build, ["probe"])[0];
                (text, probe, own)
            };
            let fits = |text: Bounds<Pixels>, control: Bounds<Pixels>| {
                text.top() >= control.top() && text.bottom() <= control.bottom()
            };

            let (text, control, own) = at(1.0, cx);
            let expected = if matches!(widget, "select" | "combobox") && own.size.height > stated {
                taller.push((preset, widget));
                own.size.height
            } else {
                stated
            };
            assert_eq!(
                control.size.height, expected,
                "{preset} {widget}: at text scale 1 the control is not its stated height {stated:?} \
                 (upstream's own: {:?})",
                own.size.height
            );
            assert!(
                fits(text, control),
                "{preset} {widget}: at text scale 1 the text at {text:?} is not inside the \
                 control at {control:?}"
            );

            // Just above 1, where the rule stops giving a control its stated
            // height, it must not come out shorter than at 1.
            let (text11, control11, _) = at(1.1, cx);
            assert!(
                control11.size.height >= control.size.height,
                "{preset} {widget}: at text scale 1.1 the control is {:?}, shorter than {:?} \
                 at scale 1",
                control11.size.height,
                control.size.height
            );
            assert!(
                fits(text11, control11),
                "{preset} {widget}: at text scale 1.1 the text at {text11:?} is not inside the \
                 control at {control11:?}"
            );

            let (text2, control2, _) = at(2.0, cx);
            assert!(
                control2.size.height >= control.size.height,
                "{preset} {widget}: at text scale 2 the control is {:?}, shorter than {:?} \
                 at scale 1",
                control2.size.height,
                control.size.height
            );
            assert!(
                fits(text2, control2),
                "{preset} {widget}: at text scale 2 the text at {text2:?} is not inside the \
                 control at {control2:?}"
            );
        }
    }
    assert_eq!(
        taller, TRIGGER_TALLER_THAN_STATED,
        "the triggers upstream draws taller than the stated minimum changed"
    );
}
