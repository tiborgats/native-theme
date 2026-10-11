//! The Windows probe's Win32 calls, all `unsafe` in windows 0.62, each in a
//! block of its own; the rest of the probe (`../windows.rs`) has none.

// Win32 FFI via the windows crate -- no safe alternative.
#![allow(unsafe_code)]

use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::Graphics::Dwm::DwmGetColorizationColor;
use windows::Win32::Graphics::Gdi::{GetSysColor, LOGFONTW, SYS_COLOR_INDEX};
use windows::Win32::System::Registry::{
    HKEY, REG_DWORD, REG_QWORD, REG_SZ, REG_VALUE_TYPE, RRF_RT_ANY, RegGetValueW,
};
use windows::Win32::UI::Accessibility::HIGHCONTRASTW;
use windows::Win32::UI::HiDpi::{
    DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, GetDpiForSystem, GetSystemMetricsForDpi,
    SetProcessDpiAwarenessContext, SystemParametersInfoForDpi,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetSystemMetrics, NONCLIENTMETRICSW, SPI_GETCARETWIDTH, SPI_GETCLEARTYPE,
    SPI_GETCLIENTAREAANIMATION, SPI_GETCOMBOBOXANIMATION, SPI_GETDROPSHADOW, SPI_GETFLATMENU,
    SPI_GETFOCUSBORDERHEIGHT, SPI_GETFOCUSBORDERWIDTH, SPI_GETFONTSMOOTHING,
    SPI_GETFONTSMOOTHINGCONTRAST, SPI_GETFONTSMOOTHINGTYPE, SPI_GETGRADIENTCAPTIONS,
    SPI_GETHIGHCONTRAST, SPI_GETHOTTRACKING, SPI_GETICONTITLELOGFONT, SPI_GETKEYBOARDCUES,
    SPI_GETLISTBOXSMOOTHSCROLLING, SPI_GETMENUANIMATION, SPI_GETMENUSHOWDELAY,
    SPI_GETNONCLIENTMETRICS, SPI_GETSELECTIONFADE, SPI_GETTOOLTIPANIMATION, SPI_GETUIEFFECTS,
    SPI_GETWHEELSCROLLLINES, SYSTEM_METRICS_INDEX, SYSTEM_PARAMETERS_INFO_ACTION,
    SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS, SystemParametersInfoW,
};
use windows::core::{BOOL, HSTRING};

/// A registry value, decoded for the types the probe reads.
pub(super) enum RegValue {
    Dword(u32),
    Qword(u64),
    Text(String),
    Other { kind: u32, bytes: u32 },
}

fn size_u32<T>() -> Result<u32, String> {
    u32::try_from(std::mem::size_of::<T>()).map_err(|e| e.to_string())
}

/// `RegGetValueW(root, subkey, name)`.
pub(super) fn reg_value(root: HKEY, subkey: &str, name: &str) -> Result<RegValue, String> {
    let subkey = HSTRING::from(subkey);
    let name = HSTRING::from(name);
    let mut kind = REG_VALUE_TYPE::default();
    let mut bytes: u32 = 0;
    // SAFETY: a size query: no data buffer, the type and byte count are
    // written to the two locals.
    let status = unsafe {
        RegGetValueW(
            root,
            &subkey,
            &name,
            RRF_RT_ANY,
            Some(&raw mut kind),
            None,
            Some(&raw mut bytes),
        )
    };
    if status != ERROR_SUCCESS {
        return Err(format!("RegGetValueW: error {}", status.0));
    }
    if kind == REG_DWORD {
        let mut v: u32 = 0;
        let mut len = size_u32::<u32>()?;
        // SAFETY: `len` is the size of `v`, the buffer written to.
        let status = unsafe {
            RegGetValueW(
                root,
                &subkey,
                &name,
                RRF_RT_ANY,
                None,
                Some((&raw mut v).cast()),
                Some(&raw mut len),
            )
        };
        return if status == ERROR_SUCCESS {
            Ok(RegValue::Dword(v))
        } else {
            Err(format!("RegGetValueW: error {}", status.0))
        };
    }
    if kind == REG_QWORD {
        let mut v: u64 = 0;
        let mut len = size_u32::<u64>()?;
        // SAFETY: `len` is the size of `v`, the buffer written to.
        let status = unsafe {
            RegGetValueW(
                root,
                &subkey,
                &name,
                RRF_RT_ANY,
                None,
                Some((&raw mut v).cast()),
                Some(&raw mut len),
            )
        };
        return if status == ERROR_SUCCESS {
            Ok(RegValue::Qword(v))
        } else {
            Err(format!("RegGetValueW: error {}", status.0))
        };
    }
    if kind == REG_SZ {
        let units = usize::try_from(bytes)
            .map_err(|e| e.to_string())?
            .div_ceil(2);
        let mut buf: Vec<u16> = vec![0; units];
        let mut len = bytes;
        // SAFETY: `buf` holds `bytes` bytes (rounded up to whole UTF-16
        // units), and `len` says so.
        let status = unsafe {
            RegGetValueW(
                root,
                &subkey,
                &name,
                RRF_RT_ANY,
                None,
                Some(buf.as_mut_ptr().cast()),
                Some(&raw mut len),
            )
        };
        if status != ERROR_SUCCESS {
            return Err(format!("RegGetValueW: error {}", status.0));
        }
        let text = buf.split(|&c| c == 0).next().unwrap_or(&[]);
        return Ok(RegValue::Text(String::from_utf16_lossy(text)));
    }
    Ok(RegValue::Other {
        kind: kind.0,
        bytes,
    })
}

/// `SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2)`.
pub(super) fn set_per_monitor_v2() -> Result<(), String> {
    // SAFETY: a documented awareness-context constant; called before any
    // window exists.
    unsafe { SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) }
        .map_err(|e| e.to_string())
}

/// `GetDpiForSystem()`.
pub(super) fn dpi_for_system() -> Result<u32, String> {
    // SAFETY: no arguments; returns a value.
    Ok(unsafe { GetDpiForSystem() })
}

/// `GetSystemMetrics(index)`.
pub(super) fn system_metric(index: SYSTEM_METRICS_INDEX) -> Result<i32, String> {
    // SAFETY: an index constant from the windows crate; returns a value
    // (0 for an index the system does not know).
    Ok(unsafe { GetSystemMetrics(index) })
}

/// `GetSystemMetricsForDpi(index, dpi)`.
pub(super) fn system_metric_for_dpi(index: SYSTEM_METRICS_INDEX, dpi: u32) -> Result<i32, String> {
    // SAFETY: as `system_metric`, plus a DPI value; native-theme's reader
    // makes the same call (native-theme/src/windows.rs).
    Ok(unsafe { GetSystemMetricsForDpi(index, dpi) })
}

/// `GetSysColor(index)`, the COLORREF (0x00BBGGRR).
pub(super) fn sys_color(index: SYS_COLOR_INDEX) -> Result<u32, String> {
    // SAFETY: an index in 0..=30; returns a value (0 for an unknown one).
    Ok(unsafe { GetSysColor(index) })
}

fn empty_ncm() -> Result<NONCLIENTMETRICSW, String> {
    Ok(NONCLIENTMETRICSW {
        cbSize: size_u32::<NONCLIENTMETRICSW>()?,
        ..Default::default()
    })
}

/// `SystemParametersInfoForDpi(SPI_GETNONCLIENTMETRICS, …, dpi)`.
pub(super) fn nonclient_metrics_for_dpi(dpi: u32) -> Result<NONCLIENTMETRICSW, String> {
    let mut ncm = empty_ncm()?;
    // SAFETY: `pvParam` is a NONCLIENTMETRICSW whose cbSize is its size, as
    // native-theme/src/windows.rs passes it.
    unsafe {
        SystemParametersInfoForDpi(
            SPI_GETNONCLIENTMETRICS.0,
            ncm.cbSize,
            Some((&raw mut ncm).cast()),
            0,
            dpi,
        )
    }
    .map_err(|e| e.to_string())?;
    Ok(ncm)
}

/// `SystemParametersInfoW(SPI_GETNONCLIENTMETRICS)`.
pub(super) fn nonclient_metrics() -> Result<NONCLIENTMETRICSW, String> {
    let mut ncm = empty_ncm()?;
    // SAFETY: as `nonclient_metrics_for_dpi`.
    unsafe {
        SystemParametersInfoW(
            SPI_GETNONCLIENTMETRICS,
            ncm.cbSize,
            Some((&raw mut ncm).cast()),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
    }
    .map_err(|e| e.to_string())?;
    Ok(ncm)
}

/// `SystemParametersInfoForDpi(SPI_GETICONTITLELOGFONT, …, dpi)`.
pub(super) fn icon_title_logfont_for_dpi(dpi: u32) -> Result<LOGFONTW, String> {
    let mut lf = LOGFONTW::default();
    let size = size_u32::<LOGFONTW>()?;
    // SAFETY: `pvParam` is a LOGFONTW and `uiParam` its size, as the action
    // documents.
    unsafe {
        SystemParametersInfoForDpi(
            SPI_GETICONTITLELOGFONT.0,
            size,
            Some((&raw mut lf).cast()),
            0,
            dpi,
        )
    }
    .map_err(|e| e.to_string())?;
    Ok(lf)
}

/// `SystemParametersInfoW(SPI_GETICONTITLELOGFONT)`.
pub(super) fn icon_title_logfont() -> Result<LOGFONTW, String> {
    let mut lf = LOGFONTW::default();
    let size = size_u32::<LOGFONTW>()?;
    // SAFETY: as `icon_title_logfont_for_dpi`.
    unsafe {
        SystemParametersInfoW(
            SPI_GETICONTITLELOGFONT,
            size,
            Some((&raw mut lf).cast()),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
    }
    .map_err(|e| e.to_string())?;
    Ok(lf)
}

/// `SystemParametersInfoW(SPI_GETHIGHCONTRAST)`: `dwFlags` and the default
/// scheme name.
pub(super) fn high_contrast() -> Result<(u32, Option<String>), String> {
    let mut hc = HIGHCONTRASTW {
        cbSize: size_u32::<HIGHCONTRASTW>()?,
        ..Default::default()
    };
    // SAFETY: `pvParam` is a HIGHCONTRASTW whose cbSize is its size, as
    // native-theme/src/windows.rs passes it.
    unsafe {
        SystemParametersInfoW(
            SPI_GETHIGHCONTRAST,
            hc.cbSize,
            Some((&raw mut hc).cast()),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
    }
    .map_err(|e| e.to_string())?;
    let scheme = if hc.lpszDefaultScheme.is_null() {
        None
    } else {
        // SAFETY: the system set a non-null, NUL-terminated wide string it
        // owns for the life of the process.
        unsafe { hc.lpszDefaultScheme.to_string() }.ok()
    };
    Ok((hc.dwFlags.0, scheme))
}

/// The actions whose `pvParam` is one 32-bit BOOL, UINT or DWORD.
const U32_ACTIONS: [SYSTEM_PARAMETERS_INFO_ACTION; 21] = [
    SPI_GETCLIENTAREAANIMATION,
    SPI_GETFOCUSBORDERWIDTH,
    SPI_GETFOCUSBORDERHEIGHT,
    SPI_GETCARETWIDTH,
    SPI_GETFLATMENU,
    SPI_GETDROPSHADOW,
    SPI_GETKEYBOARDCUES,
    SPI_GETMENUSHOWDELAY,
    SPI_GETUIEFFECTS,
    SPI_GETCLEARTYPE,
    SPI_GETFONTSMOOTHING,
    SPI_GETFONTSMOOTHINGTYPE,
    SPI_GETFONTSMOOTHINGCONTRAST,
    SPI_GETHOTTRACKING,
    SPI_GETGRADIENTCAPTIONS,
    SPI_GETMENUANIMATION,
    SPI_GETCOMBOBOXANIMATION,
    SPI_GETLISTBOXSMOOTHSCROLLING,
    SPI_GETTOOLTIPANIMATION,
    SPI_GETSELECTIONFADE,
    SPI_GETWHEELSCROLLLINES,
];

/// `SystemParametersInfoW(action, 0, &mut u32)`, for an action in
/// `U32_ACTIONS`; any other action is refused, since it may write more.
pub(super) fn spi_u32(action: SYSTEM_PARAMETERS_INFO_ACTION) -> Result<u32, String> {
    if !U32_ACTIONS.contains(&action) {
        return Err(format!("action {} is not a 32-bit action", action.0));
    }
    let mut v: u32 = 0;
    // SAFETY: every action in `U32_ACTIONS` writes one 32-bit BOOL, UINT or
    // DWORD to `pvParam`, per its SystemParametersInfoW documentation.
    unsafe {
        SystemParametersInfoW(
            action,
            0,
            Some((&raw mut v).cast()),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
    }
    .map_err(|e| e.to_string())?;
    Ok(v)
}

/// `DwmGetColorizationColor`: the 0xAARRGGBB colour and the opaque-blend flag.
pub(super) fn dwm_colorization() -> Result<(u32, bool), String> {
    let mut color: u32 = 0;
    let mut opaque = BOOL::default();
    // SAFETY: two out-pointers to locals, as native-theme/src/windows.rs
    // passes them.
    unsafe { DwmGetColorizationColor(&raw mut color, &raw mut opaque) }
        .map_err(|e| e.to_string())?;
    Ok((color, opaque.as_bool()))
}
