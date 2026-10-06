//! The showcase's actions, hosted around the whole window.
//!
//! gpui-base 0.7.1's `Root` mounts the view and, beside it, one overlay per
//! registered plugin (gpui-base root.rs, `Root::render`); gpui-component's
//! `WindowState` draws dialogs, sheets and notifications there. An action
//! dispatched from inside one of them walks only its own ancestors
//! (gpui-pre window.rs, `dispatch_action`), so handlers on the view's own
//! element never see it. `decorate` wraps the finished surface — view and
//! overlays — so the handlers sit around both.

use gpui::{
    AnyElement, App, Context, InteractiveElement as _, IntoElement, ParentElement as _, Render,
    Styled as _, Window, div,
};
use gpui_base::{Root, RootPlugin};

use crate::app::{
    OpenAbout, OpenCommandPalette, OpenPreferences, ReloadTheme, SetColorMode, SetPreset, ShowPage,
    Showcase, ToggleSidePanel,
};

/// Registered once, after `gpui_kit::init` and before any window opens (a
/// `Root` instantiates the plugins registered when it is created, gpui-base
/// root.rs, `Root::new`), so it wraps outside gpui-component's `WindowState`.
pub(crate) struct ShowcaseHost;

impl Render for ShowcaseHost {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

macro_rules! forward {
    ($surface:expr, $showcase:expr, $($action:ty => $method:ident),+ $(,)?) => {
        $surface
            $(.on_action({
                let showcase = $showcase.clone();
                move |action: &$action, window: &mut Window, cx: &mut App| {
                    showcase.update(cx, |this, cx| this.$method(action, window, cx));
                }
            }))+
    };
}

impl RootPlugin for ShowcaseHost {
    fn decorate(
        &self,
        surface: AnyElement,
        root: &Root,
        _window: &mut Window,
        _cx: &mut App,
    ) -> impl IntoElement {
        let Ok(showcase) = root.view().clone().downcast::<Showcase>() else {
            return surface;
        };
        forward!(
            div().relative().size_full(),
            showcase,
            ShowPage => on_show_page,
            SetColorMode => on_set_color_mode,
            ReloadTheme => on_reload_theme,
            ToggleSidePanel => on_toggle_side_panel,
            SetPreset => on_set_preset,
            OpenCommandPalette => on_open_command_palette,
            OpenPreferences => on_open_preferences,
            OpenAbout => on_open_about,
        )
        .child(surface)
        .into_any_element()
    }
}

/// Register the host for every window opened after this call.
pub(crate) fn register(cx: &mut App) {
    Root::register_plugin::<ShowcaseHost>(cx, |_, _| ShowcaseHost);
}
