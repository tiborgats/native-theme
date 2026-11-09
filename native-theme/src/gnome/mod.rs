//! GNOME reader: reads the colour scheme, the accent colour, contrast,
//! motion, the text-scaling factor, overlay scroll bars and the icon theme
//! from the XDG Desktop Portal's Settings interface (through ashpd) and
//! gsettings, into a [`ReaderState`] and a reader layer (S8, S9.3, S9.5).
//!
//! The conditions, the text-scaling factor, the accent and the fonts are
//! the layer's portable part, which any preset takes; the icon theme, a
//! freedesktop theme's name, is its own preset's (`for_preset.<own>`). The
//! accent is a setting GNOME defines independently of the scheme, so it is
//! stated under both schemes (S9.3): the accent libadwaita shows, the
//! portal's colour classified by the clean room's
//! [`adw_accent_nearest`](crate::gnome::helpers::adw_accent_nearest())
//! (the 0.8.0 plan's D10). The fonts patch the text-role slots `body` and
//! `monospace`, parsed by the clean room's
//! [`helpers::pango_font_description`](crate::gnome::helpers::pango_font_description()).
//!
//! # Sources
//!
//! Each a `namespace key` pair: a portal setting of `org.freedesktop.appearance`,
//! or a gsettings key, which the portal's GNOME and GTK backends forward under
//! its schema's name. The watcher observes each through the portal's
//! `SettingChanged` signal.
//!
//! - `org.freedesktop.appearance color-scheme`: light or dark.
//! - `org.freedesktop.appearance accent-color`: the accent colour, which
//!   libadwaita classifies into one of its nine accents.
//! - `org.freedesktop.appearance contrast`: high contrast.
//! - `org.freedesktop.appearance reduced-motion`: reduced motion.
//! - `org.gnome.desktop.interface color-scheme`: light or dark, where the
//!   portal gives none.
//! - `org.gnome.desktop.interface enable-animations`: animations off.
//! - `org.gnome.desktop.interface text-scaling-factor`: the text scale.
//! - `org.gnome.desktop.interface overlay-scrolling`: overlay scroll bars.
//! - `org.gnome.desktop.interface icon-theme`: the icon theme.
//! - `org.gnome.desktop.interface font-name`: the `body` slot's font.
//! - `org.gnome.desktop.interface monospace-font-name`: the `monospace`
//!   slot's font.
//! - `org.gnome.desktop.a11y.interface high-contrast`: high contrast, where
//!   the portal reports none.
//! - `org.gnome.desktop.a11y.interface reduced-motion`: reduced motion, where
//!   the portal gives none.
//!
//! # Sources not watched
//!
//! - `$XDG_SESSION_TYPE`: the session type, which decides whether the
//!   text-scaling factor is reported (`toolkit_scale_carries_text_scale`),
//!   fixed for a session.

pub(crate) mod helpers;
pub(crate) mod portal;

use ashpd::desktop::settings::{ColorScheme, Contrast as PortalContrast, ReducedMotion};

use helpers::adw_accent_nearest;

use crate::blocks::BlockPatch;

use crate::color::Color;
use crate::inputs::{Contrast, InputName, InputValue};
use crate::layer::{Bases, Layer};
use crate::presets::PresetId;
use crate::reader::Reading;
use crate::resolve::ReaderState;
use crate::selector::Scheme;
use crate::tokens::{IconTokensPatch, TextRolesPatch, TextTokensPatch};
use crate::value::{SchemeMap, Value};

/// The text-scaling factor of unscaled text, S16's no-reader state's: the
/// identity of S9.4's scaling, not a platform value.
const UNSCALED: f32 = 1.0;

/// One deadline for all the gsettings reads of one GNOME read together, so
/// that an unresponsive D-Bus costs it this once rather than once per key.
/// Ruled equal to the bound [`crate::detect`] gives one subprocess.
const GSETTINGS_DEADLINE: std::time::Duration = crate::detect::SUBPROCESS_TIMEOUT;

/// Read a single gsettings value within `timeout`, returning None if
/// gsettings is unavailable, fails or outlives it, or `timeout` is zero.
///
/// Single-quote wrapping (gsettings string output format) is stripped from
/// the result.
fn read_gsetting(schema: &str, key: &str, timeout: std::time::Duration) -> Option<String> {
    if timeout.is_zero() {
        return None;
    }
    let raw = crate::detect::gsettings_get(schema, key, timeout)?;
    let trimmed = raw.trim_matches('\'').to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

/// What the portal and gsettings gave, in plain types: built from their
/// reads in production, directly in tests. No ashpd types, no D-Bus access,
/// no gsettings subprocess calls.
struct GnomeData {
    /// The scheme: the portal's, else gsettings', else light.
    scheme: Scheme,
    /// The portal's accent, in `0..=1` per channel (the XDG portal's range;
    /// outside it the accent is unset). `None`: no accent given.
    accent_rgb: Option<(f64, f64, f64)>,
    /// Whether the portal asks for high contrast.
    high_contrast: bool,
    /// gsettings' `high-contrast` (GNOME before 44, whose portal reports
    /// none); read only where the portal reports no high contrast.
    gsettings_high_contrast: Option<bool>,
    /// Reduced motion: the portal's, else gsettings' `reduced-motion`.
    /// `None`: neither gave one.
    reduced_motion: Option<bool>,
    /// gsettings' `enable-animations`. `None`: not given.
    animations_enabled: Option<bool>,
    /// gsettings' `text-scaling-factor`. `None`: not given.
    text_scaling_factor: Option<f32>,
    /// The session type, `$XDG_SESSION_TYPE` (`x11`, `wayland`, …), as
    /// [`crate::detect::session_type`] reads it. `None`: not set.
    session_type: Option<String>,
    /// gsettings' `overlay-scrolling`. `None`: not given.
    overlay_scrolling: Option<bool>,
    /// The icon theme's name (`Adwaita`, `Papirus`). `None`: not given.
    icon_theme: Option<String>,
    /// gsettings' `font-name`, a Pango font description. `None`: not given.
    font_name: Option<String>,
    /// gsettings' `monospace-font-name`. `None`: not given.
    monospace_font_name: Option<String>,
}

/// Whether the user asked for high contrast: the portal's `contrast`, or
/// gsettings' `high-contrast` where the portal gives none (GNOME < 44).
fn effective_high_contrast(data: &GnomeData) -> bool {
    data.high_contrast || data.gsettings_high_contrast == Some(true)
}

/// Whether on a session of `session_type` the toolkit's own scale factor
/// carries GNOME's text-scaling factor: an X11 session only.
/// gnome-settings-daemon 51.0 writes the XSETTINGS `Xft/DPI` and the
/// `Xft.dpi` resource as 96 × `text-scaling-factor` × the window scale
/// (`plugins/xsettings/gsd-xsettings-manager.c:666-668`, `:742`,
/// `:797-798`), and on X11 winit 0.30.13 takes its scale factor from that
/// DPI over 96, XSETTINGS first (`src/platform_impl/linux/x11/util/randr.rs:41-53`,
/// `:145-146`), as gpui-pre-linux 0.3.10 does from `Xft.dpi`
/// (`src/linux/x11/client.rs:2674-2681`). Each does so only while its
/// override is unset: winit's `WINIT_X11_SCALE_FACTOR` (`randr.rs:112-154`)
/// and gpui's `GPUI_X11_SCALE_FACTOR` (`client.rs:2623-2669`) set the scale
/// factor instead, and then the application owns the scale and the
/// text-scaling factor is applied nowhere. Neither crate's Wayland backend
/// reads `Xft.dpi` or XSETTINGS (only their `x11` modules do), so a native
/// Wayland client's scale factor does not carry the text-scaling factor.
fn toolkit_scale_carries_text_scale(session_type: Option<&str>) -> bool {
    session_type == Some("x11")
}

/// The text-scaling factor the reader reports: gsettings' where it is
/// finite and above zero (S12 refuses another), [`UNSCALED`] where none is
/// given, and [`UNSCALED`] on an X11 session, where the toolkit applied it
/// ([`toolkit_scale_carries_text_scale`]).
fn text_scaling_factor(data: &GnomeData) -> f32 {
    if toolkit_scale_carries_text_scale(data.session_type.as_deref()) {
        return UNSCALED;
    }
    data.text_scaling_factor
        .filter(|factor| factor.is_finite() && *factor > 0.0)
        .unwrap_or(UNSCALED)
}

/// The accent libadwaita shows for the portal's accent: each channel
/// narrowed to single precision and the colour classified into one of its
/// nine accents ([`adw_accent_nearest`],
/// `libadwaita@1.10.0 src/adw-settings-impl-portal.c:124-141`), that
/// accent's colour ([`helpers::accent_color`]). `None` where a channel lies
/// outside `0..=1`, which the XDG portal specification reads as "accent
/// colour not set", so the preset's own accent shows (S9.3).
fn portal_accent((r, g, b): (f64, f64, f64)) -> Option<Color> {
    let channel = |value: f64| (0.0..=1.0).contains(&value).then_some(value as f32);
    helpers::accent_color(adw_accent_nearest(channel(r)?, channel(g)?, channel(b)?))
}

/// The text-role slots the reader patches from `data`'s font strings
/// ([`helpers::font_patch`]), the point sizes converted by `own`'s
/// `token.text.font_dpi`, GNOME reporting none (S9.4); `None` where it
/// patches none.
fn text_roles(data: &GnomeData, own: PresetId) -> Option<TextRolesPatch> {
    if data.font_name.is_none() && data.monospace_font_name.is_none() {
        return None;
    }
    let font_dpi = own.font_dpi();
    let slot = |text: &Option<String>| {
        text.as_deref()
            .and_then(|text| helpers::font_patch(text, font_dpi))
            .map(BlockPatch::Fields)
    };
    let roles = TextRolesPatch {
        body: slot(&data.font_name),
        monospace: slot(&data.monospace_font_name),
        ..TextRolesPatch::default()
    };
    (roles != TextRolesPatch::default()).then_some(roles)
}

/// The reader's state and layer from what the portal and gsettings gave,
/// its own preset's part under `own` (S9.5). Zero I/O.
///
/// - The state: the scheme; `contrast_high` under high contrast (libadwaita
///   has one contrast condition, the preset's `contrast_high`); reduced
///   motion; animations off where `enable-animations` is false; overlay
///   scroll bars; the text-scaling factor ([`text_scaling_factor`]). No
///   font DPI: GNOME lays fonts out at the preset's `token.text.font_dpi`
///   (S9.4).
/// - The common bases: `input.accent` at both schemes, the accent being a
///   setting GNOME defines independently of the scheme (S9.3); the
///   text-role slots `body` and `monospace` ([`text_roles`]).
/// - `for_preset.<own>`: `token.icons.theme`.
fn build_gnome_reading(data: &GnomeData, own: PresetId) -> Reading {
    let mut state = ReaderState::new(data.scheme, text_scaling_factor(data));
    state.contrast = effective_high_contrast(data).then_some(Contrast::High);
    state.reduced_motion = data.reduced_motion.unwrap_or(false);
    state.animations_off = data.animations_enabled == Some(false);
    state.overlay_scrollbars = data.overlay_scrolling.unwrap_or(false);

    let mut layer = Layer::default();
    if let Some(accent) = data.accent_rgb.and_then(portal_accent) {
        let accent = Value::Stated(accent);
        layer.bases.inputs.entries.push((
            InputName::accent(),
            InputValue::new(SchemeMap::new(accent.clone(), accent)),
        ));
    }
    if let Some(roles) = text_roles(data, own) {
        layer.bases.tokens.text = Some(Box::new(TextTokensPatch {
            roles: Some(Box::new(roles)),
            ..TextTokensPatch::default()
        }));
    }
    if let Some(theme) = &data.icon_theme {
        let mut own_bases = Bases::default();
        own_bases.tokens.icons = Some(Box::new(IconTokensPatch {
            theme: Some(Value::Stated(theme.clone())),
            ..IconTokensPatch::default()
        }));
        layer.for_preset.push((own, own_bases));
    }
    Reading {
        state,
        layer,
        fallback: None,
    }
}

/// The scheme: the portal's `color-scheme` where it gives one (dark for
/// `prefer-dark`, light otherwise), else gsettings' `color-scheme`
/// (`prefer-dark` dark, `prefer-light` and `default` light), else light.
fn scheme(
    portal: Option<ColorScheme>,
    gsetting: impl FnOnce(&str, &str) -> Option<String>,
) -> Scheme {
    let dark = match portal {
        Some(scheme) => matches!(scheme, ColorScheme::PreferDark),
        None => gsetting("org.gnome.desktop.interface", "color-scheme")
            .is_some_and(|value| value.contains("prefer-dark")),
    };
    if dark { Scheme::Dark } else { Scheme::Light }
}

/// Reduced motion: the portal's `reduced-motion` where it gives one, else
/// gsettings' `reduced-motion` (`reduce` against `no-preference`).
fn reduced_motion(
    portal: Option<ReducedMotion>,
    gsetting: impl FnOnce(&str, &str) -> Option<String>,
) -> Option<bool> {
    match portal {
        Some(motion) => Some(matches!(motion, ReducedMotion::ReducedMotion)),
        None => gsetting("org.gnome.desktop.a11y.interface", "reduced-motion")
            .map(|value| value == "reduce"),
    }
}

/// The reader's state and layer from what the portal answered (`None` for a
/// setting it did not give), the session type
/// ([`crate::detect::session_type`]) and the gsettings values `gsetting`
/// reads, one read per key, its own preset's part under `own`.
pub(crate) fn build_reading(
    portal_scheme: Option<ColorScheme>,
    accent: Option<ashpd::desktop::Color>,
    contrast: PortalContrast,
    portal_motion: Option<ReducedMotion>,
    session_type: Option<String>,
    mut gsetting: impl FnMut(&str, &str) -> Option<String>,
    own: PresetId,
) -> Reading {
    let flag = |value: Option<String>| value.and_then(|s| s.parse::<bool>().ok());
    let high_contrast = matches!(contrast, PortalContrast::High);
    let data = GnomeData {
        scheme: scheme(portal_scheme, &mut gsetting),
        accent_rgb: accent.map(|c| (c.red(), c.green(), c.blue())),
        high_contrast,
        gsettings_high_contrast: if high_contrast {
            None
        } else {
            flag(gsetting(
                "org.gnome.desktop.a11y.interface",
                "high-contrast",
            ))
        },
        reduced_motion: reduced_motion(portal_motion, &mut gsetting),
        animations_enabled: flag(gsetting("org.gnome.desktop.interface", "enable-animations")),
        text_scaling_factor: gsetting("org.gnome.desktop.interface", "text-scaling-factor")
            .and_then(|s| s.parse::<f32>().ok()),
        session_type,
        overlay_scrolling: flag(gsetting("org.gnome.desktop.interface", "overlay-scrolling")),
        icon_theme: gsetting("org.gnome.desktop.interface", "icon-theme"),
        font_name: gsetting("org.gnome.desktop.interface", "font-name"),
        monospace_font_name: gsetting("org.gnome.desktop.interface", "monospace-font-name"),
    };
    build_gnome_reading(&data, own)
}

/// Read the GNOME settings, its own preset's part under `own`.
///
/// Reads the colour scheme, accent colour, contrast and reduced motion from
/// the `org.freedesktop.appearance` portal namespace through ashpd. A
/// setting the portal does not give, because it is unavailable (no D-Bus
/// session, a sandbox, an old portal), does not answer within
/// [`PORTAL_DEADLINE`] or lacks the setting, is left to gsettings where
/// gsettings has it, and otherwise to the no-reader state's value. The
/// gsettings reads share one [`GSETTINGS_DEADLINE`]. The read cannot fail:
/// what neither source gives is not stated.
///
/// The text-scaling factor is reported where the toolkit has not applied
/// it: off an X11 session (`$XDG_SESSION_TYPE`,
/// [`crate::detect::session_type`]). On X11 the reader reports none: winit
/// and gpui take their X11 scale factor from the `Xft.dpi`
/// gnome-settings-daemon writes with the factor in it, so the connectors'
/// text scaling would apply it a second time
/// ([`toolkit_scale_carries_text_scale`] cites the sources). An X11 client
/// through Xwayland on a Wayland session gets that `Xft.dpi` too and draws
/// the factor twice; the reader cannot see a client's backend (docs/todo.md,
/// *Font DPI*).
pub(crate) async fn read(own: PresetId) -> Reading {
    let (portal_scheme, accent, contrast, portal_motion) = read_portal().await;
    let start = std::time::Instant::now();
    build_reading(
        portal_scheme,
        accent,
        contrast,
        portal_motion,
        crate::detect::session_type(),
        |schema, key| {
            read_gsetting(
                schema,
                key,
                GSETTINGS_DEADLINE.saturating_sub(start.elapsed()),
            )
        },
        own,
    )
}

/// The portal's four `org.freedesktop.appearance` settings: the colour
/// scheme, the accent, the contrast and reduced motion, each `None` (the
/// contrast its default) where the portal does not give it: no D-Bus
/// session, a sandbox, an old portal, no answer within [`PORTAL_DEADLINE`]
/// (one for all four), or the setting missing.
async fn read_portal() -> (
    Option<ColorScheme>,
    Option<ashpd::desktop::Color>,
    PortalContrast,
    Option<ReducedMotion>,
) {
    with_deadline(
        async {
            let settings = ashpd::desktop::settings::Settings::new().await.ok()?;
            let scheme = settings.color_scheme().await.ok();
            let accent = settings.accent_color().await.ok();
            let contrast = settings.contrast().await.unwrap_or_default();
            let reduced_motion = settings.reduced_motion().await.ok();
            Some((scheme, accent, contrast, reduced_motion))
        },
        PORTAL_DEADLINE,
    )
    .await
    .flatten()
    .unwrap_or_default()
}

/// Detect which desktop portal backend is running from the session bus names.
///
/// Looks for a bus name containing `portal.desktop.kde` or
/// `portal.desktop.gnome` to infer the desktop environment when
/// `XDG_CURRENT_DESKTOP` is ambiguous or unset. A backend that owns its name
/// (running) is preferred over one that is only activatable (installed); see
/// [`choose_portal_backend`].
///
/// Returns `None` if D-Bus is unavailable, does not answer within
/// [`PORTAL_DEADLINE`], or no single portal backend is found.
pub(crate) async fn detect_portal_backend() -> Option<crate::detect::LinuxDesktop> {
    let (running, activatable) = with_deadline(
        async {
            let connection = ashpd::zbus::Connection::session().await.ok()?;
            let proxy = ashpd::zbus::fdo::DBusProxy::new(&connection).await.ok()?;
            let running = proxy.list_names().await.ok()?;
            let activatable = proxy.list_activatable_names().await.ok()?;
            Some((running, activatable))
        },
        PORTAL_DEADLINE,
    )
    .await??;

    let running: Vec<&str> = running.iter().map(|n| n.as_str()).collect();
    let activatable: Vec<&str> = activatable.iter().map(|n| n.as_str()).collect();
    choose_portal_backend(&running, &activatable)
}

/// One deadline for all the D-Bus calls of one portal read together.
///
/// zbus sets no method timeout of its own (zbus-5.19.0
/// `src/connection/builder.rs:660`, `method_timeout: None`), and ashpd's
/// shared connection is a plain `zbus::Connection::session()` (ashpd-0.13.13
/// `src/proxy.rs:36-44`), so a hung or slow-starting portal backend would
/// block `from_system` for as long as the bus lets the call wait. Ruled equal
/// to the subprocess bound the same path already gives each gsettings read.
const PORTAL_DEADLINE: std::time::Duration = crate::detect::SUBPROCESS_TIMEOUT;

/// Await `fut`, or return `None` once `deadline` has passed.
async fn with_deadline<F: std::future::Future>(
    fut: F,
    deadline: std::time::Duration,
) -> Option<F::Output> {
    let mut fut = std::pin::pin!(fut);
    let mut timer = async_io::Timer::after(deadline);
    std::future::poll_fn(|cx| {
        if let std::task::Poll::Ready(out) = fut.as_mut().poll(cx) {
            return std::task::Poll::Ready(Some(out));
        }
        if std::pin::Pin::new(&mut timer).poll(cx).is_ready() {
            return std::task::Poll::Ready(None);
        }
        std::task::Poll::Pending
    })
    .await
}

/// Pick the desktop from the portal backends among the bus's names.
///
/// A running backend (`running`, the names with an owner) decides before an
/// activatable one (`activatable`, the installed `.service` files, in no
/// order the D-Bus spec defines). When a KDE and a GNOME backend match at the
/// same level the answer is `None`, and the caller's fallback decides.
fn choose_portal_backend(
    running: &[&str],
    activatable: &[&str],
) -> Option<crate::detect::LinuxDesktop> {
    fn backend_in(names: &[&str]) -> Result<Option<crate::detect::LinuxDesktop>, ()> {
        let kde = names.iter().any(|n| n.contains("portal.desktop.kde"));
        let gnome = names.iter().any(|n| n.contains("portal.desktop.gnome"));
        match (kde, gnome) {
            (true, true) => Err(()),
            (true, false) => Ok(Some(crate::detect::LinuxDesktop::Kde)),
            (false, true) => Ok(Some(crate::detect::LinuxDesktop::Gnome)),
            (false, false) => Ok(None),
        }
    }

    match backend_in(running) {
        Ok(Some(desktop)) => Some(desktop),
        Ok(None) => backend_in(activatable).ok().flatten(),
        Err(()) => None,
    }
}

#[cfg(test)]
mod tests;
