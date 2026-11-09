//! KColorScheme's and KIconTheme's built-in defaults, which the KDE reader
//! and a KDE port's conversion (S14) state where the file they read leaves
//! a key unset, as KDE reads it there
//! (S9.1, S9.3's *The KDE reader's readings*, S4.8, ADD-A140). Each constant
//! cites its row at the pinned source and the KDE fact that cites that row
//! (the KDE facts lane's round 5, d7fe49f2); a row is the constant's value
//! even where the fact's own value, Breeze Light's file, differs from it
//! (`DecorationHover`).
//!
//! KColorScheme's tables are Breeze Light's ("These numbers come from the
//! default color scheme which is currently Breeze Light",
//! `kcolorscheme@6.31.0 src/kcolorscheme.cpp:119-120`), whatever the scheme
//! read.

use crate::color::Color;
use crate::inputs::KdeSchemeKey;
use crate::tokens::{KdeColorEffect, KdeContrastEffect, KdeIntensityEffect};

/// One colour set's default for each of its twelve keys: its table's ten
/// colours, each set's own (`kcolorscheme.cpp:122-210`), and the two
/// decoration colours of `defaultDecorationColors`, every set's
/// (`:213-215`), as `[r, g, b]`.
pub(crate) struct SetDefaults {
    /// `BackgroundNormal`, the table's `Background`.
    background_normal: [u8; 3],
    /// `BackgroundAlternate`, `Alternate`.
    background_alternate: [u8; 3],
    /// `ForegroundNormal`, `Normal`.
    foreground_normal: [u8; 3],
    /// `ForegroundInactive`, `Inactive`.
    foreground_inactive: [u8; 3],
    /// `ForegroundActive`, `Active`.
    foreground_active: [u8; 3],
    /// `ForegroundLink`, `Link`.
    foreground_link: [u8; 3],
    /// `ForegroundVisited`, `Visited`.
    foreground_visited: [u8; 3],
    /// `ForegroundNegative`, `Negative`.
    foreground_negative: [u8; 3],
    /// `ForegroundNeutral`, `Neutral`.
    foreground_neutral: [u8; 3],
    /// `ForegroundPositive`, `Positive`.
    foreground_positive: [u8; 3],
    /// `DecorationFocus`, `defaultDecorationColors`' `Focus`.
    decoration_focus: [u8; 3],
    /// `DecorationHover`, `defaultDecorationColors`' `Hover`.
    decoration_hover: [u8; 3],
}

impl SetDefaults {
    /// The default of `key`.
    pub(crate) fn color(&self, key: KdeSchemeKey) -> Color {
        srgb8(match key {
            KdeSchemeKey::BackgroundNormal => self.background_normal,
            KdeSchemeKey::BackgroundAlternate => self.background_alternate,
            KdeSchemeKey::ForegroundNormal => self.foreground_normal,
            KdeSchemeKey::ForegroundInactive => self.foreground_inactive,
            KdeSchemeKey::ForegroundActive => self.foreground_active,
            KdeSchemeKey::ForegroundLink => self.foreground_link,
            KdeSchemeKey::ForegroundVisited => self.foreground_visited,
            KdeSchemeKey::ForegroundNegative => self.foreground_negative,
            KdeSchemeKey::ForegroundNeutral => self.foreground_neutral,
            KdeSchemeKey::ForegroundPositive => self.foreground_positive,
            KdeSchemeKey::DecorationFocus => self.decoration_focus,
            KdeSchemeKey::DecorationHover => self.decoration_hover,
        })
    }
}

/// The opaque colour of the 8-bit channels `[r, g, b]`.
fn srgb8([r, g, b]: [u8; 3]) -> Color {
    Color {
        r: f32::from(r) / 255.0,
        g: f32::from(g) / 255.0,
        b: f32::from(b) / 255.0,
        a: 1.0,
    }
}

/// `defaultDecorationColors`' `Focus` (`kcolorscheme.cpp:214`), cited on
/// every set's `decoration_focus` fact (`docs/facts/kde/inputs.toml:301`,
/// `:613`, `:925`, `:1237`, `:1549`, `:1861`, `:2173`).
const DECORATION_FOCUS: [u8; 3] = [61, 174, 233];

/// `defaultDecorationColors`' `Hover` (`kcolorscheme.cpp:215`), cited on
/// every set's `decoration_hover` fact (`docs/facts/kde/inputs.toml:327`,
/// `:639`, `:951`, `:1263`, `:1575`, `:1887`, `:2199`), whose value is
/// Breeze Light's `61,174,233`.
const DECORATION_HOVER: [u8; 3] = [147, 206, 233];

/// `View`: `defaultViewColors` (`kcolorscheme.cpp:122-132`), each row cited
/// on its fact in `docs/facts/kde/inputs.toml`.
pub(crate) const VIEW: SetDefaults = SetDefaults {
    background_normal: [255, 255, 255],    // :123, inputs.toml:1601
    background_alternate: [247, 247, 247], // :124, inputs.toml:1627
    foreground_normal: [35, 38, 41],       // :125, inputs.toml:1653
    foreground_inactive: [112, 125, 138],  // :126, inputs.toml:1679
    foreground_active: [61, 174, 233],     // :127, inputs.toml:1705
    foreground_link: [41, 128, 185],       // :128, inputs.toml:1731
    foreground_visited: [155, 89, 182],    // :129, inputs.toml:1757
    foreground_negative: [218, 68, 83],    // :130, inputs.toml:1783
    foreground_neutral: [246, 116, 0],     // :131, inputs.toml:1809
    foreground_positive: [39, 174, 96],    // :132, inputs.toml:1835
    decoration_focus: DECORATION_FOCUS,
    decoration_hover: DECORATION_HOVER,
};

/// `Window`: `defaultWindowColors` (`kcolorscheme.cpp:135-145`), each row
/// cited on its fact in `docs/facts/kde/inputs.toml`.
pub(crate) const WINDOW: SetDefaults = SetDefaults {
    background_normal: [239, 240, 241],    // :136, inputs.toml:1913
    background_alternate: [227, 229, 231], // :137, inputs.toml:1939
    foreground_normal: [35, 38, 41],       // :138, inputs.toml:1965
    foreground_inactive: [112, 125, 138],  // :139, inputs.toml:1991
    foreground_active: [61, 174, 233],     // :140, inputs.toml:2017
    foreground_link: [41, 128, 185],       // :141, inputs.toml:2043
    foreground_visited: [155, 89, 182],    // :142, inputs.toml:2069
    foreground_negative: [218, 68, 83],    // :143, inputs.toml:2095
    foreground_neutral: [246, 116, 0],     // :144, inputs.toml:2121
    foreground_positive: [39, 174, 96],    // :145, inputs.toml:2147
    decoration_focus: DECORATION_FOCUS,
    decoration_hover: DECORATION_HOVER,
};

/// `Button`: `defaultButtonColors` (`kcolorscheme.cpp:148-158`), each row
/// cited on its fact in `docs/facts/kde/inputs.toml`.
pub(crate) const BUTTON: SetDefaults = SetDefaults {
    background_normal: [252, 252, 252],    // :149, inputs.toml:41
    background_alternate: [163, 212, 250], // :150, inputs.toml:67
    foreground_normal: [35, 38, 41],       // :151, inputs.toml:93
    foreground_inactive: [112, 125, 138],  // :152, inputs.toml:119
    foreground_active: [61, 174, 233],     // :153, inputs.toml:145
    foreground_link: [41, 128, 185],       // :154, inputs.toml:171
    foreground_visited: [155, 89, 182],    // :155, inputs.toml:197
    foreground_negative: [218, 68, 83],    // :156, inputs.toml:223
    foreground_neutral: [246, 116, 0],     // :157, inputs.toml:249
    foreground_positive: [39, 174, 96],    // :158, inputs.toml:275
    decoration_focus: DECORATION_FOCUS,
    decoration_hover: DECORATION_HOVER,
};

/// `Selection`: `defaultSelectionColors` (`kcolorscheme.cpp:161-171`), each
/// row cited on its fact in `docs/facts/kde/inputs.toml`.
pub(crate) const SELECTION: SetDefaults = SetDefaults {
    background_normal: [61, 174, 233],     // :162, inputs.toml:977
    background_alternate: [163, 212, 250], // :163, inputs.toml:1003
    foreground_normal: [255, 255, 255],    // :164, inputs.toml:1029
    foreground_inactive: [112, 125, 138],  // :165, inputs.toml:1055
    foreground_active: [255, 255, 255],    // :166, inputs.toml:1081
    foreground_link: [253, 188, 75],       // :167, inputs.toml:1107
    foreground_visited: [155, 89, 182],    // :168, inputs.toml:1133
    foreground_negative: [176, 55, 69],    // :169, inputs.toml:1159
    foreground_neutral: [198, 92, 0],      // :170, inputs.toml:1185
    foreground_positive: [23, 104, 57],    // :171, inputs.toml:1211
    decoration_focus: DECORATION_FOCUS,
    decoration_hover: DECORATION_HOVER,
};

/// `Tooltip`: `defaultTooltipColors` (`kcolorscheme.cpp:174-184`), each row
/// cited on its fact in `docs/facts/kde/inputs.toml`.
pub(crate) const TOOLTIP: SetDefaults = SetDefaults {
    background_normal: [247, 247, 247],    // :175, inputs.toml:1289
    background_alternate: [239, 240, 241], // :176, inputs.toml:1315
    foreground_normal: [35, 38, 41],       // :177, inputs.toml:1341
    foreground_inactive: [112, 125, 138],  // :178, inputs.toml:1367
    foreground_active: [61, 174, 233],     // :179, inputs.toml:1393
    foreground_link: [41, 128, 185],       // :180, inputs.toml:1419
    foreground_visited: [155, 89, 182],    // :181, inputs.toml:1445
    foreground_negative: [218, 68, 83],    // :182, inputs.toml:1471
    foreground_neutral: [246, 116, 0],     // :183, inputs.toml:1497
    foreground_positive: [39, 174, 96],    // :184, inputs.toml:1523
    decoration_focus: DECORATION_FOCUS,
    decoration_hover: DECORATION_HOVER,
};

/// `Complementary`: `defaultComplementaryColors` (`kcolorscheme.cpp:187-197`),
/// each row cited on its fact in `docs/facts/kde/inputs.toml`.
pub(crate) const COMPLEMENTARY: SetDefaults = SetDefaults {
    background_normal: [42, 46, 50],      // :188, inputs.toml:353
    background_alternate: [27, 30, 32],   // :189, inputs.toml:379
    foreground_normal: [252, 252, 252],   // :190, inputs.toml:405
    foreground_inactive: [161, 169, 177], // :191, inputs.toml:431
    foreground_active: [61, 174, 233],    // :192, inputs.toml:457
    foreground_link: [29, 153, 243],      // :193, inputs.toml:483
    foreground_visited: [155, 89, 182],   // :194, inputs.toml:509
    foreground_negative: [218, 68, 83],   // :195, inputs.toml:535
    foreground_neutral: [246, 116, 0],    // :196, inputs.toml:561
    foreground_positive: [39, 174, 96],   // :197, inputs.toml:587
    decoration_focus: DECORATION_FOCUS,
    decoration_hover: DECORATION_HOVER,
};

/// `Header`: `defaultHeaderColors` (`kcolorscheme.cpp:200-210`), beneath the
/// file's `[Colors:Window]` values (`:330-333`), each row cited on its fact
/// in `docs/facts/kde/inputs.toml`.
pub(crate) const HEADER: SetDefaults = SetDefaults {
    background_normal: [222, 224, 226],    // :201, inputs.toml:665
    background_alternate: [239, 240, 241], // :202, inputs.toml:691
    foreground_normal: [35, 38, 41],       // :203, inputs.toml:717
    foreground_inactive: [112, 125, 138],  // :204, inputs.toml:743
    foreground_active: [61, 174, 233],     // :205, inputs.toml:769
    foreground_link: [41, 128, 185],       // :206, inputs.toml:795
    foreground_visited: [155, 89, 182],    // :207, inputs.toml:821
    foreground_negative: [218, 68, 83],    // :208, inputs.toml:847
    foreground_neutral: [246, 116, 0],     // :209, inputs.toml:873
    foreground_positive: [39, 174, 96],    // :210, inputs.toml:899
    decoration_focus: DECORATION_FOCUS,
    decoration_hover: DECORATION_HOVER,
};

/// One colour group's effects as KColorScheme reads them unset
/// (`kcolorscheme.cpp:36-49`), the `[ColorEffects:<group>]` keys.
pub(crate) struct EffectDefaults {
    /// `Enable` (`:39-40`).
    pub(crate) enable: bool,
    /// `IntensityEffect` (`:41`).
    pub(crate) intensity: KdeIntensityEffect,
    /// `ColorEffect` (`:42`).
    pub(crate) color: KdeColorEffect,
    /// `ContrastEffect` (`:43`).
    pub(crate) contrast: KdeContrastEffect,
    /// `IntensityAmount` (`:44`).
    pub(crate) intensity_amount: f32,
    /// `ColorAmount` (`:45`).
    pub(crate) color_amount: f32,
    /// `ContrastAmount` (`:46`).
    pub(crate) contrast_amount: f32,
    /// `Color` (`:47-49`).
    color_color: [u8; 3],
}

impl EffectDefaults {
    /// The default `Color`.
    pub(crate) fn color_color(&self) -> Color {
        srgb8(self.color_color)
    }
}

/// `[ColorEffects:Disabled]`'s defaults, each cited on its `token.kde.disabled`
/// fact in `docs/facts/kde/tokens.toml`.
pub(crate) const DISABLED: EffectDefaults = EffectDefaults {
    enable: true,                          // :39-40, the group being Disabled
    intensity: KdeIntensityEffect::Darken, // :41, tokens.toml:10868
    color: KdeColorEffect::None,           // :42, tokens.toml:10923
    contrast: KdeContrastEffect::Fade,     // :43, tokens.toml:10978
    intensity_amount: 0.10,                // :44, tokens.toml:10898
    color_amount: 0.0,                     // :45, tokens.toml:10953
    contrast_amount: 0.65,                 // :46, tokens.toml:11008
    color_color: [56, 56, 56],             // :47-49, tokens.toml:11033
};

/// `[ColorEffects:Inactive]`'s defaults, each cited on its `token.kde.inactive`
/// fact in `docs/facts/kde/tokens.toml`.
pub(crate) const INACTIVE: EffectDefaults = EffectDefaults {
    enable: false,                       // :39-40, tokens.toml:11248
    intensity: KdeIntensityEffect::None, // :41, tokens.toml:11058
    color: KdeColorEffect::Desaturate,   // :42, tokens.toml:11113
    contrast: KdeContrastEffect::Tint,   // :43, tokens.toml:11168
    intensity_amount: 0.0,               // :44, tokens.toml:11088
    color_amount: -0.9,                  // :45, tokens.toml:11143
    contrast_amount: 0.25,               // :46, tokens.toml:11198
    color_color: [112, 111, 110],        // :47-49, tokens.toml:11223
};

/// `[ColorEffects:Inactive] ChangeSelectionColor` where neither it nor the
/// group's `Enable` is set: `readEntry("Enable", true)` beneath it
/// (`kcolorscheme.cpp:304-316`, `docs/facts/kde/tokens.toml:11268`).
pub(crate) const INACTIVE_CHANGE_SELECTION_COLOR: bool = true;

/// KIconTheme's built-in default sizes, each its group's `defaultSize` in
/// `m_iconGroups` (`kiconthemes@6.31.0 src/kicontheme.cpp:154-157`,
/// `:160-166`), which `<Group>Default` replaces where the theme's
/// `index.theme` states it (`:469-472`), each cited on its
/// `token.icons.sizes` fact in `docs/facts/kde/tokens.toml`.
#[cfg_attr(
    not(all(target_os = "linux", feature = "kde")),
    allow(
        dead_code,
        reason = "the KDE reader's: it reads the icon theme's sizes"
    )
)]
pub(crate) struct IconDefaults {
    /// `Desktop`, 32 (`:161`, `tokens.toml:8777`, `large`).
    pub(crate) desktop: u16,
    /// `Toolbar`, 22 (`:162`, `tokens.toml:8185`, `toolbar`).
    pub(crate) toolbar: u16,
    /// `MainToolbar`, 22 (`:163`, `tokens.toml:8385`, `keyed.main_toolbar`).
    pub(crate) main_toolbar: u16,
    /// `Small`, 16 (`:164`, `tokens.toml:8581`, `small`).
    pub(crate) small: u16,
    /// `Panel`, 48 (`:165`, `tokens.toml:8973`, `panel`).
    pub(crate) panel: u16,
    /// `Dialog`, 32 (`:166`, `tokens.toml:9169`, `dialog`).
    pub(crate) dialog: u16,
}

/// [`IconDefaults`]' values, each row its group's.
#[cfg_attr(
    not(all(target_os = "linux", feature = "kde")),
    allow(
        dead_code,
        reason = "the KDE reader's: it reads the icon theme's sizes"
    )
)]
pub(crate) const ICON_SIZES: IconDefaults = IconDefaults {
    desktop: 32,      // :161
    toolbar: 22,      // :162
    main_toolbar: 22, // :163
    small: 16,        // :164
    panel: 48,        // :165
    dialog: 32,       // :166
};
