//! Windows: the registry's OS version, GetSystemMetrics(ForDpi), GetSysColor,
//! SystemParametersInfo(ForDpi), UISettings and AccessibilitySettings, DWM
//! colorization and the personalization registry values. The Win32 calls,
//! all `unsafe` in windows 0.62, are in [`ffi`]; this file has none.

mod ffi;

use toml::{Table, Value};
use windows::UI::ViewManagement::{AccessibilitySettings, UIColorType, UISettings};
use windows::Win32::Graphics::Gdi::{
    COLOR_3DDKSHADOW, COLOR_3DFACE, COLOR_3DHIGHLIGHT, COLOR_3DHILIGHT, COLOR_3DLIGHT,
    COLOR_3DSHADOW, COLOR_ACTIVEBORDER, COLOR_ACTIVECAPTION, COLOR_APPWORKSPACE, COLOR_BACKGROUND,
    COLOR_BTNFACE, COLOR_BTNHIGHLIGHT, COLOR_BTNHILIGHT, COLOR_BTNSHADOW, COLOR_BTNTEXT,
    COLOR_CAPTIONTEXT, COLOR_DESKTOP, COLOR_GRADIENTACTIVECAPTION, COLOR_GRADIENTINACTIVECAPTION,
    COLOR_GRAYTEXT, COLOR_HIGHLIGHT, COLOR_HIGHLIGHTTEXT, COLOR_HOTLIGHT, COLOR_INACTIVEBORDER,
    COLOR_INACTIVECAPTION, COLOR_INACTIVECAPTIONTEXT, COLOR_INFOBK, COLOR_INFOTEXT, COLOR_MENU,
    COLOR_MENUBAR, COLOR_MENUHILIGHT, COLOR_MENUTEXT, COLOR_SCROLLBAR, COLOR_WINDOW,
    COLOR_WINDOWFRAME, COLOR_WINDOWTEXT, LOGFONTW, SYS_COLOR_INDEX,
};
use windows::Win32::System::Registry::{HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
use windows::Win32::UI::Accessibility::{HCF_HIGHCONTRASTON, HIGHCONTRASTW_FLAGS};
use windows::Win32::UI::WindowsAndMessaging::{
    NONCLIENTMETRICSW, SM_CXBORDER, SM_CXCURSOR, SM_CXDOUBLECLK, SM_CXDRAG, SM_CXEDGE,
    SM_CXFIXEDFRAME, SM_CXFOCUSBORDER, SM_CXFRAME, SM_CXHSCROLL, SM_CXHTHUMB, SM_CXICON,
    SM_CXICONSPACING, SM_CXMENUCHECK, SM_CXMENUSIZE, SM_CXPADDEDBORDER, SM_CXSCREEN, SM_CXSIZE,
    SM_CXSMICON, SM_CXSMSIZE, SM_CXVSCROLL, SM_CYBORDER, SM_CYCAPTION, SM_CYCURSOR, SM_CYDOUBLECLK,
    SM_CYDRAG, SM_CYEDGE, SM_CYFIXEDFRAME, SM_CYFOCUSBORDER, SM_CYFRAME, SM_CYHSCROLL, SM_CYICON,
    SM_CYICONSPACING, SM_CYMENU, SM_CYMENUCHECK, SM_CYMENUSIZE, SM_CYSCREEN, SM_CYSIZE,
    SM_CYSMCAPTION, SM_CYSMICON, SM_CYSMSIZE, SM_CYVSCROLL, SM_CYVTHUMB, SPI_GETCARETWIDTH,
    SPI_GETCLEARTYPE, SPI_GETCLIENTAREAANIMATION, SPI_GETCOMBOBOXANIMATION, SPI_GETDROPSHADOW,
    SPI_GETFLATMENU, SPI_GETFOCUSBORDERHEIGHT, SPI_GETFOCUSBORDERWIDTH, SPI_GETFONTSMOOTHING,
    SPI_GETFONTSMOOTHINGCONTRAST, SPI_GETFONTSMOOTHINGTYPE, SPI_GETGRADIENTCAPTIONS,
    SPI_GETHOTTRACKING, SPI_GETKEYBOARDCUES, SPI_GETLISTBOXSMOOTHSCROLLING, SPI_GETMENUANIMATION,
    SPI_GETMENUSHOWDELAY, SPI_GETSELECTIONFADE, SPI_GETTOOLTIPANIMATION, SPI_GETUIEFFECTS,
    SPI_GETWHEELSCROLLLINES, SYSTEM_METRICS_INDEX, SYSTEM_PARAMETERS_INFO_ACTION,
};

use crate::out;

/// The DPIs the `…ForDpi` calls are asked at: 100 %, 125 %, 150 %, 200 %.
const METRIC_DPIS: [u32; 4] = [96, 120, 144, 192];

/// The DPIs `SystemParametersInfoForDpi` is asked at.
const FONT_DPIS: [u32; 2] = [96, 144];

/// Fill `doc` with the Windows results.
pub(crate) fn probe(doc: &mut Table) {
    out::put_table(doc, "os", os_table());
    out::put_table(doc, "probe", crate::probe_table());

    let mut win = Table::new();
    out::put_table(&mut win, "dpi", dpi_table());
    out::put_table(&mut win, "system_metrics", system_metrics_table());
    out::put_table(&mut win, "sys_colors", sys_colors_table());
    out::put_table(&mut win, "nonclient_metrics", nonclient_metrics_table());
    out::put_table(&mut win, "system_parameters", system_parameters_table());
    out::put_table(&mut win, "ui_settings", ui_settings_table());
    out::put_table(&mut win, "dwm", dwm_table());
    out::put_table(&mut win, "registry", registry_table());
    out::put_table(doc, "windows", win);
}

// ── OS ──────────────────────────────────────────────────────────────────────

const CURRENT_VERSION: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion";

fn os_table() -> Table {
    let mut t = Table::new();
    out::put(&mut t, "platform", out::text("Windows"));
    out::put(
        &mut t,
        "source",
        out::text(format!(r"HKLM\{CURRENT_VERSION}")),
    );
    for name in [
        "ProductName",
        "DisplayVersion",
        "CurrentBuild",
        "UBR",
        "EditionID",
        "InstallationType",
        "CurrentMajorVersionNumber",
        "CurrentMinorVersionNumber",
    ] {
        out::put(
            &mut t,
            name,
            reg_value(HKEY_LOCAL_MACHINE, CURRENT_VERSION, name),
        );
    }
    t
}

fn reg_value(root: HKEY, subkey: &str, name: &str) -> Value {
    match ffi::reg_value(root, subkey, name) {
        Ok(ffi::RegValue::Dword(v)) => out::int(i64::from(v)),
        Ok(ffi::RegValue::Qword(v)) => {
            i64::try_from(v).map_or_else(|_| out::text(v.to_string()), out::int)
        }
        Ok(ffi::RegValue::Text(s)) => out::text(s),
        Ok(ffi::RegValue::Other { kind, bytes }) => {
            out::text(format!("registry type {kind}, {bytes} bytes (not decoded)"))
        }
        Err(reason) => out::unavailable(reason),
    }
}

// ── DPI ─────────────────────────────────────────────────────────────────────

fn dpi_table() -> Table {
    let mut t = Table::new();
    out::put(
        &mut t,
        "SetProcessDpiAwarenessContext_PER_MONITOR_AWARE_V2",
        match ffi::set_per_monitor_v2() {
            Ok(()) => out::text("ok"),
            Err(reason) => out::unavailable(reason),
        },
    );
    out::put(
        &mut t,
        "GetDpiForSystem",
        match ffi::dpi_for_system() {
            Ok(dpi) => out::int(i64::from(dpi)),
            Err(reason) => out::unavailable(reason),
        },
    );
    out::put(
        &mut t,
        "note",
        out::text(
            "GetSystemMetrics and SystemParametersInfoW answer at the system DPI of this per-monitor-aware process",
        ),
    );
    t
}

// ── GetSystemMetrics ────────────────────────────────────────────────────────

macro_rules! named {
    ($($name:ident),* $(,)?) => {
        [$((stringify!($name), $name)),*]
    };
}

fn system_metrics() -> [(&'static str, SYSTEM_METRICS_INDEX); 42] {
    named![
        SM_CXBORDER,
        SM_CYBORDER,
        SM_CXEDGE,
        SM_CYEDGE,
        SM_CXFOCUSBORDER,
        SM_CYFOCUSBORDER,
        SM_CXFIXEDFRAME,
        SM_CYFIXEDFRAME,
        SM_CXFRAME,
        SM_CYFRAME,
        SM_CXPADDEDBORDER,
        SM_CXVSCROLL,
        SM_CYVSCROLL,
        SM_CXHSCROLL,
        SM_CYHSCROLL,
        SM_CXHTHUMB,
        SM_CYVTHUMB,
        SM_CYCAPTION,
        SM_CYSMCAPTION,
        SM_CXSIZE,
        SM_CYSIZE,
        SM_CXSMSIZE,
        SM_CYSMSIZE,
        SM_CYMENU,
        SM_CXMENUSIZE,
        SM_CYMENUSIZE,
        SM_CXMENUCHECK,
        SM_CYMENUCHECK,
        SM_CXICON,
        SM_CYICON,
        SM_CXSMICON,
        SM_CYSMICON,
        SM_CXICONSPACING,
        SM_CYICONSPACING,
        SM_CXCURSOR,
        SM_CYCURSOR,
        SM_CXDRAG,
        SM_CYDRAG,
        SM_CXDOUBLECLK,
        SM_CYDOUBLECLK,
        SM_CXSCREEN,
        SM_CYSCREEN,
    ]
}

fn system_metrics_table() -> Table {
    let mut t = Table::new();
    for (name, index) in system_metrics() {
        let mut m = Table::new();
        for dpi in METRIC_DPIS {
            out::put(
                &mut m,
                &format!("ForDpi_{dpi}"),
                match ffi::system_metric_for_dpi(index, dpi) {
                    Ok(v) => out::int(i64::from(v)),
                    Err(reason) => out::unavailable(reason),
                },
            );
        }
        out::put(
            &mut m,
            "GetSystemMetrics",
            match ffi::system_metric(index) {
                Ok(v) => out::int(i64::from(v)),
                Err(reason) => out::unavailable(reason),
            },
        );
        out::put_table(&mut t, name, m);
    }
    t
}

// ── GetSysColor ─────────────────────────────────────────────────────────────

fn sys_color_names() -> [(&'static str, SYS_COLOR_INDEX); 36] {
    named![
        COLOR_SCROLLBAR,
        COLOR_BACKGROUND,
        COLOR_DESKTOP,
        COLOR_ACTIVECAPTION,
        COLOR_INACTIVECAPTION,
        COLOR_MENU,
        COLOR_WINDOW,
        COLOR_WINDOWFRAME,
        COLOR_MENUTEXT,
        COLOR_WINDOWTEXT,
        COLOR_CAPTIONTEXT,
        COLOR_ACTIVEBORDER,
        COLOR_INACTIVEBORDER,
        COLOR_APPWORKSPACE,
        COLOR_HIGHLIGHT,
        COLOR_HIGHLIGHTTEXT,
        COLOR_BTNFACE,
        COLOR_3DFACE,
        COLOR_BTNSHADOW,
        COLOR_3DSHADOW,
        COLOR_GRAYTEXT,
        COLOR_BTNTEXT,
        COLOR_INACTIVECAPTIONTEXT,
        COLOR_BTNHIGHLIGHT,
        COLOR_BTNHILIGHT,
        COLOR_3DHIGHLIGHT,
        COLOR_3DHILIGHT,
        COLOR_3DDKSHADOW,
        COLOR_3DLIGHT,
        COLOR_INFOTEXT,
        COLOR_INFOBK,
        COLOR_HOTLIGHT,
        COLOR_GRADIENTACTIVECAPTION,
        COLOR_GRADIENTINACTIVECAPTION,
        COLOR_MENUHILIGHT,
        COLOR_MENUBAR,
    ]
}

/// The constants docs/platform-facts.md:355-357 says Microsoft supports on
/// Windows 10 and later.
const WIN10_SUPPORTED: [&str; 8] = [
    "COLOR_WINDOW",
    "COLOR_WINDOWTEXT",
    "COLOR_HIGHLIGHT",
    "COLOR_HIGHLIGHTTEXT",
    "COLOR_3DFACE",
    "COLOR_GRAYTEXT",
    "COLOR_BTNTEXT",
    "COLOR_HOTLIGHT",
];

/// The constants docs/platform-facts.md:360-365 says Microsoft annotates
/// "not supported" on Windows 10 and later.
const WIN10_NOT_SUPPORTED: [&str; 11] = [
    "COLOR_MENU",
    "COLOR_MENUTEXT",
    "COLOR_SCROLLBAR",
    "COLOR_INFOBK",
    "COLOR_INFOTEXT",
    "COLOR_ACTIVECAPTION",
    "COLOR_CAPTIONTEXT",
    "COLOR_INACTIVECAPTION",
    "COLOR_INACTIVECAPTIONTEXT",
    "COLOR_3DSHADOW",
    "COLOR_3DHIGHLIGHT",
];

fn sys_colors_table() -> Table {
    let names = sys_color_names();
    let mut t = Table::new();
    out::put(
        &mut t,
        "format",
        out::text(
            "#rrggbb from the COLORREF; windows10 marks follow docs/platform-facts.md:355-365",
        ),
    );
    for index in 0..=30_i32 {
        let these: Vec<&str> = names
            .iter()
            .filter(|(_, c)| c.0 == index)
            .map(|(n, _)| *n)
            .collect();
        let mut c = Table::new();
        out::put(
            &mut c,
            "names",
            out::array(these.iter().map(|n| out::text(*n)).collect()),
        );
        out::put(
            &mut c,
            "value",
            match ffi::sys_color(SYS_COLOR_INDEX(index)) {
                Ok(colorref) => {
                    let [r, g, b, _] = colorref.to_le_bytes();
                    out::text(format!("#{r:02x}{g:02x}{b:02x}"))
                }
                Err(reason) => out::unavailable(reason),
            },
        );
        let windows10 = if these.iter().any(|n| WIN10_SUPPORTED.contains(n)) {
            "supported (docs/platform-facts.md:355-357)"
        } else if these.iter().any(|n| WIN10_NOT_SUPPORTED.contains(n)) {
            "not supported (docs/platform-facts.md:360-365)"
        } else {
            "not listed in docs/platform-facts.md:355-365"
        };
        out::put(&mut c, "windows10", out::text(windows10));
        out::put_table(&mut t, &format!("index_{index:02}"), c);
    }
    t
}

// ── SystemParametersInfo ────────────────────────────────────────────────────

fn logfont_table(lf: &LOGFONTW, dpi: Option<u32>) -> Table {
    let face = lf.lfFaceName.split(|&c| c == 0).next().unwrap_or(&[]);
    let mut t = Table::new();
    out::put(
        &mut t,
        "lfFaceName",
        out::text(String::from_utf16_lossy(face)),
    );
    out::put(&mut t, "lfHeight", out::int(i64::from(lf.lfHeight)));
    match dpi {
        Some(dpi) if dpi > 0 => out::put(
            &mut t,
            "points",
            out::float(f64::from(lf.lfHeight.unsigned_abs()) * 72.0 / f64::from(dpi)),
        ),
        _ => out::put(
            &mut t,
            "points",
            out::not_probed("the DPI this LOGFONTW is scaled for is not known"),
        ),
    }
    out::put(&mut t, "lfWidth", out::int(i64::from(lf.lfWidth)));
    out::put(&mut t, "lfWeight", out::int(i64::from(lf.lfWeight)));
    out::put(&mut t, "lfItalic", out::int(i64::from(lf.lfItalic)));
    out::put(&mut t, "lfCharSet", out::int(i64::from(lf.lfCharSet.0)));
    out::put(&mut t, "lfQuality", out::int(i64::from(lf.lfQuality.0)));
    out::put(
        &mut t,
        "lfPitchAndFamily",
        out::int(i64::from(lf.lfPitchAndFamily)),
    );
    t
}

fn ncm_table(ncm: &NONCLIENTMETRICSW, dpi: Option<u32>) -> Table {
    let mut t = Table::new();
    out::put(&mut t, "cbSize", out::int(i64::from(ncm.cbSize)));
    for (name, v) in [
        ("iBorderWidth", ncm.iBorderWidth),
        ("iScrollWidth", ncm.iScrollWidth),
        ("iScrollHeight", ncm.iScrollHeight),
        ("iCaptionWidth", ncm.iCaptionWidth),
        ("iCaptionHeight", ncm.iCaptionHeight),
        ("iSmCaptionWidth", ncm.iSmCaptionWidth),
        ("iSmCaptionHeight", ncm.iSmCaptionHeight),
        ("iMenuWidth", ncm.iMenuWidth),
        ("iMenuHeight", ncm.iMenuHeight),
        ("iPaddedBorderWidth", ncm.iPaddedBorderWidth),
    ] {
        out::put(&mut t, name, out::int(i64::from(v)));
    }
    for (name, lf) in [
        ("lfCaptionFont", &ncm.lfCaptionFont),
        ("lfSmCaptionFont", &ncm.lfSmCaptionFont),
        ("lfMenuFont", &ncm.lfMenuFont),
        ("lfStatusFont", &ncm.lfStatusFont),
        ("lfMessageFont", &ncm.lfMessageFont),
    ] {
        out::put_table(&mut t, name, logfont_table(lf, dpi));
    }
    t
}

fn nonclient_metrics_table() -> Table {
    let mut t = Table::new();
    for dpi in FONT_DPIS {
        let key = format!("SystemParametersInfoForDpi_{dpi}");
        match ffi::nonclient_metrics_for_dpi(dpi) {
            Ok(ncm) => out::put_table(&mut t, &key, ncm_table(&ncm, Some(dpi))),
            Err(reason) => out::put(&mut t, &key, out::unavailable(reason)),
        }
    }
    let system_dpi = ffi::dpi_for_system().ok();
    match ffi::nonclient_metrics() {
        Ok(ncm) => out::put_table(&mut t, "SystemParametersInfoW", ncm_table(&ncm, system_dpi)),
        Err(reason) => out::put(&mut t, "SystemParametersInfoW", out::unavailable(reason)),
    }
    t
}

fn spi_values() -> [(&'static str, SYSTEM_PARAMETERS_INFO_ACTION); 21] {
    named![
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
    ]
}

fn system_parameters_table() -> Table {
    let mut t = Table::new();
    out::put(
        &mut t,
        "note",
        out::text("SPI_GETNONCLIENTMETRICS is under nonclient_metrics; BOOL values are 0 or 1"),
    );
    for (name, action) in spi_values() {
        out::put(
            &mut t,
            name,
            match ffi::spi_u32(action) {
                Ok(v) => out::int(i64::from(v)),
                Err(reason) => out::unavailable(reason),
            },
        );
    }

    let mut icon_title = Table::new();
    for dpi in FONT_DPIS {
        let key = format!("SystemParametersInfoForDpi_{dpi}");
        match ffi::icon_title_logfont_for_dpi(dpi) {
            Ok(lf) => out::put_table(&mut icon_title, &key, logfont_table(&lf, Some(dpi))),
            Err(reason) => out::put(&mut icon_title, &key, out::unavailable(reason)),
        }
    }
    match ffi::icon_title_logfont() {
        Ok(lf) => out::put_table(
            &mut icon_title,
            "SystemParametersInfoW",
            logfont_table(&lf, ffi::dpi_for_system().ok()),
        ),
        Err(reason) => out::put(
            &mut icon_title,
            "SystemParametersInfoW",
            out::unavailable(reason),
        ),
    }
    out::put_table(&mut t, "SPI_GETICONTITLELOGFONT", icon_title);

    out::put(
        &mut t,
        "SPI_GETHIGHCONTRAST",
        match ffi::high_contrast() {
            Ok((flags, scheme)) => {
                let mut hc = Table::new();
                out::put(&mut hc, "dwFlags", out::int(i64::from(flags)));
                out::put(
                    &mut hc,
                    "HCF_HIGHCONTRASTON",
                    out::boolean(HIGHCONTRASTW_FLAGS(flags).contains(HCF_HIGHCONTRASTON)),
                );
                out::put(
                    &mut hc,
                    "lpszDefaultScheme",
                    scheme.map_or_else(|| out::unavailable("null"), out::text),
                );
                Value::Table(hc)
            }
            Err(reason) => out::unavailable(reason),
        },
    );
    t
}

// ── UISettings ──────────────────────────────────────────────────────────────

fn ui_settings_table() -> Table {
    let mut t = Table::new();
    match UISettings::new() {
        Ok(settings) => {
            let mut colors = Table::new();
            for (name, kind) in [
                ("Background", UIColorType::Background),
                ("Foreground", UIColorType::Foreground),
                ("AccentDark3", UIColorType::AccentDark3),
                ("AccentDark2", UIColorType::AccentDark2),
                ("AccentDark1", UIColorType::AccentDark1),
                ("Accent", UIColorType::Accent),
                ("AccentLight1", UIColorType::AccentLight1),
                ("AccentLight2", UIColorType::AccentLight2),
                ("AccentLight3", UIColorType::AccentLight3),
                ("Complement", UIColorType::Complement),
            ] {
                out::put(
                    &mut colors,
                    name,
                    match settings.GetColorValue(kind) {
                        Ok(c) => {
                            out::text(format!("#{:02x}{:02x}{:02x}{:02x}", c.R, c.G, c.B, c.A))
                        }
                        Err(e) => out::unavailable(e),
                    },
                );
            }
            out::put_table(&mut t, "GetColorValue", colors);
            out::put(
                &mut t,
                "TextScaleFactor",
                settings
                    .TextScaleFactor()
                    .map_or_else(out::unavailable, out::float),
            );
            out::put(
                &mut t,
                "AdvancedEffectsEnabled",
                settings
                    .AdvancedEffectsEnabled()
                    .map_or_else(out::unavailable, out::boolean),
            );
            out::put(
                &mut t,
                "AnimationsEnabled",
                settings
                    .AnimationsEnabled()
                    .map_or_else(out::unavailable, out::boolean),
            );
            out::put(
                &mut t,
                "AutoHideScrollBars",
                settings
                    .AutoHideScrollBars()
                    .map_or_else(out::unavailable, out::boolean),
            );
        }
        Err(e) => out::put(&mut t, "UISettings", out::unavailable(e)),
    }
    match AccessibilitySettings::new() {
        Ok(acc) => {
            out::put(
                &mut t,
                "AccessibilitySettings_HighContrast",
                acc.HighContrast()
                    .map_or_else(out::unavailable, out::boolean),
            );
            out::put(
                &mut t,
                "AccessibilitySettings_HighContrastScheme",
                acc.HighContrastScheme()
                    .map_or_else(out::unavailable, |s| out::text(s.to_string_lossy())),
            );
        }
        Err(e) => out::put(&mut t, "AccessibilitySettings", out::unavailable(e)),
    }
    t
}

// ── DWM and registry ────────────────────────────────────────────────────────

fn dwm_table() -> Table {
    let mut t = Table::new();
    match ffi::dwm_colorization() {
        Ok((color, opaque_blend)) => {
            let [a, r, g, b] = color.to_be_bytes();
            out::put(
                &mut t,
                "DwmGetColorizationColor",
                out::text(format!("#{r:02x}{g:02x}{b:02x}{a:02x}")),
            );
            out::put(
                &mut t,
                "raw_0xAARRGGBB",
                out::text(format!("{color:#010x}")),
            );
            out::put(&mut t, "opaque_blend", out::boolean(opaque_blend));
        }
        Err(reason) => out::put(&mut t, "DwmGetColorizationColor", out::unavailable(reason)),
    }
    t
}

fn registry_table() -> Table {
    let personalize = r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize";
    let dwm = r"Software\Microsoft\Windows\DWM";
    let mut t = Table::new();
    let mut p = Table::new();
    for name in [
        "AppsUseLightTheme",
        "SystemUsesLightTheme",
        "EnableTransparency",
    ] {
        out::put(
            &mut p,
            name,
            reg_value(HKEY_CURRENT_USER, personalize, name),
        );
    }
    out::put_table(&mut t, &format!(r"HKCU\{personalize}"), p);
    let mut d = Table::new();
    for name in ["ColorPrevalence", "AccentColor"] {
        out::put(&mut d, name, reg_value(HKEY_CURRENT_USER, dwm, name));
    }
    out::put_table(&mut t, &format!(r"HKCU\{dwm}"), d);
    t
}
