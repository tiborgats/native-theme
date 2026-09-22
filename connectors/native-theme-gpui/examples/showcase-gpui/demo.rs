//! Demo helpers the pages share.

use gpui::{App, Div, Entity, SharedString, Stateful, prelude::*};
use gpui_component::{ActiveTheme, TitleBar, menu::AppMenuBar};
use native_theme_gpui::geometry;

use crate::CHROME_APP_MENU_BAR;
use crate::app::Quit;
use crate::info::{self, InfoExt as _, InfoRegistry, native_info};

/// A `TitleBar` refined by `geometry::title_bar`, reading `label`, holding
/// `app_menu_bar` where the platform has no menu bar of its own, and quitting
/// the application from its close button.
pub(crate) fn title_bar(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    label: impl Into<SharedString>,
    app_menu_bar: Entity<AppMenuBar>,
) -> Stateful<Div> {
    let mut bar_info = info::title_bar(cx.theme());
    let bar = native_info(
        TitleBar::new(),
        cx,
        geometry::title_bar,
        "title_bar",
        &mut bar_info,
    )
    // Linux only: upstream drops the handler on every other platform
    // (title_bar.rs:95-104). On Windows the OS acts on the X itself; macOS
    // draws its own controls.
    .on_close_window(|_, window, cx| window.dispatch_action(Box::new(Quit), cx))
    // Plain text, not a Label: `Label::render` sets `foreground` on its own
    // element (label.rs:211), which would hide the title-bar font's colour
    // `geometry::title_bar` just gave the bar.
    .child(label.into())
    .when(cfg!(not(target_os = "macos")), |bar| {
        bar.child(
            app_menu_bar
                .info(ui, "chrome-app-menu-bar", info::app_menu_bar(cx.theme()))
                .debug_selector(|| CHROME_APP_MENU_BAR.into()),
        )
    });
    bar.info(ui, "chrome-title-bar", bar_info)
}
