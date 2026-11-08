//! The font DPI the KDE reader reports: `Xft.dpi` on an X11 session, as
//! Qt's xcb plugin takes its logical DPI from it, parsed by the clean
//! room's [`qxcb_xft_dpi`] (the 0.8.0 plan's D10; S9.3's *The KDE reader's
//! readings*, S9.4, K3.46, ADD-A140).

use super::qxcb_xft_dpi::qxcb_xft_dpi;

/// The font DPI on the session `session_type` (`$XDG_SESSION_TYPE`): on
/// `x11` the DPI Qt's xcb plugin forces from the X resources, `xrdb
/// -query`'s text ([`crate::detect::x_resources`], [`font_dpi_of`]);
/// `None` on any other session, where Qt's Wayland screen gives the
/// logical DPI `kde-breeze`'s `token.text.font_dpi` states
/// (`qtbase@6.12.0 src/plugins/platforms/wayland/qwaylandscreen.cpp:150-160`,
/// `docs/facts/kde/tokens.toml:7959`), and where xrdb gives nothing.
pub(super) fn read_font_dpi(session_type: Option<&str>) -> Option<f32> {
    if session_type != Some("x11") {
        return None;
    }
    font_dpi_of(&crate::detect::x_resources()?)
}

/// The font DPI the X resources `resources` force: the last `Xft.dpi` line
/// Qt parses, where it is above zero (`qtbase@6.12.0
/// src/plugins/platforms/xcb/qxcbscreen.cpp:298-314`, `:366-373`,
/// `:856-876`); `None` where they set none, and Qt falls back to the
/// preset's DPI (`:870-875`).
pub(super) fn font_dpi_of(resources: &str) -> Option<f32> {
    qxcb_xft_dpi(resources).map(|dpi| dpi as f32)
}
