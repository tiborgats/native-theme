//! The macOS reader: the appearance, the semantic colours AppKit resolves
//! under both appearances, the accessibility display options, overlay
//! scroll bars and keyboard navigation, into a [`ReaderState`] and a reader
//! layer (S8, S9.3, S9.5); the running macOS version, for the generation
//! table; and the system UI font Core Text resolves, for [`crate::fonts`].
//!
//! Each colour is the `NSColor` class property its preset's fact measured
//! (`docs/facts/macos/roles.toml`), resolved inside
//! `performAsCurrentDrawingAppearance:` under `NSAppearanceNameAqua` and
//! `NSAppearanceNameDarkAqua` and converted to sRGB, as the OS probe reads
//! it, so on a measured OS the reader's layer states the preset's values.
//! The five accent inputs (S9.1) are stated as one group per scheme (S9.3):
//! `input.accent` in the common bases, which any preset takes, the other
//! four under `for_preset.<own>` with the roles, which are macOS's own
//! (S9.5). The roles that reference an input or another role
//! (`role.global.accent`, `role.selection.background`, …) are left to the
//! preset, which states them as those references; the reader reads the role
//! referenced. Of the keyed roles the slice's port reads `role.@system_red`,
//! `@system_orange`, `@system_green`, `@system_blue` and
//! `@selected_control_secondary` (ADD-A138); the others are the readers
//! unit's.
//!
//! While Increase Contrast is on, AppKit resolves both appearances to their
//! increased-contrast colours (the os-probe's `increase-contrast-on` passes
//! of runs 40320895749, 41006842699 and 41195846264, on macOS 14, 15 and
//! 26), so the reads are stated as the inputs' `contrast.increased` bases
//! and the roles as reader rules under `scheme:<s>`, `contrast_increased` and
//! `preset:<own>`; while it is off, as the inputs' per-scheme values and the
//! roles' bases. The other condition's slot is one the platform gives none
//! at, no failure and no `Fallback` (S9.3, ADD-A138), the layers beneath
//! keeping theirs: `appearanceNamed:` returns nil for the high-contrast
//! appearance names (AppKit's `NSAppearance.h:73-74`, Xcode 26.6), and no
//! API resolves the plain colours while the setting is on.
//!
//! The reader patches no text-role slot: System Settings has no font setting
//! (S9.5), and the presets state each style's face from the facts.
//!
//! # Sources
//!
//! What the reader reads that the user's settings change; the watcher
//! observes each through the notification that reports it:
//!
//! - `AppleInterfaceStyle`: light or dark, the global user default.
//! - `NSColor`: the semantic colours, the accent and highlight colours among
//!   them, read under both appearances.
//! - `NSScroller.preferredScrollerStyle`: overlay scroll bars.
//! - `NSWorkspace.accessibilityDisplayOptions`: increase contrast, reduce
//!   motion and reduce transparency.
//!
//! # Sources not watched
//!
//! - `NSApplication.isFullKeyboardAccessEnabled`: keyboard navigation
//!   (System Settings › Keyboard), whose changes key-value observing does
//!   not report (AppKit's `NSApplication.h:377-380`, Xcode 26.6); it shows
//!   at the next read.
//! - `NSProcessInfo.operatingSystemVersion`: the generation, fixed for a
//!   process.
//!
//! `NSScroller.preferredScrollerStyle` and `isFullKeyboardAccessEnabled` are
//! AppKit's main-thread API (`MainThreadMarker`): a read off the main
//! thread leaves them unread, and the state keeps S16's no-reader `false`
//! for both, as the GNOME reader keeps an unread `overlay-scrolling`.

// Objective-C FFI via objc2 -- no safe alternative
#![allow(unsafe_code)]

#[cfg(all(target_os = "macos", feature = "macos"))]
use block2::RcBlock;
#[cfg(all(target_os = "macos", feature = "macos"))]
use objc2::rc::Retained;
#[cfg(all(target_os = "macos", feature = "macos"))]
use objc2_app_kit::{
    NSAppearance, NSApplication, NSColor, NSColorSpace, NSScroller, NSScrollerStyle, NSWorkspace,
};
#[cfg(all(target_os = "macos", feature = "macos"))]
use objc2_foundation::{NSString, NSUserDefaults};

use crate::blocks::BlockPatch;
use crate::color::Color;
use crate::error::ReaderFailure;
use crate::inputs::{Contrast, InputName, InputValue};
use crate::key::Key;
use crate::layer::{Bases, Layer};
use crate::presets::PresetId;
use crate::reader::Reading;
use crate::resolve::ReaderState;
use crate::roles::{
    ColorRolesPatch, DisabledRolesPatch, GlobalRolesPatch, SurfaceRolesPatch, SurfaceSet,
};
use crate::selector::{Condition, Rule, RuleBody, Scheme, Selector, SetMatch};
use crate::value::{SchemeMap, SurfaceMap, Value};

/// The light appearance's name, which `appearanceNamed:` resolves.
const AQUA: &str = "NSAppearanceNameAqua";

/// The dark appearance's name.
const DARK_AQUA: &str = "NSAppearanceNameDarkAqua";

/// The text-scaling factor of unscaled text, S16's no-reader state's: the
/// identity of S9.4's scaling, not a platform value. macOS's Text size
/// setting has no public API to read (MAC-X26).
const UNSCALED: f32 = 1.0;

/// The first macOS major version with `performAsCurrentDrawingAppearance:`
/// (`API_AVAILABLE(macos(11.0))`, AppKit's `NSAppearance.h:36`, Xcode 26.6).
/// Below it the selector does not exist, so the reader does not send it.
const DRAWING_APPEARANCE_MAJOR: u32 = 11;

/// Whether `version` has `performAsCurrentDrawingAppearance:`.
#[cfg_attr(
    not(all(target_os = "macos", feature = "macos")),
    allow(dead_code, reason = "the macOS reader's: not linked in this build")
)]
fn has_drawing_appearance(version: crate::pipeline::OsVersion) -> bool {
    version.major >= DRAWING_APPEARANCE_MAJOR
}

/// One appearance's colours as AppKit resolved them, each `None` where it
/// has no sRGB form. Each field's doc names the `NSColor` class property
/// the presets' fact for its target measured, and the target.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    not(all(target_os = "macos", feature = "macos")),
    allow(dead_code, reason = "the macOS reader's: not linked in this build")
)]
pub(crate) struct AppearanceColors {
    /// `controlAccentColor`: `input.accent`.
    pub(crate) accent: Option<Color>,
    /// `selectedContentBackgroundColor`: `input.selected_content_background`.
    pub(crate) selected_content_background: Option<Color>,
    /// `selectedTextBackgroundColor`: `input.highlight`.
    pub(crate) highlight: Option<Color>,
    /// `keyboardFocusIndicatorColor`: `input.keyboard_focus_indicator`.
    pub(crate) keyboard_focus_indicator: Option<Color>,
    /// `selectedControlColor`: `input.selected_control`.
    pub(crate) selected_control: Option<Color>,
    /// `windowBackgroundColor`: `role.window.background`.
    pub(crate) window_background: Option<Color>,
    /// `labelColor`: `role.window.foreground`.
    pub(crate) label: Option<Color>,
    /// `secondaryLabelColor`: `role.window.foreground_inactive`.
    pub(crate) secondary_label: Option<Color>,
    /// `linkColor`: `role.window.link`.
    pub(crate) link: Option<Color>,
    /// `separatorColor`: `role.window.border`.
    pub(crate) separator: Option<Color>,
    /// `tertiaryLabelColor`: `role.window.disabled.foreground`.
    pub(crate) tertiary_label: Option<Color>,
    /// `controlBackgroundColor`: `role.view.background`.
    pub(crate) control_background: Option<Color>,
    /// `alternatingContentBackgroundColors[1]`:
    /// `role.view.background_alternate`.
    pub(crate) alternating_content_background: Option<Color>,
    /// `alternateSelectedControlTextColor`: `role.selection.foreground` and
    /// `role.global.accent_foreground`.
    pub(crate) alternate_selected_control_text: Option<Color>,
    /// `selectedTextColor`: `role.global.selection_text_foreground`.
    pub(crate) selected_text: Option<Color>,
    /// `systemRedColor`: `role.@system_red`.
    pub(crate) system_red: Option<Color>,
    /// `systemOrangeColor`: `role.@system_orange`.
    pub(crate) system_orange: Option<Color>,
    /// `systemGreenColor`: `role.@system_green`.
    pub(crate) system_green: Option<Color>,
    /// `systemBlueColor`: `role.@system_blue`.
    pub(crate) system_blue: Option<Color>,
    /// `secondarySelectedControlColor`: `role.@selected_control_secondary`,
    /// the inactive selection the presets state
    /// `role.selection.background` as under `window_inactive`.
    pub(crate) secondary_selected_control: Option<Color>,
}

#[cfg_attr(
    not(all(target_os = "macos", feature = "macos")),
    allow(dead_code, reason = "the macOS reader's: not linked in this build")
)]
impl AppearanceColors {
    /// The five accent inputs, in the presets' `accent_inputs` order: each
    /// input's key, the property read for it and its colour.
    fn accent_inputs(&self) -> [(Key, &'static str, Option<Color>); 5] {
        [
            (crate::key!("accent"), "controlAccentColor", self.accent),
            (
                crate::key!("selected_content_background"),
                "selectedContentBackgroundColor",
                self.selected_content_background,
            ),
            (
                crate::key!("highlight"),
                "selectedTextBackgroundColor",
                self.highlight,
            ),
            (
                crate::key!("keyboard_focus_indicator"),
                "keyboardFocusIndicatorColor",
                self.keyboard_focus_indicator,
            ),
            (
                crate::key!("selected_control"),
                "selectedControlColor",
                self.selected_control,
            ),
        ]
    }

    /// The colour-role patch of what was read: every role read, none the
    /// preset states as a reference to an input.
    fn roles(&self) -> ColorRolesPatch {
        let stated = |color: Option<Color>| color.map(Value::Stated);
        let window = SurfaceRolesPatch {
            background: stated(self.window_background),
            foreground: stated(self.label),
            foreground_inactive: stated(self.secondary_label),
            link: stated(self.link),
            border: stated(self.separator),
            disabled: self.tertiary_label.map(|color| DisabledRolesPatch {
                foreground: Some(Value::Stated(color)),
                ..DisabledRolesPatch::default()
            }),
            ..SurfaceRolesPatch::default()
        };
        let view = SurfaceRolesPatch {
            background: stated(self.control_background),
            background_alternate: stated(self.alternating_content_background),
            ..SurfaceRolesPatch::default()
        };
        let selection = SurfaceRolesPatch {
            foreground: stated(self.alternate_selected_control_text),
            ..SurfaceRolesPatch::default()
        };
        let mut surfaces = SurfaceMap::default();
        let mut any_surface = false;
        for (set, patch) in [
            (SurfaceSet::Window, window),
            (SurfaceSet::View, view),
            (SurfaceSet::Selection, selection),
        ] {
            if patch != SurfaceRolesPatch::default() {
                surfaces.insert(set, BlockPatch::Fields(patch));
                any_surface = true;
            }
        }
        let global = GlobalRolesPatch {
            accent_foreground: stated(self.alternate_selected_control_text),
            selection_text_foreground: stated(self.selected_text),
            ..GlobalRolesPatch::default()
        };
        let keyed: Vec<(Key, Value<Color>)> = [
            (crate::key!("system_red"), self.system_red),
            (crate::key!("system_orange"), self.system_orange),
            (crate::key!("system_green"), self.system_green),
            (crate::key!("system_blue"), self.system_blue),
            (
                crate::key!("selected_control_secondary"),
                self.secondary_selected_control,
            ),
        ]
        .into_iter()
        .filter_map(|(key, color)| Some((key, Value::Stated(color?))))
        .collect();
        ColorRolesPatch {
            surfaces: any_surface.then_some(surfaces),
            global: (global != GlobalRolesPatch::default()).then_some(global),
            keyed: (!keyed.is_empty()).then_some(keyed),
            ..ColorRolesPatch::default()
        }
    }
}

/// What AppKit gave, in plain types: built from its reads on macOS,
/// directly in tests.
#[cfg_attr(
    not(all(target_os = "macos", feature = "macos")),
    allow(dead_code, reason = "the macOS reader's: not linked in this build")
)]
pub(crate) struct MacosData {
    /// The scheme: dark where `AppleInterfaceStyle` is `Dark`.
    pub(crate) scheme: Scheme,
    /// `accessibilityDisplayShouldIncreaseContrast`.
    pub(crate) increase_contrast: bool,
    /// `accessibilityDisplayShouldReduceMotion`.
    pub(crate) reduce_motion: bool,
    /// `accessibilityDisplayShouldReduceTransparency`.
    pub(crate) reduce_transparency: bool,
    /// Whether `preferredScrollerStyle` is the overlay style; `None`: not
    /// read, off the main thread.
    pub(crate) overlay_scrollbars: Option<bool>,
    /// `isFullKeyboardAccessEnabled`; `None`: not read, off the main thread.
    pub(crate) full_keyboard_access: Option<bool>,
    /// The colours under [`AQUA`]; `None`: AppKit gives no such appearance.
    pub(crate) light: Option<AppearanceColors>,
    /// The colours under [`DARK_AQUA`]; `None`: AppKit gives no such
    /// appearance.
    pub(crate) dark: Option<AppearanceColors>,
}

/// The reader's state and layer from what AppKit gave, its own preset's
/// part under `own` (S9.5). Zero I/O.
///
/// - The state: the scheme; `Contrast::Increased` under Increase Contrast
///   (macOS has one contrast condition); reduced motion and reduced
///   transparency; overlay scroll bars and full keyboard access where they
///   were read, else `false`; text unscaled ([`UNSCALED`]); no font DPI,
///   macOS having no setting, so the preset's `token.text.font_dpi` of 72
///   converts its points (S9.4).
/// - Per appearance read, the five accent inputs as one group: none of them
///   at a scheme where one has no sRGB form or the appearance is missing,
///   reported as the status's fallback, naming the input (S9.3).
///   `input.accent` in the common bases, the other four under
///   `for_preset.<own>`; per-scheme values, or `contrast.increased` bases
///   under Increase Contrast.
/// - Per appearance read, the roles read: `colors.<scheme>` under
///   `for_preset.<own>`, or under Increase Contrast a rule on the colours
///   under `scheme:<s>`, `contrast_increased` and `preset:<own>`.
#[cfg_attr(
    not(all(target_os = "macos", feature = "macos")),
    allow(dead_code, reason = "the macOS reader's: not linked in this build")
)]
pub(crate) fn build_reading(data: &MacosData, own: PresetId) -> Reading {
    let mut state = ReaderState::new(data.scheme, UNSCALED);
    state.contrast = data.increase_contrast.then_some(Contrast::Increased);
    state.reduced_motion = data.reduce_motion;
    state.reduced_transparency = data.reduce_transparency;
    state.overlay_scrollbars = data.overlay_scrollbars.unwrap_or(false);
    state.full_keyboard_access = data.full_keyboard_access.unwrap_or(false);

    let mut layer = Layer::default();
    let mut own_bases = Bases::default();
    let mut group: [SchemeMap<Value<Color>>; 5] = Default::default();
    let mut fallback = None;
    for (scheme, name, colors) in [
        (Scheme::Light, AQUA, &data.light),
        (Scheme::Dark, DARK_AQUA, &data.dark),
    ] {
        let Some(colors) = colors else {
            fallback.get_or_insert_with(|| {
                ReaderFailure::Platform(format!(
                    "input.accent: AppKit gives no {name} appearance to read the accent inputs under"
                ))
            });
            continue;
        };
        let inputs = colors.accent_inputs();
        match inputs.iter().find(|(_, _, color)| color.is_none()) {
            Some((key, property, _)) => {
                fallback.get_or_insert_with(|| {
                    ReaderFailure::Platform(format!(
                        "input.{key}: NSColor.{property} has no sRGB form under {name}"
                    ))
                });
            }
            None => {
                for (slot, (_, _, color)) in group.iter_mut().zip(inputs) {
                    *slot.slot(scheme) = color.map(Value::Stated);
                }
            }
        }
        let roles = colors.roles();
        if data.increase_contrast {
            let selector = Selector {
                conditions: SetMatch {
                    all_of: vec![
                        Condition::Scheme(scheme),
                        Condition::ContrastIncreased,
                        Condition::Preset(own),
                    ],
                    none_of: Vec::new(),
                },
                ..Selector::default()
            };
            layer
                .rules
                .push(Rule::new(selector, RuleBody::Colors(Box::new(roles))));
        } else {
            *own_bases.colors.slot(scheme) = Some(roles);
        }
    }

    let names = AppearanceColors::default().accent_inputs();
    for ((key, _, _), per_scheme) in names.into_iter().zip(group) {
        if per_scheme.light.is_none() && per_scheme.dark.is_none() {
            continue;
        }
        let value = if data.increase_contrast {
            InputValue {
                per_scheme: SchemeMap::default(),
                per_contrast: vec![(Contrast::Increased, per_scheme)],
            }
        } else {
            InputValue::new(per_scheme)
        };
        let bases = if key == crate::key!("accent") {
            &mut layer.bases
        } else {
            &mut own_bases
        };
        bases.inputs.entries.push((InputName::from_key(key), value));
    }
    if own_bases != Bases::default() {
        layer.for_preset.push((own, own_bases));
    }
    Reading {
        state,
        layer,
        fallback,
    }
}

/// The colour `NSColor`'s sRGB components give: `None` where a component is
/// not finite or the alpha lies outside `0..=1` (S2.1).
#[cfg_attr(
    not(all(target_os = "macos", feature = "macos")),
    allow(dead_code, reason = "the macOS reader's: not linked in this build")
)]
fn color_from_components(r: f64, g: f64, b: f64, a: f64) -> Option<Color> {
    let finite = [r, g, b, a].iter().all(|c| c.is_finite());
    (finite && (0.0..=1.0).contains(&a)).then_some(Color {
        r: r as f32,
        g: g as f32,
        b: b as f32,
        a: a as f32,
    })
}

/// `color` in sRGB (`colorUsingColorSpace:`, which converts Display P3
/// colours), as the OS probe reads it; `None` where it has no sRGB form (a
/// pattern colour).
#[cfg(all(target_os = "macos", feature = "macos"))]
fn srgb(color: &NSColor, space: &NSColorSpace) -> Option<Color> {
    let color = color.colorUsingColorSpace(space)?;
    color_from_components(
        color.redComponent(),
        color.greenComponent(),
        color.blueComponent(),
        color.alphaComponent(),
    )
}

/// The colours of the appearance current for drawing; called inside
/// `performAsCurrentDrawingAppearance:`, so that dynamic colours resolve to
/// it.
#[cfg(all(target_os = "macos", feature = "macos"))]
fn drawing_appearance_colors() -> AppearanceColors {
    let space = NSColorSpace::sRGBColorSpace();
    let read = |color: Retained<NSColor>| srgb(&color, &space);
    // The property `role.@selected_control_secondary`'s facts measured
    // (ADD-A138), deprecated from macOS 11 for
    // `unemphasizedSelectedContentBackgroundColor`, which AppKit's header
    // calls its alias (`NSColor.h:222`, `:429`, Xcode 26.6).
    #[allow(deprecated, reason = "the property the presets' facts measured")]
    let secondary_selected_control = NSColor::secondarySelectedControlColor();
    AppearanceColors {
        accent: read(NSColor::controlAccentColor()),
        selected_content_background: read(NSColor::selectedContentBackgroundColor()),
        highlight: read(NSColor::selectedTextBackgroundColor()),
        keyboard_focus_indicator: read(NSColor::keyboardFocusIndicatorColor()),
        selected_control: read(NSColor::selectedControlColor()),
        window_background: read(NSColor::windowBackgroundColor()),
        label: read(NSColor::labelColor()),
        secondary_label: read(NSColor::secondaryLabelColor()),
        link: read(NSColor::linkColor()),
        separator: read(NSColor::separatorColor()),
        tertiary_label: read(NSColor::tertiaryLabelColor()),
        control_background: read(NSColor::controlBackgroundColor()),
        alternating_content_background: NSColor::alternatingContentBackgroundColors()
            .iter()
            .nth(1)
            .and_then(read),
        alternate_selected_control_text: read(NSColor::alternateSelectedControlTextColor()),
        selected_text: read(NSColor::selectedTextColor()),
        system_red: read(NSColor::systemRedColor()),
        system_orange: read(NSColor::systemOrangeColor()),
        system_green: read(NSColor::systemGreenColor()),
        system_blue: read(NSColor::systemBlueColor()),
        secondary_selected_control: read(secondary_selected_control),
    }
}

/// `appearance`'s colours: [`drawing_appearance_colors`] inside
/// `performAsCurrentDrawingAppearance:`; every colour `None` should AppKit
/// not run the block.
#[cfg(all(target_os = "macos", feature = "macos"))]
fn appearance_colors(appearance: &NSAppearance) -> AppearanceColors {
    let colors = std::cell::Cell::new(None);
    {
        let block = RcBlock::new(|| colors.set(Some(drawing_appearance_colors())));
        appearance.performAsCurrentDrawingAppearance(&block);
    }
    colors.into_inner().unwrap_or_default()
}

/// The scheme `AppleInterfaceStyle` gives: dark where it is `Dark`; the
/// default is absent in light mode, so any other answer is light.
#[cfg(all(target_os = "macos", feature = "macos"))]
fn system_scheme() -> Scheme {
    let style = NSUserDefaults::standardUserDefaults()
        .stringForKey(&NSString::from_str("AppleInterfaceStyle"));
    if style.is_some_and(|style| style.to_string().eq_ignore_ascii_case("dark")) {
        Scheme::Dark
    } else {
        Scheme::Light
    }
}

/// Read the macOS settings, its own preset's part under `own`.
///
/// Refused on macOS before 11, which lacks
/// `performAsCurrentDrawingAppearance:`, and where AppKit gives neither
/// appearance. Off the main thread the scroller style and keyboard
/// navigation are not read (`MainThreadMarker`).
#[cfg(all(target_os = "macos", feature = "macos"))]
pub(crate) fn read(own: PresetId) -> Result<Reading, ReaderFailure> {
    let version = os_version();
    if !has_drawing_appearance(version) {
        return Err(ReaderFailure::Platform(format!(
            "macOS {version} lacks performAsCurrentDrawingAppearance:, which the reader resolves the colours with (macOS 11 and later)"
        )));
    }
    let light = NSAppearance::appearanceNamed(&NSString::from_str(AQUA));
    let dark = NSAppearance::appearanceNamed(&NSString::from_str(DARK_AQUA));
    if light.is_none() && dark.is_none() {
        return Err(ReaderFailure::Platform(format!(
            "AppKit gives neither the {AQUA} nor the {DARK_AQUA} appearance"
        )));
    }
    let workspace = NSWorkspace::sharedWorkspace();
    let main_thread = objc2::MainThreadMarker::new();
    let data = MacosData {
        scheme: system_scheme(),
        increase_contrast: workspace.accessibilityDisplayShouldIncreaseContrast(),
        reduce_motion: workspace.accessibilityDisplayShouldReduceMotion(),
        reduce_transparency: workspace.accessibilityDisplayShouldReduceTransparency(),
        overlay_scrollbars: main_thread
            .map(|mtm| NSScroller::preferredScrollerStyle(mtm) == NSScrollerStyle::Overlay),
        full_keyboard_access: main_thread
            .map(|mtm| NSApplication::sharedApplication(mtm).isFullKeyboardAccessEnabled()),
        light: light.as_deref().map(appearance_colors),
        dark: dark.as_deref().map(appearance_colors),
    };
    Ok(build_reading(&data, own))
}

/// The running macOS version, from `NSProcessInfo.operatingSystemVersion`
/// (a safe binding in objc2-foundation 0.3.2), for the generation table.
/// AppKit's components are never negative; one that were would read as 0, and
/// a version of 0.0.0 gets `macos-monterey` with `VersionMatch::PresetNewer`.
/// Early macOS 26 betas reported 16 here to a binary built against the macOS
/// 15 SDK; `pipeline::MACOS_GENERATIONS` gives the sources and the mapping.
/// The SF Symbols loader reads it too (`sficons::has_symbol_images`).
#[cfg(all(target_os = "macos", any(feature = "macos", feature = "system-icons")))]
pub(crate) fn os_version() -> crate::pipeline::OsVersion {
    let v = objc2_foundation::NSProcessInfo::processInfo().operatingSystemVersion();
    let part = |n: isize| u32::try_from(n).unwrap_or_default();
    crate::pipeline::OsVersion::new(
        part(v.majorVersion),
        part(v.minorVersion),
        part(v.patchVersion),
    )
}

/// The macOS system UI font as Core Text resolves it: its family name and
/// the file it lives in (egui connector spec §8.2).
#[cfg(all(target_os = "macos", feature = "system-fonts"))]
pub(crate) struct SystemUiFont {
    /// Core Text's family name for the font (`kCTFontFamilyNameAttribute`).
    pub(crate) family: String,
    /// The font file (`kCTFontURLAttribute` as a file path).
    pub(crate) path: std::path::PathBuf,
}

/// Ask Core Text which font is the system UI font — `CTFontUIFontType::System`
/// at the type's default size, no language — and, for `italic`, its italic
/// variant (the upright font where Core Text has none). `None` where Core
/// Text returns none, an attribute is missing or is not the type the
/// attribute documents, or the URL is not a file path. `CTFont::family_name`
/// is not used: its binding `expect`s a non-null result (objc2-core-text
/// 0.3.2 `src/generated/CTFont.rs` lines 977–983).
#[cfg(all(target_os = "macos", feature = "system-fonts"))]
pub(crate) fn system_ui_font(italic: bool) -> Option<SystemUiFont> {
    use objc2_core_foundation::{CFString, CFURL};
    use objc2_core_text::{
        CTFont, CTFontSymbolicTraits, CTFontUIFontType, kCTFontFamilyNameAttribute,
        kCTFontURLAttribute,
    };
    // SAFETY: `CTFontCreateUIFontForLanguage` takes a UI font type, a size
    // (0.0 = the type's default) and an optional language; a null language
    // is documented as allowed, and the binding maps a NULL result to None.
    let upright = unsafe { CTFont::new_ui_font_for_language(CTFontUIFontType::System, 0.0, None) }?;
    let font = if italic {
        // SAFETY: a null matrix is documented as allowed by the binding's
        // safety note; the value and mask both name the italic trait.
        let slanted = unsafe {
            upright.copy_with_symbolic_traits(
                0.0,
                std::ptr::null(),
                CTFontSymbolicTraits::ItalicTrait,
                CTFontSymbolicTraits::ItalicTrait,
            )
        };
        slanted.unwrap_or(upright)
    } else {
        upright
    };
    // SAFETY: the two keys are Core Text's own statics, declared in `extern`
    // blocks; reading them is an FFI read of a constant.
    let (family_key, url_key) = unsafe { (kCTFontFamilyNameAttribute, kCTFontURLAttribute) };
    // SAFETY: `CTFontCopyAttribute` returns a retained reference or NULL,
    // which the binding maps to None; the downcasts check the type id.
    let family = unsafe { font.attribute(family_key) }?
        .downcast::<CFString>()
        .ok()?
        .to_string();
    // SAFETY: as above: `url_key` is Core Text's own static, `font.attribute`
    // returns a retained reference or NULL (mapped to None), and the downcast
    // checks the type id.
    let path = unsafe { font.attribute(url_key) }?
        .downcast::<CFURL>()
        .ok()?
        .to_file_path()?;
    Some(SystemUiFont { family, path })
}

#[cfg(test)]
pub(crate) mod tests;
