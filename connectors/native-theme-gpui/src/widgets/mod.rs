//! Theme-drawn controls for the six widgets gpui-component draws from its own
//! literals (feature `widgets`, on by default).
//!
//! gpui-component builds its checkbox, radio, switch, slider, progress bar and
//! spinner from sizes and alpha blends of its own, which no `ThemeColor` field
//! or `StyleRefinement` reaches: the indicator in rems per `Size`
//! (`checkbox.rs:219-224`), the switch's track and thumb in pixels
//! (`switch.rs:150-157`), the slider's rail as its fill at 0.2
//! (`slider.rs:290`), the progress bar's track likewise
//! (`progress/progress.rs:135`), and the spinner as a turning icon
//! (`spinner.rs:25`). The controls here are built the way gpui-component
//! builds its own -- on gpui-base's headless primitives, which own
//! activation, focus, keyboard, dragging and the AccessKit role -- and paint
//! every part from the [`ResolvedTheme`] leaf that states it. The spinner
//! draws the application's icon set's own loading indicator, and an arc only
//! for a set without one.
//!
//! The tab bar joins them because no gpui-component `TabBar` variant draws
//! what `tab.*` states: each paints an idle tab transparent
//! (`tab/tab.rs:132`) and marks the selected one its own way, with a primary
//! underline or a frame in `border` (`tab/tab.rs`, `TabVariant`). The
//! separator joins them because gpui-component's draws its line a literal
//! `px(1.)` thick (`separator.rs:78-82`).
//!
//! Each widget reads the variant [`apply`](crate::apply) installed for the
//! current mode at render, through [`ActiveNativeTheme`]. With none installed,
//! or with a length that is not finite, it renders gpui-component's own
//! control instead, so nothing here ever paints a value no source gives.
//!
//! Rules, from `docs/todo_gpui-widgets-spec.md` §1:
//!
//! * a hover colour is a layer composited over the idle fill; a `None` soft
//!   option copies the colour it would cover (a tab's hover takes the place
//!   of its fill, over the bar, as Breeze paints it: docs/platform-facts.md
//!   §2.11);
//! * a disabled control paints its stated disabled colours, and its
//!   `disabled_opacity` is not multiplied on top;
//! * a size the theme does not state keeps gpui-component's own value, named
//!   as a constant with its upstream line.
//!
//! Upstream citations are verified against gpui-component 0.6.6, gpui-base
//! 0.6.6 and gpui-pre 0.3.6.

/// `IntoElement` for a `RenderOnce` widget, as gpui's `#[derive(IntoElement)]`
/// writes it (gpui-pre-macros `derive_into_element.rs`). Written out because
/// the derive rewrites its `gpui::` paths to `gpui_kit::` whenever the
/// manifest names gpui-kit (`gpui_pre_facade_paths.rs`), which this crate's
/// dev-dependencies do, and the library does not depend on gpui-kit.
macro_rules! into_element {
    ($($widget:ty),+ $(,)?) => {$(
        impl gpui::IntoElement for $widget {
            type Element = gpui::ViewElement<Self>;

            #[track_caller]
            fn into_element(self) -> Self::Element {
                gpui::ViewElement::new(self)
            }
        }
    )+};
}

mod checkbox;
mod progress;
mod separator;
mod slider;
mod spinner;
mod switch;
mod tab_bar;
#[cfg(test)]
mod tests;

pub use checkbox::{Checkbox, CheckboxLook, Radio, RadioGroup};
pub use progress::{ProgressBar, ProgressBarLook};
pub use separator::{Separator, SeparatorLook};
pub use slider::{Slider, SliderLook};
pub use spinner::{Spinner, SpinnerLook};
pub use switch::{Switch, SwitchLook};
pub use tab_bar::{Tab, TabBar, TabLook};

into_element!(
    Checkbox,
    Radio,
    RadioGroup,
    Switch,
    Slider,
    ProgressBar,
    Spinner,
    TabBar,
    Separator
);

use gpui::{App, Hsla, Pixels, px};
use native_theme::color::Rgba;

use crate::{ActiveNativeTheme, Native};

/// The installed native view for the current mode, if any.
fn native(cx: &App) -> Option<Native<'_>> {
    cx.native_theme().and_then(|nt| nt.native(cx))
}

/// A stated colour as gpui's.
fn color(rgba: Rgba) -> Hsla {
    crate::colors::rgba_to_hsla(rgba)
}

/// `layer` composited over `base`: a hover state layer on the fill it covers
/// (spec §1.3, C17). An opaque layer composites to itself. The same blend
/// `colors.rs` composites the button states with.
fn over(base: Hsla, layer: Hsla) -> Hsla {
    base.blend(layer)
}

/// A stated length in pixels, or `None` when it is not a finite,
/// non-negative number: the widget then falls back to gpui-component's own
/// control (spec §1.1).
fn length(v: f32) -> Option<Pixels> {
    (v.is_finite() && v >= 0.0).then(|| px(v))
}

/// A font size the text-scaling factor scales, as the `geometry` builders
/// scale theirs.
fn text_size(size: f32, n: Native<'_>) -> Pixels {
    px(crate::scaled_text_size(size, n.accessibility))
}
