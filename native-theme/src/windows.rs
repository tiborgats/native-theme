//! The Windows reader: light or dark, the accent and its ramp, high
//! contrast and its system colours, the animation and transparency effects
//! and the text scale, into a [`ReaderState`] and a reader layer (S8, S9.3,
//! S9.5); and the running Windows version, for the generation table. The
//! reader's build from what Windows gave is compiled on every platform, for
//! its tests.
//!
//! The accent is a setting Windows defines independently of the scheme, so
//! it is stated under both schemes (S9.3), with its ramp as one group
//! (S9.2): `input.accent` in the common bases, which any preset takes, the
//! six `input.accent_ramp.*` shades under `for_preset.<own>`, the preset's
//! own inputs (S9.5). A colour of the group that cannot be read drops the
//! whole group, reported as the status's fallback, naming the input.
//!
//! Under high contrast the reader states the eight system colours WinUI's
//! `HighContrast` dictionaries draw with, `GetSysColor` of each index
//! (`role.@system_window` … `role.@system_gray_text`), as a reader rule under
//! `contrast_high` and `preset:<own>`; the preset's bases for them are
//! WinUI's placeholders (ADD-A135). Outside high contrast those indices hold
//! classic Win32 colours WinUI does not draw, so they are not read, and the
//! high-contrast slots are ones the platform gives none at, no `Fallback`
//! (S9.3, ADD-A138).
//!
//! The reader patches no text-role slot: WinUI draws its own type ramp, and
//! Windows' text-size setting is the text-scaling factor (S9.4, S9.5). It
//! reads no icon size: `windows-11`'s small and large sizes are pending
//! (U516), whose facts decide whether they are the shell's metrics the
//! reader would read or WinUI's own sizes (S9.3, ADD-A140).
//!
//! # Sources
//!
//! What the reader reads that the user's settings change and a WinRT event
//! reports; the watcher subscribes to each event:
//!
//! - `UISettings.GetColorValue`: the accent colour and its shades, and the
//!   foreground, which tells light from dark (`UISettings::ColorValuesChanged`).
//! - `UISettings.TextScaleFactor`: the text scale
//!   (`UISettings::TextScaleFactorChanged`).
//! - `UISettings.AnimationsEnabled`: animations off and reduced motion, the
//!   *Animation effects* setting turned off (`UISettings::AnimationsEnabledChanged`, Windows 10
//!   2004 and later).
//! - `UISettings.AdvancedEffectsEnabled`: reduced transparency, the
//!   *Transparency effects* setting turned off
//!   (`UISettings::AdvancedEffectsEnabledChanged`, Windows 10 1703 and later,
//!   <https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uisettings.advancedeffectsenabled>).
//!
//! # Sources not watched
//!
//! Each is reported by a message Windows sends to top-level windows, which
//! the watcher, without a window, does not receive: `WM_SETTINGCHANGE` with
//! the `SPI_SET*` action in `wParam` for the `SystemParametersInfo` one
//! (<https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-settingchange>)
//! and `WM_SYSCOLORCHANGE`
//! (<https://learn.microsoft.com/en-us/windows/win32/gdi/wm-syscolorchange>).
//!
//! - `SPI_GETHIGHCONTRAST`: high contrast. Its WinRT event,
//!   `AccessibilitySettings::HighContrastChanged`, "never fires" outside UWP:
//!   it relies on the `CoreWindow` to handle `WM_THEMECHANGED`, and a Win32
//!   process has none
//!   (<https://github.com/microsoft/WindowsAppSDK/blob/main/specs/themes/ThemeSettings.md>);
//!   the watcher subscribes to it where Windows accepts the registration
//!   (GitHub's `windows-latest` image refuses it with "Element not found").
//! - `GetSysColor`: the high-contrast colours.
//! - The running Windows version: the generation, fixed for a process.

// Win32 FFI through the `windows` crate: `SystemParametersInfoW` and
// `GetSysColor` have no safe binding.
#![allow(unsafe_code)]

use crate::color::Color;
use crate::error::ReaderFailure;
use crate::inputs::{Contrast, InputName, InputValue};
use crate::key::Key;
use crate::layer::{Bases, Layer};
use crate::presets::PresetId;
use crate::reader::Reading;
use crate::resolve::ReaderState;
use crate::roles::ColorRolesPatch;
use crate::selector::{Condition, Rule, RuleBody, Scheme, Selector, SetMatch};
use crate::value::{SchemeMap, Value};

/// The text-scaling factor of unscaled text, S16's no-reader state's: the
/// identity of S9.4's scaling, not a platform value.
const UNSCALED: f32 = 1.0;

/// The ramp's shades, in the presets' `accent_ramp` order: each input's key
/// and the `UIColorType` read for it.
const RAMP: [(&str, &str); 6] = [
    ("dark1", "AccentDark1"),
    ("dark2", "AccentDark2"),
    ("dark3", "AccentDark3"),
    ("light1", "AccentLight1"),
    ("light2", "AccentLight2"),
    ("light3", "AccentLight3"),
];

/// The running Windows version, from the `windows-version` crate's safe
/// `OsVersion::current()` (`RtlGetVersion`), for the generation table:
/// Windows 11 still reports major version 10, and its build tells the
/// generations apart.
#[cfg(all(target_os = "windows", feature = "windows"))]
pub(crate) fn os_version() -> crate::pipeline::OsVersion {
    let v = ::windows_version::OsVersion::current();
    crate::pipeline::OsVersion::new(v.major, v.minor, v.build)
}

/// COM's multithreaded apartment, kept for the process from the first call
/// to the process's end: `CoIncrementMTAUsage`, once, on a thread of its
/// own, its cookie never given back; the failure where it failed. Called
/// before every stock icon load (`winicons`) and every start of the Windows
/// watcher (`watch::windows`).
///
/// When the last thread uninitializes COM in the process, COM unloads the
/// servers it loaded (`ProcessUninitialize`, `CleanUpDllsForProcess`) and
/// refuses COM work starting on other threads meanwhile. A stock icon load
/// runs on the caller's thread, which this crate does not own: on one that
/// has not initialized COM, the load uses the MTA another thread keeps ("A
/// thread has an implicit MTA apartment type if it does not initialize the
/// COM apartment itself, and if another thread has already initialized the
/// MTA in the process",
/// <https://learn.microsoft.com/en-us/windows/win32/api/objidlbase/ne-objidlbase-apttypequalifier>),
/// and the shell's server it runs in (`thumbcache.dll`) was unloaded under
/// it, an access violation (CI run 42554706381, the Windows watcher's thread
/// uninitializing). The Windows watcher's thread initializes and
/// uninitializes its own apartment: its start failed where another
/// thread's uninitialization was the last (0x8000000E, "the application is
/// exiting", and 0x80080008, `CO_E_SERVER_STOPPING`; CI run 42556588847),
/// and its own, where it was the last, shut COM down under the process's
/// other COM work. `CoIncrementMTAUsage` is COM's call for an API that
/// "requires COM to be initialized, but has no information about whether
/// the current thread is already in an apartment, and does not need the
/// current thread to go into a particular apartment"; it "ensures that the
/// system doesn't free resources related to MTA support, even if the MTA
/// thread count goes to 0"
/// (<https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coincrementmtausage>).
/// Kept for the process, no thread's uninitialization is the last.
#[cfg(all(target_os = "windows", feature = "system-icons"))]
pub(crate) fn mta_kept() -> Result<(), &'static str> {
    static KEPT: std::sync::OnceLock<Result<(), String>> = std::sync::OnceLock::new();
    KEPT.get_or_init(|| {
        // On a thread of its own: the call puts a thread in no apartment
        // into the MTA, and the caller's thread is left as it is. The usage
        // outlives the thread (its cookie may be given back from any other).
        let keeper = std::thread::Builder::new()
            .spawn(|| {
                // SAFETY: CoIncrementMTAUsage writes its cookie into a value
                // the binding owns and returns; it is not called during
                // process shutdown or from DllMain, which its documentation
                // rules out. The cookie is dropped without
                // CoDecrementMTAUsage: the usage is kept for the process, on
                // purpose.
                unsafe { ::windows::Win32::System::Com::CoIncrementMTAUsage() }
                    .map(|_kept_for_the_process| ())
                    .map_err(|err| err.to_string())
            })
            .map_err(|err| format!("no thread to call it on: {err}"))?;
        keeper
            .join()
            .map_err(|_| "its thread panicked".to_owned())?
    })
    .as_ref()
    .copied()
    .map_err(String::as_str)
}

/// Runs `test` while another thread joins the process's multithreaded
/// apartment and leaves it again and again, as other COM users of a process
/// may: each leaving is the process's last uninitialization of COM unless
/// something keeps it ([`mta_kept`]).
#[cfg(all(
    test,
    target_os = "windows",
    any(feature = "system-icons", all(feature = "windows", feature = "watch"))
))]
pub(crate) fn while_another_thread_leaves_the_mta<R>(test: impl FnOnce() -> R) -> R {
    use ::windows::Win32::System::Com::{COINIT_MULTITHREADED, CoInitializeEx, CoUninitialize};
    use std::sync::atomic::{AtomicBool, Ordering};

    /// Sets its flag when dropped: the test's end, or its unwinding.
    struct SetOnDrop<'a>(&'a AtomicBool);
    impl Drop for SetOnDrop<'_> {
        fn drop(&mut self) {
            self.0.store(true, Ordering::Relaxed);
        }
    }

    let done = AtomicBool::new(false);
    std::thread::scope(|scope| {
        scope.spawn(|| {
            while !done.load(Ordering::Relaxed) {
                // SAFETY: a successful CoInitializeEx, S_FALSE included, is
                // balanced by one CoUninitialize on this thread, with no COM
                // object created between them.
                unsafe {
                    if CoInitializeEx(None, COINIT_MULTITHREADED).is_ok() {
                        CoUninitialize();
                    }
                }
            }
        });
        let _running = SetOnDrop(&done);
        test()
    })
}

/// The eight `GetSysColor` colours WinUI's `HighContrast` theme dictionaries
/// draw with, each a `SystemColor*Color` resource, `GetSysColor` of its
/// index (`microsoft-ui-xaml@2.5.1
/// src/dxaml/xcp/components/theming/FrameworkTheming.cpp:145-158`, `:219-232`;
/// `src/dxaml/xcp/components/theminginterop/SystemThemingInterop.cpp:202`), and the keyed role it
/// is (ADD-A135, the Windows trace's W3-35).
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(
    not(all(target_os = "windows", feature = "windows")),
    allow(dead_code, reason = "the Windows reader's: not linked in this build")
)]
pub(crate) struct SystemColors {
    /// `COLOR_WINDOW`: `role.@system_window`.
    pub(crate) window: Color,
    /// `COLOR_WINDOWTEXT`: `role.@system_window_text`.
    pub(crate) window_text: Color,
    /// `COLOR_BTNFACE`: `role.@system_button_face`.
    pub(crate) button_face: Color,
    /// `COLOR_BTNTEXT`: `role.@system_button_text`.
    pub(crate) button_text: Color,
    /// `COLOR_HIGHLIGHT`: `role.@system_highlight`.
    pub(crate) highlight: Color,
    /// `COLOR_HIGHLIGHTTEXT`: `role.@system_highlight_text`.
    pub(crate) highlight_text: Color,
    /// `COLOR_HOTLIGHT`: `role.@system_hotlight`.
    pub(crate) hotlight: Color,
    /// `COLOR_GRAYTEXT`: `role.@system_gray_text`.
    pub(crate) gray_text: Color,
}

#[cfg_attr(
    not(all(target_os = "windows", feature = "windows")),
    allow(dead_code, reason = "the Windows reader's: not linked in this build")
)]
impl SystemColors {
    /// The keyed roles, each its colour.
    fn roles(&self) -> Vec<(Key, Value<Color>)> {
        [
            (crate::key!("system_window"), self.window),
            (crate::key!("system_window_text"), self.window_text),
            (crate::key!("system_button_face"), self.button_face),
            (crate::key!("system_button_text"), self.button_text),
            (crate::key!("system_highlight"), self.highlight),
            (crate::key!("system_highlight_text"), self.highlight_text),
            (crate::key!("system_hotlight"), self.hotlight),
            (crate::key!("system_gray_text"), self.gray_text),
        ]
        .into_iter()
        .map(|(key, color)| (key, Value::Stated(color)))
        .collect()
    }
}

/// What Windows gave, in plain types: built from its reads on Windows,
/// directly in tests. A colour read that failed holds what failed.
#[cfg_attr(
    not(all(target_os = "windows", feature = "windows")),
    allow(dead_code, reason = "the Windows reader's: not linked in this build")
)]
pub(crate) struct WindowsData {
    /// `UISettings.GetColorValue(Foreground)`.
    pub(crate) foreground: Color,
    /// `UISettings.GetColorValue(Accent)`.
    pub(crate) accent: Result<Color, String>,
    /// The ramp's shades, in [`RAMP`]'s order.
    pub(crate) ramp: [Result<Color, String>; 6],
    /// `SPI_GETHIGHCONTRAST`'s `HCF_HIGHCONTRASTON`; `None`: the call failed.
    pub(crate) high_contrast: Option<bool>,
    /// The system colours, read under high contrast only.
    pub(crate) system_colors: Option<SystemColors>,
    /// `UISettings.AnimationsEnabled`; `None`: not given.
    pub(crate) animations_enabled: Option<bool>,
    /// `UISettings.AdvancedEffectsEnabled`; `None`: not given.
    pub(crate) advanced_effects_enabled: Option<bool>,
    /// `UISettings.TextScaleFactor`; `None`: not given.
    pub(crate) text_scale_factor: Option<f64>,
}

/// The scheme the foreground tells, as Microsoft's guide for Win32 apps
/// tells it: dark where the foreground "can be classified as light",
/// `((5 * clr.G) + (2 * clr.R) + clr.B) > (8 * 128)` ("Support Dark and
/// Light themes in Win32 apps", ms.date 2026-06-16, `IsColorLight`,
/// `windows-dev-docs@f54166ed
/// hub/apps/desktop/modernize/ui/apply-windows-themes.md:5`, `:75-78`,
/// `:88`), "If the foreground is light, then Dark mode is enabled". In `f32`, whose
/// sums of these 8-bit products are exact.
#[cfg_attr(
    not(all(target_os = "windows", feature = "windows")),
    allow(dead_code, reason = "the Windows reader's: not linked in this build")
)]
fn scheme(foreground: Color) -> Scheme {
    let [r, g, b, _] = foreground.to_srgb8_clamped().map(f32::from);
    if 5.0 * g + 2.0 * r + b > 8.0 * 128.0 {
        Scheme::Dark
    } else {
        Scheme::Light
    }
}

/// The reader's state and layer from what Windows gave, its own preset's
/// part under `own` (S9.5). Zero I/O.
///
/// - The state: the scheme ([`scheme`]); `Contrast::High` under high
///   contrast; animations off and reduced motion where *Animation effects*
///   is off, the one setting standing for both (W3-37); reduced
///   transparency where *Transparency effects* is; the text-scaling factor
///   where it is finite and above zero, else unscaled ([`UNSCALED`]); no
///   font DPI, the reader reading no font (S9.4).
/// - The accent group at both schemes, the accent in the common bases and
///   its ramp under `for_preset.<own>`; none of it where one of the seven
///   failed, the status's fallback naming it (S9.2, S9.3).
/// - Under high contrast, a rule on the system colours under
///   `contrast_high` and `preset:<own>`.
#[cfg_attr(
    not(all(target_os = "windows", feature = "windows")),
    allow(dead_code, reason = "the Windows reader's: not linked in this build")
)]
pub(crate) fn build_reading(data: &WindowsData, own: PresetId) -> Reading {
    let high_contrast = data.high_contrast == Some(true);
    let factor = data
        .text_scale_factor
        .map(|factor| factor as f32)
        .filter(|factor| factor.is_finite() && *factor > 0.0);
    let mut state = ReaderState::new(scheme(data.foreground), factor.unwrap_or(UNSCALED));
    state.contrast = high_contrast.then_some(Contrast::High);
    state.animations_off = data.animations_enabled == Some(false);
    state.reduced_motion = state.animations_off;
    state.reduced_transparency = data.advanced_effects_enabled == Some(false);

    let mut layer = Layer::default();
    let group = std::iter::once(("accent".to_owned(), "Accent", &data.accent)).chain(
        RAMP.iter()
            .zip(&data.ramp)
            .map(|((key, kind), color)| (format!("accent_ramp.{key}"), *kind, color)),
    );
    let mut read = Vec::new();
    let mut fallback = None;
    for (path, kind, color) in group {
        match (color, InputName::new(&path)) {
            (Ok(color), Ok(name)) => read.push((name, *color)),
            (Err(why), _) => {
                fallback = Some(ReaderFailure::Platform(format!(
                    "input.{path}: UISettings.GetColorValue({kind}) failed: {why}"
                )));
                break;
            }
            (Ok(_), Err(error)) => {
                fallback = Some(ReaderFailure::Platform(format!("input.{path}: {error}")));
                break;
            }
        }
    }
    let mut own_bases = Bases::default();
    if fallback.is_none() {
        for (name, color) in read {
            let value = InputValue::new(SchemeMap::new(Value::Stated(color), Value::Stated(color)));
            let bases = if name == InputName::accent() {
                &mut layer.bases
            } else {
                &mut own_bases
            };
            bases.inputs.entries.push((name, value));
        }
    }
    if own_bases != Bases::default() {
        layer.for_preset.push((own, own_bases));
    }

    if let (true, Some(colors)) = (high_contrast, &data.system_colors) {
        let selector = Selector {
            conditions: SetMatch {
                all_of: vec![Condition::ContrastHigh, Condition::Preset(own)],
                none_of: Vec::new(),
            },
            ..Selector::default()
        };
        let roles = ColorRolesPatch {
            keyed: Some(colors.roles()),
            ..ColorRolesPatch::default()
        };
        layer
            .rules
            .push(Rule::new(selector, RuleBody::Colors(Box::new(roles))));
    }

    Reading {
        state,
        layer,
        fallback,
    }
}

/// A WinRT colour as the model's colour.
#[cfg(all(target_os = "windows", feature = "windows"))]
fn win_color(c: ::windows::UI::Color) -> Color {
    Color {
        r: f32::from(c.R) / 255.0,
        g: f32::from(c.G) / 255.0,
        b: f32::from(c.B) / 255.0,
        a: f32::from(c.A) / 255.0,
    }
}

/// A Win32 COLORREF (`0x00BBGGRR`, red in the low byte) as the model's
/// colour, opaque.
#[cfg_attr(
    not(all(target_os = "windows", feature = "windows")),
    allow(dead_code, reason = "the Windows reader's: not linked in this build")
)]
fn colorref(c: u32) -> Color {
    let [r, g, b, _] = c.to_le_bytes();
    Color {
        r: f32::from(r) / 255.0,
        g: f32::from(g) / 255.0,
        b: f32::from(b) / 255.0,
        a: 1.0,
    }
}

/// Whether high contrast is on: `SPI_GETHIGHCONTRAST`'s
/// `HCF_HIGHCONTRASTON`; `None` when the call fails.
#[cfg(all(target_os = "windows", feature = "windows"))]
fn read_high_contrast() -> Option<bool> {
    use ::windows::Win32::UI::Accessibility::{HCF_HIGHCONTRASTON, HIGHCONTRASTW};
    use ::windows::Win32::UI::WindowsAndMessaging::{
        SPI_GETHIGHCONTRAST, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS, SystemParametersInfoW,
    };

    let mut hc = HIGHCONTRASTW {
        cbSize: u32::try_from(std::mem::size_of::<HIGHCONTRASTW>()).ok()?,
        ..Default::default()
    };
    // SAFETY: `hc` is a live HIGHCONTRASTW whose `cbSize` is its size, as
    // SPI_GETHIGHCONTRAST requires of `uiParam`, and `pvParam` points to it
    // for the whole call; only `dwFlags` is read afterwards
    // (https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow).
    let read = unsafe {
        SystemParametersInfoW(
            SPI_GETHIGHCONTRAST,
            hc.cbSize,
            Some(std::ptr::from_mut(&mut hc).cast()),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
    };
    read.is_ok()
        .then(|| hc.dwFlags.contains(HCF_HIGHCONTRASTON))
}

/// The eight high-contrast system colours ([`SystemColors`]).
#[cfg(all(target_os = "windows", feature = "windows"))]
fn read_system_colors() -> SystemColors {
    use ::windows::Win32::Graphics::Gdi::{
        COLOR_BTNFACE, COLOR_BTNTEXT, COLOR_GRAYTEXT, COLOR_HIGHLIGHT, COLOR_HIGHLIGHTTEXT,
        COLOR_HOTLIGHT, COLOR_WINDOW, COLOR_WINDOWTEXT, GetSysColor, SYS_COLOR_INDEX,
    };

    fn sys_color(index: SYS_COLOR_INDEX) -> Color {
        // SAFETY: GetSysColor takes a colour index by value and touches no
        // memory of the caller's
        // (https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor).
        colorref(unsafe { GetSysColor(index) })
    }

    SystemColors {
        window: sys_color(COLOR_WINDOW),
        window_text: sys_color(COLOR_WINDOWTEXT),
        button_face: sys_color(COLOR_BTNFACE),
        button_text: sys_color(COLOR_BTNTEXT),
        highlight: sys_color(COLOR_HIGHLIGHT),
        highlight_text: sys_color(COLOR_HIGHLIGHTTEXT),
        hotlight: sys_color(COLOR_HOTLIGHT),
        gray_text: sys_color(COLOR_GRAYTEXT),
    }
}

/// Reads the Windows settings, its own preset's part under `own`: the
/// colours, the text scale and the effects from `UISettings`, high
/// contrast from `SPI_GETHIGHCONTRAST` and, under it, the system colours
/// from `GetSysColor`.
///
/// Fails where `UISettings` cannot be created (before Windows 10) or the
/// foreground, which decides the scheme, cannot be read
/// (`ReaderFailure::Platform`); an accent or shade that cannot be read is
/// the status's fallback.
#[cfg(all(target_os = "windows", feature = "windows"))]
pub(crate) fn read(own: PresetId) -> Result<Reading, ReaderFailure> {
    use ::windows::UI::ViewManagement::{UIColorType, UISettings};

    let settings = UISettings::new()
        .map_err(|e| ReaderFailure::Platform(format!("UISettings unavailable: {e}")))?;
    let color = |kind: UIColorType| {
        settings
            .GetColorValue(kind)
            .map(win_color)
            .map_err(|e| e.to_string())
    };
    let foreground = color(UIColorType::Foreground).map_err(|e| {
        ReaderFailure::Platform(format!("UISettings.GetColorValue(Foreground) failed: {e}"))
    })?;
    let high_contrast = read_high_contrast();
    let data = WindowsData {
        foreground,
        accent: color(UIColorType::Accent),
        ramp: [
            UIColorType::AccentDark1,
            UIColorType::AccentDark2,
            UIColorType::AccentDark3,
            UIColorType::AccentLight1,
            UIColorType::AccentLight2,
            UIColorType::AccentLight3,
        ]
        .map(color),
        high_contrast,
        system_colors: (high_contrast == Some(true)).then(read_system_colors),
        animations_enabled: settings.AnimationsEnabled().ok(),
        advanced_effects_enabled: settings.AdvancedEffectsEnabled().ok(),
        text_scale_factor: settings.TextScaleFactor().ok(),
    };
    Ok(build_reading(&data, own))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
pub(crate) mod tests;
