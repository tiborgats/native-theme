//! The button pass (`--button-pass`): AppKit's buttons in titled windows,
//! one window (a sheet) at a time, captured per appearance and state with
//! `screencapture`. Each state is captured over the window's own background
//! and over three opaque backdrops (black, white, grey) drawn behind the
//! buttons, so each button's own layer, its colour and alpha, is solved from
//! the black and white captures and checked against the grey and the window's
//! ([`layer`]). The window's key status is read just before and after each
//! capture.
//!
//! The sheets: `push`, the push buttons of spike M (plain, default, toggled
//! off and on) at every control size; `styles_a` to `styles_c`, every other
//! bezel style, border shape, tint and content the catalogue's `button` entry
//! maps a `macos-*` variant to, at every control size. The default mode also
//! captures the hovered and clicked push buttons, a toolbar ([`extras`]), a
//! text view's caret, and reads the buttons' geometry and fonts without a
//! window. The other modes are the conditions: each button focused in turn
//! (`--focus`), a second window of the probe key (`--not-key`), and another
//! application active (`--inactive`).

mod extras;
mod transition;

use std::cell::Cell;
use std::path::{Path, PathBuf};

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{ClassType, MainThreadMarker, MainThreadOnly, sel};
use objc2_app_kit::{
    NSAppearance, NSAppearanceCustomization, NSApplication, NSApplicationActivationPolicy,
    NSBezelStyle, NSBitmapImageRep, NSBox, NSBoxType, NSButton, NSButtonType, NSCellImagePosition,
    NSColor, NSComboButton, NSComboButtonStyle, NSControl, NSControlBorderShape, NSControlSize,
    NSEvent, NSEventMask, NSEventType, NSImage, NSResponder, NSScreen, NSTintProminence,
    NSTitlePosition, NSView, NSWindow, NSWorkspace,
};
use objc2_core_graphics::{
    CGEvent, CGEventTapLocation, CGEventType, CGMouseButton, CGPreflightPostEventAccess,
    CGWarpMouseCursorPosition,
};
use objc2_foundation::{NSData, NSDate, NSOperatingSystemVersion, NSPoint, NSRect, NSSize};
use toml::{Table, Value};

use super::layer::{Layer, PxRect, Rgba, hex};
use super::{
    ACTIVATE_WAIT_STEPS, APPEARANCES, KEY_WAIT_STEPS, PUMP_STEP, SETTLE_STEPS, activate_ignoring,
    activation_table, control_common, ffi, font_table, insets, not_key_window, ns, pixel_index,
    point_text, pump, rect, screencapture, size, state_value, wait_or_kill,
};
use crate::out;

/// The control sizes the pass places, with the macOS major version each
/// needs: `NSControlSizeLarge` is macOS 11+, `NSControlSizeExtraLarge` 26+.
const BUTTON_SIZES: [(&str, NSControlSize, isize); 5] = [
    ("mini", NSControlSize::Mini, 0),
    ("small", NSControlSize::Small, 0),
    ("regular", NSControlSize::Regular, 0),
    ("large", NSControlSize::Large, 11),
    ("extraLarge", NSControlSize::ExtraLarge, 26),
];

/// Every button's title.
const BUTTON_TITLE: &str = "Button";

/// The SF Symbol the image buttons show.
const SYMBOL_NAME: &str = "star";

/// The push sheet's content frame origin on the screen, in points.
const BUTTON_WINDOW_ORIGIN: NSPoint = NSPoint::new(200.0, 200.0);

/// The space round the grid of buttons in the content view, in points.
const BUTTON_MARGIN: f64 = 20.0;

/// The distance from one push column's left edge to the next one's.
const BUTTON_COLUMN_STEP: f64 = 140.0;

/// The height of a row; each button is centred in its row, on whole points.
const BUTTON_ROW_STEP: f64 = 56.0;

/// The push sheet's content size: four columns, five rows.
const BUTTON_CONTENT_SIZE: NSSize = NSSize::new(
    2.0 * BUTTON_MARGIN + 4.0 * BUTTON_COLUMN_STEP,
    2.0 * BUTTON_MARGIN + 5.0 * BUTTON_ROW_STEP,
);

/// The style sheets' content frame origin on the screen, in points.
const STYLE_WINDOW_ORIGIN: NSPoint = NSPoint::new(40.0, 60.0);

/// The distance from one size column's left edge to the next one's in a
/// style sheet.
const STYLE_COLUMN_STEP: f64 = 150.0;

/// The rows (styles) of one style sheet.
const STYLE_ROWS: usize = 7;

/// A style sheet's content size: five size columns, seven style rows.
const STYLE_CONTENT_SIZE: NSSize = NSSize::new(
    2.0 * BUTTON_MARGIN + 5.0 * STYLE_COLUMN_STEP,
    2.0 * BUTTON_MARGIN + 7.0 * BUTTON_ROW_STEP,
);

/// How far round a button's frame its pixels are compared between two
/// captures and its layer is read, in points, so a ring, glow or shadow
/// outside the frame counts.
const DIFF_MARGIN: f64 = 6.0;

/// The grey backdrop's sRGB byte, the same in each channel.
const GREY_BYTE: u8 = 128;

/// How far right of the window's frame the cursor is parked, in points.
const CURSOR_AWAY_OFFSET: f64 = 100.0;

/// The steps the pump runs after the cursor moves, so a hover the button
/// animates in has settled before the capture.
const HOVER_STEPS: u32 = 20;

/// How long after the mouse-down the click capture is taken.
const CLICK_CAPTURE_DELAY: std::time::Duration = std::time::Duration::from_millis(1000);

/// The most steps the pump waits for a click capture and its mouse-up.
const CLICK_WAIT_STEPS: u32 = 300;

/// The application the inactive mode activates where deactivating the
/// probe leaves it active.
const OTHER_APPLICATION: &str = "com.apple.finder";

/// The condition the pass captures its buttons in.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mode {
    /// The window key, the application active.
    Key,
    /// The window key, each button made first responder in turn.
    Focus,
    /// A second window of the probe key and main.
    NotKey,
    /// The probe's application deactivated, another one active.
    Inactive,
    /// The window key, two buttons focused and unfocused while the screen
    /// is recorded ([`transition`]).
    FocusTransition,
}

impl Mode {
    fn name(self) -> &'static str {
        match self {
            Mode::Key => "key",
            Mode::Focus => "focus",
            Mode::NotKey => "not_key",
            Mode::Inactive => "inactive",
            Mode::FocusTransition => "focus_transition",
        }
    }

    fn how(self) -> &'static str {
        match self {
            Mode::Key => {
                "each sheet's window made key and main, the application active; the states normal, pressed and disabled over every backdrop, on the push sheet also hovered and clicked over the window's background; then the toolbar and the caret windows and the windowless geometry"
            }
            Mode::Focus => {
                "each sheet's window made key; per button (every push sheet button, the regular size of every style) makeFirstResponder: with the button, then the window captured over every backdrop; the button's acceptsFirstResponder and canBecomeKeyView and NSApp's isFullKeyboardAccessEnabled are recorded, and a button that does not become first responder is not captured"
            }
            Mode::NotKey => {
                "each sheet's window made key, then a second, small titled window of the probe made key and front (the not_key table): the sheet's window neither key nor main, the application active; the states normal, pressed and disabled over every backdrop; then the toolbar window"
            }
            Mode::Inactive => {
                "each sheet's window made key (the application activated first where it is not active), then -[NSApplication deactivate] and, where the application is still active after the pump, `open -b com.apple.finder` (the inactive table); the states normal, pressed and disabled over every backdrop; then the toolbar window"
            }
            Mode::FocusTransition => {
                "no sheet is captured; the push sheet's window made key, and two of its buttons focused and unfocused while screencapture -v records the screen (focus_transition)"
            }
        }
    }
}

/// What a capture's buttons are drawn over.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Backdrop {
    /// The window's own background: no backdrop drawn.
    Window,
    Black,
    White,
    Grey,
}

impl Backdrop {
    const ALL: [Backdrop; 4] = [
        Backdrop::Window,
        Backdrop::Black,
        Backdrop::White,
        Backdrop::Grey,
    ];

    fn key(self) -> &'static str {
        match self {
            Backdrop::Window => "window",
            Backdrop::Black => "over_black",
            Backdrop::White => "over_white",
            Backdrop::Grey => "over_grey",
        }
    }

    fn suffix(self) -> &'static str {
        match self {
            Backdrop::Window => "",
            Backdrop::Black => "-over-black",
            Backdrop::White => "-over-white",
            Backdrop::Grey => "-over-grey",
        }
    }

    /// The sRGB byte the backdrop is filled with, in each channel.
    fn byte(self) -> Option<u8> {
        match self {
            Backdrop::Window => None,
            Backdrop::Black => Some(0),
            Backdrop::White => Some(255),
            Backdrop::Grey => Some(GREY_BYTE),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Variant {
    Plain,
    Default,
    ToggleOff,
    ToggleOn,
}

impl Variant {
    const ALL: [Variant; 4] = [
        Variant::Plain,
        Variant::Default,
        Variant::ToggleOff,
        Variant::ToggleOn,
    ];

    fn name(self) -> &'static str {
        match self {
            Variant::Plain => "plain",
            Variant::Default => "default",
            Variant::ToggleOff => "toggle_off",
            Variant::ToggleOn => "toggle_on",
        }
    }

    fn setup(self) -> &'static str {
        match self {
            Variant::Plain => "buttonType momentaryPushIn",
            Variant::Default => "buttonType momentaryPushIn, keyEquivalent \"\\r\" (Return)",
            Variant::ToggleOff => "buttonType pushOnPushOff, state off",
            Variant::ToggleOn => "buttonType pushOnPushOff, state on",
        }
    }

    /// Whether the pass clicks it: a click flips a toggle's state.
    fn clicked(self) -> bool {
        matches!(self, Variant::Plain | Variant::Default)
    }
}

/// A row of the style sheets: a bezel style, border shape, tint or content
/// of a push-type button, or an `NSComboButton`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Style {
    FlexiblePush,
    SmallSquare,
    ShadowlessSquare,
    AccessoryBar,
    AccessoryBarAction,
    Badge,
    Circular,
    Help,
    Toolbar,
    Glass,
    Flat,
    Primary,
    Tinted,
    Destructive,
    Capsule,
    RoundedRectangle,
    Circle,
    ComboSplit,
    ComboUnified,
    IconOnly,
    WithIcon,
}

impl Style {
    const ALL: [Style; 21] = [
        Style::FlexiblePush,
        Style::SmallSquare,
        Style::ShadowlessSquare,
        Style::AccessoryBar,
        Style::AccessoryBarAction,
        Style::Badge,
        Style::Circular,
        Style::Help,
        Style::Toolbar,
        Style::Glass,
        Style::Flat,
        Style::Primary,
        Style::Tinted,
        Style::Destructive,
        Style::Capsule,
        Style::RoundedRectangle,
        Style::Circle,
        Style::ComboSplit,
        Style::ComboUnified,
        Style::IconOnly,
        Style::WithIcon,
    ];

    fn name(self) -> &'static str {
        match self {
            Style::FlexiblePush => "flexible_push",
            Style::SmallSquare => "small_square",
            Style::ShadowlessSquare => "shadowless_square",
            Style::AccessoryBar => "accessory_bar",
            Style::AccessoryBarAction => "accessory_bar_action",
            Style::Badge => "badge",
            Style::Circular => "circular",
            Style::Help => "help",
            Style::Toolbar => "toolbar",
            Style::Glass => "glass",
            Style::Flat => "flat",
            Style::Primary => "primary",
            Style::Tinted => "tinted",
            Style::Destructive => "destructive",
            Style::Capsule => "capsule",
            Style::RoundedRectangle => "rounded_rectangle",
            Style::Circle => "circle",
            Style::ComboSplit => "combo_split",
            Style::ComboUnified => "combo_unified",
            Style::IconOnly => "icon_only",
            Style::WithIcon => "with_icon",
        }
    }

    fn setup(self) -> &'static str {
        match self {
            Style::FlexiblePush => "bezelStyle flexiblePush (2)",
            Style::SmallSquare => "bezelStyle smallSquare (10)",
            Style::ShadowlessSquare => "bezelStyle shadowlessSquare (6)",
            Style::AccessoryBar => "bezelStyle accessoryBar (13; recessed is its old name)",
            Style::AccessoryBarAction => {
                "bezelStyle accessoryBarAction (12; roundRect is its old name)"
            }
            Style::Badge => "bezelStyle badge (15; inline is its old name)",
            Style::Circular => {
                "bezelStyle circular (7), the symbol image only (imagePosition imageOnly), empty title"
            }
            Style::Help => "bezelStyle helpButton (9), empty title",
            Style::Toolbar => "bezelStyle toolbar (11; texturedRounded is its old name)",
            Style::Glass => "bezelStyle glass (16), macOS 26",
            Style::Flat => "bezelStyle push, setBordered: NO",
            Style::Primary => "bezelStyle push, setTintProminence: primary (2), macOS 26",
            Style::Tinted => "bezelStyle push, setTintProminence: secondary (3), macOS 26",
            Style::Destructive => "bezelStyle push, setHasDestructiveAction: YES",
            Style::Capsule => "bezelStyle push, setBorderShape: capsule (1), macOS 26",
            Style::RoundedRectangle => {
                "bezelStyle push, setBorderShape: roundedRectangle (2), macOS 26"
            }
            Style::Circle => {
                "bezelStyle push, setBorderShape: circle (3), the symbol image only, empty title, macOS 26"
            }
            Style::ComboSplit => "NSComboButton, style split (0), its default menu, macOS 13",
            Style::ComboUnified => "NSComboButton, style unified (1), its default menu, macOS 13",
            Style::IconOnly => "bezelStyle push, the symbol image only (imageOnly), empty title",
            Style::WithIcon => "bezelStyle push, the symbol image and the title (imageLeading)",
        }
    }

    /// The macOS major version the style needs.
    fn needs(self) -> isize {
        match self {
            Style::Glass
            | Style::Primary
            | Style::Tinted
            | Style::Capsule
            | Style::RoundedRectangle
            | Style::Circle => 26,
            Style::ComboSplit | Style::ComboUnified => 13,
            _ => 0,
        }
    }
}

/// One button of a sheet.
struct Case {
    name: String,
    size: NSControlSize,
    kind: &'static str,
    setup: &'static str,
    control: Retained<NSControl>,
    button: Option<Retained<NSButton>>,
    /// Whether the default mode clicks it (the push sheet's regular plain
    /// and default buttons).
    clickable: bool,
    /// Whether the focus mode focuses it.
    focus: bool,
}

/// A window of buttons with its backdrop.
struct Sheet {
    name: &'static str,
    window: Retained<NSWindow>,
    content: Retained<NSView>,
    backdrop: Retained<NSBox>,
    cases: Vec<Case>,
    /// Whether the hovered and clicked states are captured.
    pointer_states: bool,
    /// The rows or columns the OS has no control for, and the sheet's layout.
    notes: Table,
}

impl Sheet {
    /// Draw `backdrop` behind the buttons: hide the backdrop box for the
    /// window's own background, fill it with the backdrop's colour otherwise.
    fn show(&self, backdrop: Backdrop) {
        match backdrop.byte() {
            None => self.backdrop.setHidden(true),
            Some(b) => {
                let c = f64::from(b) / 255.0;
                self.backdrop
                    .setFillColor(&NSColor::colorWithSRGBRed_green_blue_alpha(c, c, c, 1.0));
                self.backdrop.setHidden(false);
            }
        }
        self.content.setNeedsDisplay(true);
    }

    /// The point each capture reads its backdrop at, in points from the
    /// window frame's top left: the middle of the content view's left margin.
    fn backdrop_point(&self) -> NSPoint {
        let frame = self.window.frame().size;
        let content = self.content.frame().size;
        NSPoint::new(BUTTON_MARGIN / 2.0, frame.height - content.height / 2.0)
    }

    /// The directory and the artifact-relative folder of one appearance's
    /// captures: `<appearance>` for the push sheet, as spike M wrote them,
    /// `<appearance>/<sheet>` for the others.
    fn folder(&self, out_dir: &Path, key: &str) -> (PathBuf, String) {
        if self.name == "push" {
            (out_dir.join(key), key.to_owned())
        } else {
            (
                out_dir.join(key).join(self.name),
                format!("{key}/{}", self.name),
            )
        }
    }
}

/// A sheet's window, its content view and its backdrop box.
type SheetParts = (Retained<NSWindow>, Retained<NSView>, Retained<NSBox>);

/// A titled window holding a backdrop box over its whole content view.
fn sheet_window(
    mtm: MainThreadMarker,
    origin: NSPoint,
    content_size: NSSize,
) -> Result<SheetParts, String> {
    let window = ffi::titled_window(mtm, NSRect::new(origin, content_size))?;
    window.setTitle(&ns(""));
    let content = window
        .contentView()
        .ok_or_else(|| "the titled window has no content view".to_owned())?;
    let backdrop = NSBox::initWithFrame(
        NSBox::alloc(mtm),
        NSRect::new(NSPoint::new(0.0, 0.0), content_size),
    );
    backdrop.setBoxType(NSBoxType::Custom);
    backdrop.setTitlePosition(NSTitlePosition::NoTitle);
    backdrop.setBorderWidth(0.0);
    backdrop.setHidden(true);
    content.addSubview(&backdrop);
    Ok((window, content, backdrop))
}

/// Place `control`, sized to fit, centred in its row on whole points.
fn place(control: &NSControl, content: &NSView, x: f64, row: f64, content_height: f64) {
    control.sizeToFit();
    let fitted = control.frame().size;
    let row_bottom = content_height - BUTTON_MARGIN - (row + 1.0) * BUTTON_ROW_STEP;
    control.setFrameOrigin(NSPoint::new(
        x,
        (row_bottom + (BUTTON_ROW_STEP - fitted.height) / 2.0).floor(),
    ));
    content.addSubview(control);
}

/// The push sheet: a row per control size, a column per variant.
fn push_sheet(mtm: MainThreadMarker, version: &NSOperatingSystemVersion) -> Result<Sheet, String> {
    let (window, content, backdrop) = sheet_window(mtm, BUTTON_WINDOW_ORIGIN, BUTTON_CONTENT_SIZE)?;
    let mut notes = Table::new();
    let mut cases = Vec::new();
    let mut row = 0.0_f64;
    for (size_name, control_size, needs) in BUTTON_SIZES {
        if version.majorVersion < needs {
            out::put(
                &mut notes,
                size_name,
                out::unavailable(format!("{size_name} needs macOS {needs}")),
            );
            row += 1.0;
            continue;
        }
        let mut column = 0.0_f64;
        for variant in Variant::ALL {
            let b = NSButton::new(mtm);
            match variant {
                Variant::ToggleOff | Variant::ToggleOn => {
                    b.setButtonType(NSButtonType::PushOnPushOff);
                    b.setState(state_value(variant == Variant::ToggleOn));
                }
                Variant::Plain | Variant::Default => {
                    b.setButtonType(NSButtonType::MomentaryPushIn);
                }
            }
            if variant == Variant::Default {
                b.setKeyEquivalent(&ns("\r"));
            }
            b.setBezelStyle(NSBezelStyle::Push);
            b.setTitle(&ns(BUTTON_TITLE));
            control_common(&b, control_size);
            place(
                &b,
                &content,
                BUTTON_MARGIN + column * BUTTON_COLUMN_STEP,
                row,
                BUTTON_CONTENT_SIZE.height,
            );
            cases.push(Case {
                name: format!("{size_name}-{}", variant.name()),
                size: control_size,
                kind: variant.name(),
                setup: variant.setup(),
                control: Retained::into_super(b.clone()),
                button: Some(b),
                clickable: control_size == NSControlSize::Regular && variant.clicked(),
                focus: true,
            });
            column += 1.0;
        }
        row += 1.0;
    }
    out::put(
        &mut notes,
        "layout",
        out::text(format!(
            "content {} x {} pt from {}; a row per control size (mini, small, regular, large, extraLarge) from the top, {BUTTON_ROW_STEP} pt apart, a column per variant (plain, default, toggle_off, toggle_on) every {BUTTON_COLUMN_STEP} pt from x {BUTTON_MARGIN}; every NSButton bezelStyle push, title \"{BUTTON_TITLE}\"",
            BUTTON_CONTENT_SIZE.width,
            BUTTON_CONTENT_SIZE.height,
            point_text(BUTTON_WINDOW_ORIGIN),
        )),
    );
    Ok(Sheet {
        name: "push",
        window,
        content,
        backdrop,
        cases,
        pointer_states: true,
        notes,
    })
}

/// A style sheet: a row per style of `styles`, a column per control size.
fn style_sheet(
    mtm: MainThreadMarker,
    version: &NSOperatingSystemVersion,
    name: &'static str,
    styles: &[Style],
) -> Result<Sheet, String> {
    let (window, content, backdrop) = sheet_window(mtm, STYLE_WINDOW_ORIGIN, STYLE_CONTENT_SIZE)?;
    let mut notes = Table::new();
    let mut cases = Vec::new();
    let mut row = 0.0_f64;
    for style in styles {
        if version.majorVersion < style.needs() {
            out::put(
                &mut notes,
                style.name(),
                out::unavailable(format!("{} needs macOS {}", style.name(), style.needs())),
            );
            row += 1.0;
            continue;
        }
        let mut column = 0.0_f64;
        for (size_name, control_size, needs) in BUTTON_SIZES {
            if version.majorVersion < needs {
                column += 1.0;
                continue;
            }
            match style_control(mtm, *style, control_size) {
                Ok((control, button)) => {
                    place(
                        &control,
                        &content,
                        BUTTON_MARGIN + column * STYLE_COLUMN_STEP,
                        row,
                        STYLE_CONTENT_SIZE.height,
                    );
                    cases.push(Case {
                        name: format!("{size_name}-{}", style.name()),
                        size: control_size,
                        kind: style.name(),
                        setup: style.setup(),
                        control,
                        button,
                        clickable: false,
                        focus: control_size == NSControlSize::Regular,
                    });
                }
                Err(reason) => out::put(
                    &mut notes,
                    &format!("{size_name}-{}", style.name()),
                    out::unavailable(reason),
                ),
            }
            column += 1.0;
        }
        row += 1.0;
    }
    out::put(
        &mut notes,
        "layout",
        out::text(format!(
            "content {} x {} pt from {}; a row per style from the top, {BUTTON_ROW_STEP} pt apart, a column per control size (mini, small, regular, large, extraLarge) every {STYLE_COLUMN_STEP} pt from x {BUTTON_MARGIN}; title \"{BUTTON_TITLE}\" unless the style's setup says otherwise; the symbol image is imageWithSystemSymbolName: \"{SYMBOL_NAME}\"",
            STYLE_CONTENT_SIZE.width,
            STYLE_CONTENT_SIZE.height,
            point_text(STYLE_WINDOW_ORIGIN),
        )),
    );
    Ok(Sheet {
        name,
        window,
        content,
        backdrop,
        cases,
        pointer_states: false,
        notes,
    })
}

/// The SF Symbol the image buttons show.
fn symbol_image() -> Result<Retained<NSImage>, String> {
    NSImage::imageWithSystemSymbolName_accessibilityDescription(&ns(SYMBOL_NAME), None)
        .ok_or_else(|| format!("imageWithSystemSymbolName: \"{SYMBOL_NAME}\" is nil"))
}

/// One style's control at `size`, given the control size and its system
/// font; the button where it is an NSButton.
fn style_control(
    mtm: MainThreadMarker,
    style: Style,
    size: NSControlSize,
) -> Result<(Retained<NSControl>, Option<Retained<NSButton>>), String> {
    if matches!(style, Style::ComboSplit | Style::ComboUnified) {
        let combo = NSComboButton::new(mtm);
        combo.setTitle(&ns(BUTTON_TITLE));
        combo.setStyle(if style == Style::ComboSplit {
            NSComboButtonStyle::Split
        } else {
            NSComboButtonStyle::Unified
        });
        control_common(&combo, size);
        return Ok((Retained::into_super(combo), None));
    }
    let b = NSButton::new(mtm);
    b.setButtonType(NSButtonType::MomentaryPushIn);
    b.setTitle(&ns(BUTTON_TITLE));
    b.setBezelStyle(NSBezelStyle::Push);
    let class = NSButton::class();
    let image_only = |b: &NSButton| -> Result<(), String> {
        let image = symbol_image()?;
        b.setImage(Some(&image));
        b.setImagePosition(NSCellImagePosition::ImageOnly);
        b.setTitle(&ns(""));
        Ok(())
    };
    let tint = |b: &NSButton, p: NSTintProminence| -> Result<(), String> {
        if !class.responds_to(sel!(setTintProminence:)) {
            return Err("NSButton does not respond to setTintProminence:".to_owned());
        }
        b.setTintProminence(p);
        Ok(())
    };
    let shape = |b: &NSButton, s: NSControlBorderShape| -> Result<(), String> {
        if !class.responds_to(sel!(setBorderShape:)) {
            return Err("NSButton does not respond to setBorderShape:".to_owned());
        }
        b.setBorderShape(s);
        Ok(())
    };
    match style {
        Style::FlexiblePush => b.setBezelStyle(NSBezelStyle::FlexiblePush),
        Style::SmallSquare => b.setBezelStyle(NSBezelStyle::SmallSquare),
        // Deprecated in the SDK, with no replacement: still a style AppKit draws.
        #[allow(deprecated)]
        Style::ShadowlessSquare => b.setBezelStyle(NSBezelStyle::ShadowlessSquare),
        Style::AccessoryBar => b.setBezelStyle(NSBezelStyle::AccessoryBar),
        Style::AccessoryBarAction => b.setBezelStyle(NSBezelStyle::AccessoryBarAction),
        Style::Badge => b.setBezelStyle(NSBezelStyle::Badge),
        Style::Circular => {
            b.setBezelStyle(NSBezelStyle::Circular);
            image_only(&b)?;
        }
        Style::Help => {
            b.setBezelStyle(NSBezelStyle::HelpButton);
            b.setTitle(&ns(""));
        }
        Style::Toolbar => b.setBezelStyle(NSBezelStyle::Toolbar),
        Style::Glass => b.setBezelStyle(NSBezelStyle::Glass),
        Style::Flat => b.setBordered(false),
        Style::Primary => tint(&b, NSTintProminence::Primary)?,
        Style::Tinted => tint(&b, NSTintProminence::Secondary)?,
        Style::Destructive => b.setHasDestructiveAction(true),
        Style::Capsule => shape(&b, NSControlBorderShape::Capsule)?,
        Style::RoundedRectangle => shape(&b, NSControlBorderShape::RoundedRectangle)?,
        Style::Circle => {
            shape(&b, NSControlBorderShape::Circle)?;
            image_only(&b)?;
        }
        Style::IconOnly => image_only(&b)?,
        Style::WithIcon => {
            let image = symbol_image()?;
            b.setImage(Some(&image));
            b.setImagePosition(NSCellImagePosition::ImageLeading);
        }
        Style::ComboSplit | Style::ComboUnified => {}
    }
    control_common(&b, size);
    Ok((Retained::into_super(b.clone()), Some(b)))
}

/// What every capture of the pass shares.
struct Pass<'a> {
    mtm: MainThreadMarker,
    app: &'a NSApplication,
    mode: Mode,
    out_dir: &'a Path,
    /// The primary screen's height, which turns AppKit's screen points
    /// (bottom-left origin) into Quartz's global ones (top-left origin).
    primary_height: f64,
    /// Whether every capture so far was taken while its window was key.
    all_key: Cell<bool>,
    /// Whether any capture so far was taken while its window was key.
    any_key: Cell<bool>,
}

/// `--button-pass`: activate the application, then capture each sheet in
/// `mode`.
pub(super) fn button_pass_table(
    mtm: MainThreadMarker,
    app: &NSApplication,
    out_dir: &Path,
    version: &NSOperatingSystemVersion,
    mode: Mode,
) -> Table {
    let mut t = Table::new();
    out::put(&mut t, "mode", out::text(mode.name()));
    out::put(&mut t, "mode_method", out::text(mode.how()));
    out::put(&mut t, "method", out::text(method_text()));
    out::put_table(&mut t, "states", states_table());
    out::put(&mut t, "backdrops", out::text(backdrops_text()));
    out::put(&mut t, "layer_method", out::text(layer_text()));

    out::put(
        &mut t,
        "activationPolicy_regular_set",
        out::boolean(app.setActivationPolicy(NSApplicationActivationPolicy::Regular)),
    );
    app.finishLaunching();
    out::put_table(&mut t, "activation", activation_table(app));
    out::put(
        &mut t,
        "CGPreflightPostEventAccess",
        out::boolean(CGPreflightPostEventAccess()),
    );
    out::put(
        &mut t,
        "isFullKeyboardAccessEnabled",
        out::boolean(app.isFullKeyboardAccessEnabled()),
    );

    let primary_height = NSScreen::screens(mtm)
        .iter()
        .next()
        .map(|screen| screen.frame().size.height);
    let Some(primary_height) = primary_height else {
        out::put(
            &mut t,
            "error",
            out::unavailable("NSScreen screens is empty"),
        );
        return t;
    };
    let pass = Pass {
        mtm,
        app,
        mode,
        out_dir,
        primary_height,
        all_key: Cell::new(true),
        any_key: Cell::new(false),
    };

    if mode == Mode::FocusTransition {
        out::put_table(
            &mut t,
            "focus_transition",
            transition::transition_table(&pass, version),
        );
        out::put(&mut t, "all_captures_key", out::boolean(pass.all_key.get()));
        out::put(&mut t, "any_capture_key", out::boolean(pass.any_key.get()));
        return t;
    }

    let mut sheets = Table::new();
    let style_groups: Vec<&[Style]> = Style::ALL.chunks(STYLE_ROWS).collect();
    let mut built: Vec<Result<Sheet, String>> = vec![push_sheet(mtm, version)];
    for (styles, name) in style_groups
        .iter()
        .zip(["styles_a", "styles_b", "styles_c"])
    {
        built.push(style_sheet(mtm, version, name, styles));
    }
    let names = ["push", "styles_a", "styles_b", "styles_c"];
    for (sheet, name) in built.into_iter().zip(names) {
        match sheet {
            Ok(sheet) => out::put_table(&mut sheets, name, pass.sheet(&sheet)),
            Err(reason) => out::put(&mut sheets, name, out::unavailable(reason)),
        }
    }
    out::put_table(&mut t, "sheets", sheets);

    if mode != Mode::Focus {
        out::put_table(&mut t, "toolbar", extras::toolbar_table(&pass));
    }
    if mode == Mode::Key {
        out::put_table(&mut t, "caret", extras::caret_table(&pass));
        out::put_table(&mut t, "geometry", extras::geometry_table(mtm, version));
    }
    out::put(&mut t, "all_captures_key", out::boolean(pass.all_key.get()));
    out::put(&mut t, "any_capture_key", out::boolean(pass.any_key.get()));
    t
}

fn method_text() -> String {
    format!(
        "activation policy set to Regular, finishLaunching and activation as in the key-window section; each sheet is a titled NSWindow (closable, miniaturizable, resizable, empty title) holding a backdrop (an NSBox, custom, no border, no title, over the whole content view, hidden for the window's own background) and a grid of controls, each given setControlSize: and the systemFontOfSize: of systemFontSizeForControlSize: and sized to fit, centred in its row on whole points; the window is made key with makeKeyAndOrderFront: and makeFirstResponder: nil, the run loop pumped until isKeyWindow, then the mode's condition is set (mode_method); per appearance the window's appearance is set and the run loop pumped {SETTLE_STEPS} steps of {} ms, then each state is set (states) and per backdrop the window displayed, the run loop pumped {SETTLE_STEPS} steps ({HOVER_STEPS} after a cursor move) and the window captured with screencapture -x -o -l <windowNumber> to <appearance>/[<sheet>/]<state>[-<button>][-over-<backdrop>].png (the push sheet's captures in <appearance>/, as spike M wrote them); isKeyWindow and NSApp isActive are read just before and just after each capture (key_at_capture: both reads of isKeyWindow are true); each capture over the window's background is compared with the same appearance's normal one inside each button's frame grown by {DIFF_MARGIN} pt (diff_from_normal: the pixels whose bytes differ and the largest difference of one byte); the cursor is parked {CURSOR_AWAY_OFFSET} pt right of the push sheet's frame before the normal capture and after the hovered and clicked ones",
        PUMP_STEP.as_millis(),
    )
}

fn states_table() -> Table {
    let mut states = Table::new();
    for (state, how) in [
        (
            "normal",
            "nothing set; on the push sheet the cursor parked outside the window",
        ),
        (
            "pressed",
            "-[NSButton highlight:] YES on every NSButton (an NSComboButton has no highlight:), as NSButtonCell's mouse tracking does, then the run loop pumped; the cells' isHighlighted read back (cells_highlighted)",
        ),
        ("disabled", "setEnabled: NO on every control"),
        (
            "hovered",
            "push sheet, key mode, over the window's background: per button the cursor warped to the button's centre (CGWarpMouseCursorPosition) and a mouse-moved CGEvent posted there at the HID tap, then the run loop pumped; the mouse events AppKit dispatched meanwhile are counted (cursor.events)",
        ),
        (
            "clicked",
            "push sheet, key mode, over the window's background: per plain and default button at the regular size the cursor moved there as for hovered, a left mouse-down CGEvent posted at the HID tap, and from a second thread the window captured after the delay and a mouse-up posted after the capture, while the main thread's run loop runs NSButtonCell's tracking; the window's key status is read before the mouse-down and after the mouse-up",
        ),
        (
            "focused",
            "focus mode: per button makeFirstResponder: with the button, captured over every backdrop, then makeFirstResponder: nil after the sheet's buttons",
        ),
    ] {
        out::put(&mut states, state, out::text(how));
    }
    states
}

fn backdrops_text() -> String {
    format!(
        "every state but hovered and clicked is captured four times: over the window's own background (window: the capture's file is <state>.png, its keys beside these), and with the backdrop box filled with colorWithSRGBRed:green:blue:alpha: of the bytes 0 (over_black), 255 (over_white) and {GREY_BYTE} (over_grey) in each channel; backdrop_raw is the capture's pixel at the middle of the content view's left margin ({} pt in, half the content's height up), as #rrggbbaa",
        BUTTON_MARGIN / 2.0
    )
}

fn layer_text() -> String {
    format!(
        "layer: per pixel, from the over_black and over_white captures and their backdrop_raw bytes B, alpha a = 1 - (C_white - C_black) / (B_white - B_black) per channel, averaged over r, g, b (alpha_channel_spread_max_bytes: the largest difference of the three, times 255) and clamped to 0..1, and the premultiplied colour P = C_black - (1 - a) B_black clamped to 0..a; the layer is written as a PNG of straight (not premultiplied) colour and alpha, tagged sRGB, to <stem>-layer.png (a focused button's: its region only); check_grey and check_window composite the layer over the over_grey and window captures' backdrop_raw and compare it with those captures' opaque pixels (max_byte_delta, pixels_over_2; not_opaque_pixels, the window's rounded corners, are not compared), for the whole file and per button; per button (buttons), in the frame and the frame grown by {DIFF_MARGIN} pt (region): alpha_max; body_alpha, the median alpha of the frame's middle row's pixels of alpha >= 8/255; fill_mode, the most frequent straight colour and alpha among the frame's pixels whose alpha is within 8/255 of body_alpha; bezel, the bounds of the region's pixels of alpha >= half body_alpha; fill, the straight colour and alpha at fill_point_px_xy, on the bezel's middle row two pixels inside the first pixel from its left edge whose alpha reaches 0.98 of the row's median (clear of the corner and the label; a gradient bezel's colour on that row); corner_top_left and corner_top_right, per row from the bezel's top the missing coverage (1 - alpha / the row's median alpha of its pixels of alpha >= 8/255, clamped to 0..1) summed from the bezel's edge inward to the first pixel at 0.98 coverage, up to two rows under 0.02, half the bezel's height or 40 rows, and the radius of the circle whose insets fit those rows best (least squares, 0.25 px steps; fit_rms_px says how well a circle fits); ink, the bezel's pixels of alpha >= half body_alpha whose straight colour or alpha differs from the fill's by more than 48 of 255 in a channel, their bounds and the one farthest from the fill (farthest_from_fill: the label's colour where a glyph covers a whole pixel); outside_frame, the region's pixels outside the frame with alpha >= 8/255, their bounds and the one of the largest alpha; diff_from_unfocused (focus mode), the region's pixels whose premultiplied colour or alpha differs from the unfocused layer's by more than 2 of 255, their bounds and the straight colour where the alpha grew most"
    )
}

/// The capture taken over the window's own background, and the layer
/// solved from the black and white ones.
struct Shots {
    window: Option<Rgba>,
    layer: Option<Layer>,
}

impl Pass<'_> {
    /// One sheet: its buttons' setup and geometry, its window made key and
    /// the mode's condition set, and each appearance's captures.
    fn sheet(&self, sheet: &Sheet) -> Table {
        let mut t = sheet.notes.clone();
        let mut buttons = Table::new();
        let frame_size = sheet.window.frame().size;
        for case in &sheet.cases {
            out::put_table(&mut buttons, &case.name, case_table(case, frame_size));
        }
        out::put_table(&mut t, "buttons", buttons);

        let other = self.present(&sheet.window, &mut t);
        out::put(
            &mut t,
            "window_backingScaleFactor",
            out::float(sheet.window.backingScaleFactor()),
        );
        let window_frame = sheet.window.frame();
        out::put(&mut t, "window_frame_xywh", rect(window_frame));
        out::put(
            &mut t,
            "defaultButtonCell",
            default_cell_name(&sheet.window, &sheet.cases),
        );
        out::put(
            &mut t,
            "backdrop_point_xy",
            out::floats(&[sheet.backdrop_point().x, sheet.backdrop_point().y]),
        );

        for (key, name) in APPEARANCES {
            let Some(appearance) = NSAppearance::appearanceNamed(&ns(name)) else {
                out::put(
                    &mut t,
                    key,
                    out::unavailable(format!("appearanceNamed({name}) is nil")),
                );
                continue;
            };
            sheet.window.setAppearance(Some(&appearance));
            sheet.content.setNeedsDisplay(true);
            self.settle(&sheet.window, SETTLE_STEPS);
            let run = if self.mode == Mode::Focus {
                self.focus_appearance(sheet, key)
            } else {
                self.appearance(sheet, key)
            };
            out::put_table(&mut t, key, run);
        }
        sheet.window.setAppearance(None);
        sheet.window.orderOut(None);
        sheet.window.close();
        if let Some(other) = other {
            other.orderOut(None);
            other.close();
        }
        t
    }

    /// Make `window` key and set the mode's condition, recorded in `t`; the
    /// second window of the not-key mode, which the caller closes.
    fn present(&self, window: &NSWindow, t: &mut Table) -> Option<Retained<NSWindow>> {
        if self.mode == Mode::Inactive && !self.app.isActive() {
            activate_ignoring(self.app);
            let steps = pump(self.app, ACTIVATE_WAIT_STEPS, || self.app.isActive());
            out::put(&mut *t, "reactivate_steps", out::int(i64::from(steps)));
            out::put(&mut *t, "reactivated", out::boolean(self.app.isActive()));
        }
        window.makeKeyAndOrderFront(None);
        out::put(
            &mut *t,
            "makeFirstResponder_nil",
            out::boolean(window.makeFirstResponder(None)),
        );
        let steps = pump(self.app, KEY_WAIT_STEPS, || {
            if !self.app.isActive() {
                activate_ignoring(self.app);
            }
            window.isKeyWindow()
        });
        out::put(&mut *t, "key_wait_steps", out::int(i64::from(steps)));
        out::put(
            &mut *t,
            "isKeyWindow_presented",
            out::boolean(window.isKeyWindow()),
        );
        let other = match self.mode {
            Mode::NotKey => not_key_window(self.mtm, self.app, window, t),
            Mode::Inactive => {
                out::put_table(t, "inactive", self.deactivate());
                None
            }
            Mode::Key | Mode::Focus | Mode::FocusTransition => None,
        };
        out::put(&mut *t, "isKeyWindow", out::boolean(window.isKeyWindow()));
        out::put(&mut *t, "isMainWindow", out::boolean(window.isMainWindow()));
        out::put(&mut *t, "NSApp_isActive", out::boolean(self.app.isActive()));
        other
    }

    /// Deactivate the application: `-[NSApplication deactivate]`, then, where
    /// it is still active, `open -b` of another application; each step's
    /// result and the frontmost application after.
    fn deactivate(&self) -> Table {
        let mut t = Table::new();
        self.app.deactivate();
        let steps = pump(self.app, ACTIVATE_WAIT_STEPS, || !self.app.isActive());
        out::put(&mut t, "deactivate_steps", out::int(i64::from(steps)));
        out::put(
            &mut t,
            "deactivate_isActive",
            out::boolean(self.app.isActive()),
        );
        if self.app.isActive() {
            let opened = std::process::Command::new("open")
                .arg("-b")
                .arg(OTHER_APPLICATION)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .map_err(|e| format!("cannot run open: {e}"))
                .and_then(wait_or_kill);
            out::put(
                &mut t,
                &format!("open_b_{}", OTHER_APPLICATION.replace('.', "_")),
                opened.map_or_else(out::unavailable, |()| out::text("exit 0")),
            );
            let steps = pump(self.app, ACTIVATE_WAIT_STEPS, || !self.app.isActive());
            out::put(&mut t, "open_steps", out::int(i64::from(steps)));
            out::put(&mut t, "open_isActive", out::boolean(self.app.isActive()));
        }
        let front = NSWorkspace::sharedWorkspace()
            .frontmostApplication()
            .and_then(|a| a.bundleIdentifier())
            .map_or_else(
                || out::unavailable("no frontmost application or bundle identifier"),
                |id| out::text(id.to_string()),
            );
        out::put(&mut t, "frontmostApplication", front);
        t
    }

    /// One appearance's captures of a sheet in the key, not-key or inactive
    /// mode.
    fn appearance(&self, sheet: &Sheet, key: &str) -> Table {
        let mut t = Table::new();
        let (dir, rel) = sheet.folder(self.out_dir, key);
        if let Err(e) = std::fs::create_dir_all(&dir) {
            out::put(
                &mut t,
                "error",
                out::unavailable(format!("cannot create {}: {e}", dir.display())),
            );
            return t;
        }
        let all: Vec<&Case> = sheet.cases.iter().collect();
        let pointer = sheet.pointer_states && self.mode == Mode::Key;
        let away = away_point(&sheet.window);

        let mut normal = Table::new();
        if pointer {
            out::put_table(&mut normal, "cursor", self.move_cursor(&sheet.window, away));
        }
        let normal_shots = self.shots(sheet, &mut normal, &dir, &rel, "normal", &all, false, None);
        out::put_table(&mut t, "normal", normal);
        let normal_rgba = normal_shots.window;

        let mut pressed = Table::new();
        for case in &sheet.cases {
            if let Some(b) = &case.button {
                b.highlight(true);
            }
        }
        self.settle(&sheet.window, SETTLE_STEPS);
        let highlighted = sheet
            .cases
            .iter()
            .filter(|c| {
                c.button
                    .as_ref()
                    .and_then(|b| b.cell())
                    .is_some_and(|cell| cell.isHighlighted())
            })
            .map(|c| out::text(c.name.clone()))
            .collect();
        out::put(&mut pressed, "cells_highlighted", out::array(highlighted));
        let pressed_shots = self.shots(
            sheet,
            &mut pressed,
            &dir,
            &rel,
            "pressed",
            &all,
            false,
            None,
        );
        diffs(
            &mut pressed,
            "diff_from_normal",
            normal_rgba.as_ref(),
            pressed_shots.window.as_ref(),
            &all,
            &sheet.window,
        );
        for case in &sheet.cases {
            if let Some(b) = &case.button {
                b.highlight(false);
            }
        }
        out::put_table(&mut t, "pressed", pressed);

        let mut disabled = Table::new();
        for case in &sheet.cases {
            case.control.setEnabled(false);
        }
        self.settle(&sheet.window, SETTLE_STEPS);
        let disabled_shots = self.shots(
            sheet,
            &mut disabled,
            &dir,
            &rel,
            "disabled",
            &all,
            false,
            None,
        );
        diffs(
            &mut disabled,
            "diff_from_normal",
            normal_rgba.as_ref(),
            disabled_shots.window.as_ref(),
            &all,
            &sheet.window,
        );
        for case in &sheet.cases {
            case.control.setEnabled(true);
        }
        self.settle(&sheet.window, SETTLE_STEPS);
        out::put_table(&mut t, "disabled", disabled);

        if pointer {
            self.pointer_states(
                &mut t,
                sheet,
                &dir,
                &rel,
                normal_rgba.as_ref(),
                pressed_shots.window.as_ref(),
                away,
            );
        }
        t
    }

    /// The hovered and clicked captures of the push sheet.
    #[allow(clippy::too_many_arguments)]
    fn pointer_states(
        &self,
        t: &mut Table,
        sheet: &Sheet,
        dir: &Path,
        rel: &str,
        normal: Option<&Rgba>,
        pressed: Option<&Rgba>,
        away: NSPoint,
    ) {
        let window = &sheet.window;
        let mut hovered = Table::new();
        for case in &sheet.cases {
            let mut h = Table::new();
            out::put_table(
                &mut h,
                "cursor",
                self.move_cursor(window, centre(window, case)),
            );
            let rep = self.shot(
                window,
                &mut h,
                dir,
                rel,
                &format!("hovered-{}.png", case.name),
            );
            diffs(
                &mut h,
                "diff_from_normal",
                normal,
                rep.as_ref(),
                &[case],
                window,
            );
            out::put_table(&mut hovered, &case.name, h);
        }
        out::put_table(
            &mut hovered,
            "cursor_parked",
            self.move_cursor(window, away),
        );
        out::put_table(t, "hovered", hovered);

        let mut clicked = Table::new();
        for case in sheet.cases.iter().filter(|c| c.clickable) {
            let mut c = Table::new();
            out::put_table(
                &mut c,
                "cursor",
                self.move_cursor(window, centre(window, case)),
            );
            let rep = self.click(&mut c, window, dir, rel, case);
            diffs(
                &mut c,
                "diff_from_normal",
                normal,
                rep.as_ref(),
                &[case],
                window,
            );
            diffs(
                &mut c,
                "diff_from_pressed",
                pressed,
                rep.as_ref(),
                &[case],
                window,
            );
            out::put_table(&mut clicked, &case.name, c);
        }
        out::put_table(
            &mut clicked,
            "cursor_parked",
            self.move_cursor(window, away),
        );
        out::put_table(t, "clicked", clicked);
    }

    /// One appearance's focused captures of a sheet: first the sheet with no
    /// button focused (unfocused), then each button made first responder in
    /// turn, each captured over every backdrop; a focused button's layer is
    /// compared with the unfocused one (diff_from_unfocused).
    fn focus_appearance(&self, sheet: &Sheet, key: &str) -> Table {
        let mut t = Table::new();
        let (dir, rel) = sheet.folder(self.out_dir, key);
        if let Err(e) = std::fs::create_dir_all(&dir) {
            out::put(
                &mut t,
                "error",
                out::unavailable(format!("cannot create {}: {e}", dir.display())),
            );
            return t;
        }
        let focusable: Vec<&Case> = sheet.cases.iter().filter(|c| c.focus).collect();
        let mut unfocused = Table::new();
        out::put(
            &mut unfocused,
            "makeFirstResponder_nil",
            out::boolean(sheet.window.makeFirstResponder(None)),
        );
        self.settle(&sheet.window, SETTLE_STEPS);
        let baseline = self
            .shots(
                sheet,
                &mut unfocused,
                &dir,
                &rel,
                "unfocused",
                &focusable,
                false,
                None,
            )
            .layer;
        out::put_table(&mut t, "unfocused", unfocused);
        let mut focused = Table::new();
        for case in sheet.cases.iter().filter(|c| c.focus) {
            let mut f = Table::new();
            let responder: &NSResponder = case.control.as_ref();
            out::put(
                &mut f,
                "acceptsFirstResponder",
                out::boolean(responder.acceptsFirstResponder()),
            );
            out::put(
                &mut f,
                "canBecomeKeyView",
                out::boolean(case.control.canBecomeKeyView()),
            );
            let made = sheet.window.makeFirstResponder(Some(responder));
            out::put(&mut f, "makeFirstResponder", out::boolean(made));
            let first = sheet
                .window
                .firstResponder()
                .is_some_and(|r| std::ptr::eq(&*r, responder));
            out::put(&mut f, "firstResponder_is_the_control", out::boolean(first));
            if made {
                self.settle(&sheet.window, SETTLE_STEPS);
                self.shots(
                    sheet,
                    &mut f,
                    &dir,
                    &rel,
                    &format!("focused-{}", case.name),
                    &[case],
                    true,
                    baseline.as_ref(),
                );
            } else {
                out::put(
                    &mut f,
                    "capture",
                    out::not_probed("makeFirstResponder: returned NO"),
                );
            }
            out::put_table(&mut focused, &case.name, f);
        }
        out::put(
            &mut focused,
            "makeFirstResponder_nil",
            out::boolean(sheet.window.makeFirstResponder(None)),
        );
        self.settle(&sheet.window, SETTLE_STEPS);
        out::put_table(&mut t, "focused", focused);
        t
    }

    /// Capture the sheet over every backdrop as `<stem>[-over-<backdrop>]`,
    /// the window's own into `t` and the others into `t.over_<backdrop>`;
    /// solve the layer from the black and white ones into `t.layer`, with
    /// the measures of `cases`, written whole or, with `crop`, as the first
    /// case's region. The backdrop is the window's own again after.
    #[allow(clippy::too_many_arguments)]
    fn shots(
        &self,
        sheet: &Sheet,
        t: &mut Table,
        dir: &Path,
        rel: &str,
        stem: &str,
        cases: &[&Case],
        crop: bool,
        baseline: Option<&Layer>,
    ) -> Shots {
        let window = &sheet.window;
        let frame = window.frame().size;
        let point = sheet.backdrop_point();
        let mut captured: Vec<(Backdrop, Option<Rgba>, Option<[u8; 4]>)> = Vec::new();
        for backdrop in Backdrop::ALL {
            sheet.show(backdrop);
            self.settle(window, SETTLE_STEPS);
            let file = format!("{stem}{}.png", backdrop.suffix());
            let mut sub = Table::new();
            let target = if backdrop == Backdrop::Window {
                &mut *t
            } else {
                &mut sub
            };
            if let Some(b) = backdrop.byte() {
                out::put(target, "requested", out::text(hex([b, b, b, 0xff])));
            }
            let rgba = self.shot(window, target, dir, rel, &file);
            let sampled = rgba.as_ref().and_then(|rgba| {
                let ratio = px_ratio(rgba.width, frame.width);
                let x = pixel_index(point.x, ratio, isize::try_from(rgba.width).ok()?)?;
                let y = pixel_index(point.y, ratio, isize::try_from(rgba.height).ok()?)?;
                rgba.pixel(usize::try_from(x).ok()?, usize::try_from(y).ok()?)
            });
            match sampled {
                Some(raw) => out::put(target, "backdrop_raw", out::text(hex(raw))),
                None => out::put(
                    target,
                    "backdrop_raw",
                    out::unavailable("no capture, or the point lies outside it"),
                ),
            }
            if backdrop != Backdrop::Window {
                out::put_table(t, backdrop.key(), sub);
            }
            captured.push((backdrop, rgba, sampled));
        }
        sheet.show(Backdrop::Window);

        let find = |b: Backdrop| {
            captured
                .iter()
                .find(|(k, _, _)| *k == b)
                .and_then(|(_, rgba, raw)| Some((rgba.as_ref()?, (*raw)?)))
        };
        let (table, layer) = match (find(Backdrop::Black), find(Backdrop::White)) {
            (Some(black), Some(white)) => self.layer(
                sheet,
                dir,
                rel,
                stem,
                cases,
                crop,
                [black, white],
                [find(Backdrop::Grey), find(Backdrop::Window)],
                baseline,
            ),
            _ => {
                let mut l = Table::new();
                out::put(
                    &mut l,
                    "error",
                    out::unavailable("the black or white capture or its backdrop is missing"),
                );
                (l, None)
            }
        };
        out::put_table(t, "layer", table);
        let window_rgba = captured
            .into_iter()
            .find(|(k, _, _)| *k == Backdrop::Window)
            .and_then(|(_, rgba, _)| rgba);
        Shots {
            window: window_rgba,
            layer,
        }
    }

    /// The layer solved from the black and white captures and their
    /// backdrops, checked against the grey and window ones, written as PNG
    /// and measured per case.
    #[allow(clippy::too_many_arguments)]
    fn layer(
        &self,
        sheet: &Sheet,
        dir: &Path,
        rel: &str,
        stem: &str,
        cases: &[&Case],
        crop: bool,
        solve_from: [(&Rgba, [u8; 4]); 2],
        checks: [Option<(&Rgba, [u8; 4])>; 2],
        baseline: Option<&Layer>,
    ) -> (Table, Option<Layer>) {
        let [black, white] = solve_from;
        let [grey, window] = checks;
        let (layer, mut l) = match Layer::solve(black.0, white.0, black.1, white.1) {
            Ok(solved) => solved,
            Err(reason) => {
                let mut l = Table::new();
                out::put(&mut l, "error", out::unavailable(reason));
                return (l, None);
            }
        };
        let frame = sheet.window.frame().size;
        let Some(ratio) = px_ratio(black.0.width, frame.width) else {
            out::put(&mut l, "error", out::unavailable("no pixel ratio"));
            return (l, Some(layer));
        };
        let regions: Vec<(&Case, Option<PxRect>, Option<PxRect>)> = cases
            .iter()
            .map(|case| {
                let r = from_top_left(&case.control, frame);
                (
                    *case,
                    PxRect::from_points(r, 0.0, ratio, black.0.width, black.0.height),
                    PxRect::from_points(r, DIFF_MARGIN, ratio, black.0.width, black.0.height),
                )
            })
            .collect();
        let area = if crop {
            regions.first().and_then(|(_, _, region)| *region)
        } else {
            Some(layer.bounds())
        };
        let Some(area) = area else {
            out::put(
                &mut l,
                "error",
                out::unavailable("the button lies outside the capture"),
            );
            return (l, Some(layer));
        };
        for (key, capture) in [("check_grey", grey), ("check_window", window)] {
            match capture {
                Some((rgba, raw)) => out::put_table(&mut l, key, layer.check(rgba, raw, area)),
                None => out::put(&mut l, key, out::unavailable("the capture is missing")),
            }
        }
        let file = format!("{stem}-layer.png");
        let written = layer.png(area).and_then(|png| {
            let path = dir.join(&file);
            std::fs::write(&path, png).map_err(|e| format!("cannot write {}: {e}", path.display()))
        });
        match written {
            Ok(()) => {
                out::put(&mut l, "file", out::text(format!("{rel}/{file}")));
                out::put(&mut l, "file_rect_px_xywh", area.value());
            }
            Err(reason) => out::put(&mut l, "file", out::unavailable(reason)),
        }
        let mut buttons = Table::new();
        for (case, frame_px, region_px) in regions {
            match (frame_px, region_px) {
                (Some(f), Some(r)) => {
                    let mut m = layer.measures(f, r, ratio);
                    for (key, capture) in [("check_grey", grey), ("check_window", window)] {
                        if let Some((rgba, raw)) = capture {
                            out::put_table(&mut m, key, layer.check(rgba, raw, r));
                        }
                    }
                    if let Some(base) = baseline {
                        out::put_table(&mut m, "diff_from_unfocused", layer.diff(base, r, ratio));
                    }
                    out::put_table(&mut buttons, &case.name, m);
                }
                _ => out::put(
                    &mut buttons,
                    &case.name,
                    out::unavailable("the button lies outside the capture"),
                ),
            }
        }
        out::put_table(&mut l, "buttons", buttons);
        (l, Some(layer))
    }

    /// Display the window and pump the run loop `steps` steps.
    fn settle(&self, window: &NSWindow, steps: u32) {
        window.displayIfNeeded();
        pump(self.app, steps, || false);
    }

    /// `p`, in AppKit's screen points, in Quartz's global display points.
    fn quartz(&self, p: NSPoint) -> NSPoint {
        NSPoint::new(p.x, self.primary_height - p.y)
    }

    /// Warp the cursor to `target` (AppKit's screen points), post a
    /// mouse-moved event there and pump the run loop `HOVER_STEPS` steps,
    /// counting the mouse events AppKit dispatches; where the cursor then is.
    fn move_cursor(&self, window: &NSWindow, target: NSPoint) -> Table {
        let mut t = Table::new();
        let at = self.quartz(target);
        out::put(
            &mut t,
            "target_screen_xy",
            out::floats(&[target.x, target.y]),
        );
        out::put(&mut t, "target_quartz_xy", out::floats(&[at.x, at.y]));
        let warp = CGWarpMouseCursorPosition(at);
        out::put(
            &mut t,
            "CGWarpMouseCursorPosition",
            out::int(i64::from(warp.0)),
        );
        let moved =
            CGEvent::new_mouse_event(None, CGEventType::MouseMoved, at, CGMouseButton::Left);
        out::put(&mut t, "mouseMoved_created", out::boolean(moved.is_some()));
        CGEvent::post(CGEventTapLocation::HIDEventTap, moved.as_deref());
        window.displayIfNeeded();
        out::put_table(&mut t, "events", pump_counting(self.app, HOVER_STEPS));
        let location = NSEvent::mouseLocation();
        out::put(
            &mut t,
            "mouseLocation_screen_xy",
            out::floats(&[location.x, location.y]),
        );
        let under =
            NSWindow::windowNumberAtPoint_belowWindowWithWindowNumber(location, 0, self.mtm);
        out::put(
            &mut t,
            "window_under_cursor_is_ours",
            out::boolean(under == window.windowNumber()),
        );
        t
    }

    /// Capture `window` to `<dir>/<file>`, reading its key status just
    /// before and after; the capture, decoded.
    fn shot(
        &self,
        window: &NSWindow,
        t: &mut Table,
        dir: &Path,
        rel: &str,
        file: &str,
    ) -> Option<Rgba> {
        let before = (window.isKeyWindow(), self.app.isActive());
        let path = dir.join(file);
        let captured = screencapture(window.windowNumber(), &path);
        let after = (window.isKeyWindow(), self.app.isActive());
        self.record_key(t, before, after);
        read_capture(t, captured, &path, &format!("{rel}/{file}"))
    }

    /// Record the window's key status and the application's activity read
    /// before and after a capture.
    fn record_key(&self, t: &mut Table, before: (bool, bool), after: (bool, bool)) {
        out::put(t, "isKeyWindow_before", out::boolean(before.0));
        out::put(t, "isKeyWindow_after", out::boolean(after.0));
        out::put(t, "NSApp_isActive_before", out::boolean(before.1));
        out::put(t, "NSApp_isActive_after", out::boolean(after.1));
        let key = before.0 && after.0;
        out::put(t, "key_at_capture", out::boolean(key));
        self.all_key.set(self.all_key.get() && key);
        self.any_key.set(self.any_key.get() || before.0 || after.0);
    }

    /// Press `case` with a real mouse-down and capture the window from a
    /// second thread while NSButtonCell tracks the mouse on this one; the
    /// second thread posts the mouse-up once the capture is done.
    fn click(
        &self,
        t: &mut Table,
        window: &NSWindow,
        dir: &Path,
        rel: &str,
        case: &Case,
    ) -> Option<Rgba> {
        let at = self.quartz(centre(window, case));
        let file = format!("clicked-{}.png", case.name);
        let path: PathBuf = dir.join(&file);
        let Some(down) =
            CGEvent::new_mouse_event(None, CGEventType::LeftMouseDown, at, CGMouseButton::Left)
        else {
            out::put(
                t,
                "capture",
                out::unavailable("the mouse-down CGEvent is NULL"),
            );
            return None;
        };
        let before = (window.isKeyWindow(), self.app.isActive());
        let number = window.windowNumber();
        let worker_path = path.clone();
        let worker = std::thread::spawn(move || {
            std::thread::sleep(CLICK_CAPTURE_DELAY);
            let captured = screencapture(number, &worker_path);
            let up =
                CGEvent::new_mouse_event(None, CGEventType::LeftMouseUp, at, CGMouseButton::Left);
            let created = up.is_some();
            CGEvent::post(CGEventTapLocation::HIDEventTap, up.as_deref());
            (captured, created)
        });
        CGEvent::post(CGEventTapLocation::HIDEventTap, Some(&*down));
        let steps = pump(self.app, CLICK_WAIT_STEPS, || worker.is_finished());
        out::put(t, "wait_steps", out::int(i64::from(steps)));
        let (captured, up_created) = worker
            .join()
            .unwrap_or_else(|_| (Err("the capture thread panicked".to_owned()), false));
        out::put(t, "mouseUp_created", out::boolean(up_created));
        // Deliver the mouse-up, so the tracking ends before the next state.
        self.settle(window, SETTLE_STEPS);
        let after = (window.isKeyWindow(), self.app.isActive());
        self.record_key(t, before, after);
        out::put(
            t,
            "key_status_read",
            out::text(
                "before the mouse-down and after the mouse-up: the main thread is in the button's mouse tracking while the capture is taken",
            ),
        );
        out::put(
            t,
            "highlighted_after",
            out::boolean(
                case.button
                    .as_ref()
                    .and_then(|b| b.cell())
                    .is_some_and(|cell| cell.isHighlighted()),
            ),
        );
        read_capture(t, captured, &path, &format!("{rel}/{file}"))
    }
}

/// Where the cursor is parked: right of the window's frame, in AppKit's
/// screen points.
fn away_point(window: &NSWindow) -> NSPoint {
    let f = window.frame();
    NSPoint::new(
        f.origin.x + f.size.width + CURSOR_AWAY_OFFSET,
        f.origin.y + f.size.height / 2.0,
    )
}

/// The centre of a button in AppKit's screen points.
fn centre(window: &NSWindow, case: &Case) -> NSPoint {
    let c = &case.control;
    let r = window.convertRectToScreen(c.convertRect_toView(c.bounds(), None));
    NSPoint::new(
        r.origin.x + r.size.width / 2.0,
        r.origin.y + r.size.height / 2.0,
    )
}

/// Pixels per point of a capture `pixels` wide of a window `points` wide.
fn px_ratio(pixels: usize, points: f64) -> Option<f64> {
    super::pixel_ratio(isize::try_from(pixels).ok()?, points)
}

/// Which button's cell the window holds as its default button cell.
fn default_cell_name(window: &NSWindow, cases: &[Case]) -> Value {
    let Some(cell) = window.defaultButtonCell() else {
        return out::text("nil");
    };
    let cell: &AnyObject = cell.as_ref();
    for case in cases {
        if let Some(own) = case.control.cell() {
            let own: &AnyObject = own.as_ref();
            if std::ptr::eq(own, cell) {
                return out::text(case.name.clone());
            }
        }
    }
    out::text("a cell of none of the pass's buttons")
}

/// A raw enum value as a TOML integer, or its decimal text where it does not
/// fit an `i64`.
fn raw<T: TryInto<i64> + Copy + std::fmt::Display>(v: T) -> Value {
    v.try_into()
        .map_or_else(|_| out::text(v.to_string()), out::int)
}

/// A button's setup and geometry; its frame in the window frame's points
/// from the top left, as the captures lay it out.
fn case_table(case: &Case, frame: NSSize) -> Table {
    let c = &case.control;
    let mut t = Table::new();
    out::put(&mut t, "variant", out::text(case.kind));
    out::put(&mut t, "setup", out::text(case.setup));
    out::put(
        &mut t,
        "class",
        out::text(c.class().name().to_string_lossy()),
    );
    out::put(&mut t, "controlSize_requested_raw", raw(case.size.0));
    out::put(&mut t, "controlSize_raw", raw(c.controlSize().0));
    out::put(&mut t, "frame_xywh", rect(c.frame()));
    out::put(
        &mut t,
        "intrinsicContentSize_wh",
        size(c.intrinsicContentSize()),
    );
    out::put(
        &mut t,
        "alignmentRectInsets_tlbr",
        insets(c.alignmentRectInsets()),
    );
    out::put(
        &mut t,
        "in_window_frame_from_top_left_xywh",
        rect(from_top_left(c, frame)),
    );
    match c.font() {
        Some(font) => out::put_table(&mut t, "font", font_table(&font)),
        None => out::put(&mut t, "font", out::unavailable("nil")),
    }
    if let Some(b) = &case.button {
        out::put(
            &mut t,
            "keyEquivalent",
            out::text(b.keyEquivalent().to_string()),
        );
        out::put(&mut t, "state", out::int_isize(b.state()));
        out::put(&mut t, "bezelStyle_raw", raw(b.bezelStyle().0));
        out::put(&mut t, "isBordered", out::boolean(b.isBordered()));
        out::put(
            &mut t,
            "hasDestructiveAction",
            out::boolean(b.hasDestructiveAction()),
        );
        let class = NSButton::class();
        if class.responds_to(sel!(tintProminence)) {
            out::put(&mut t, "tintProminence_raw", raw(b.tintProminence().0));
        }
        if class.responds_to(sel!(borderShape)) {
            out::put(&mut t, "borderShape_raw", raw(b.borderShape().0));
        }
        out::put(&mut t, "imagePosition_raw", raw(b.imagePosition().0));
        if let Some(image) = b.image() {
            out::put(&mut t, "image_size_wh", size(image.size()));
        }
        if let Some(cell) = b.cell() {
            let bounds = b.bounds();
            out::put(
                &mut t,
                "cell_titleRectForBounds_xywh",
                rect(cell.titleRectForBounds(bounds)),
            );
            out::put(
                &mut t,
                "cell_imageRectForBounds_xywh",
                rect(cell.imageRectForBounds(bounds)),
            );
            out::put(
                &mut t,
                "cell_drawingRectForBounds_xywh",
                rect(cell.drawingRectForBounds(bounds)),
            );
            out::put(&mut t, "cell_cellSize_wh", size(cell.cellSize()));
        }
    }
    t
}

/// The view's frame in the window frame's points from its top left.
fn from_top_left(view: &NSView, frame: NSSize) -> NSRect {
    let r = view.convertRect_toView(view.bounds(), None);
    NSRect::new(
        NSPoint::new(r.origin.x, frame.height - r.origin.y - r.size.height),
        r.size,
    )
}

/// Compare `rep` with `base` inside each of `cases`' frames grown by
/// `DIFF_MARGIN`, into `t[name]`.
fn diffs(
    t: &mut Table,
    name: &str,
    base: Option<&Rgba>,
    rep: Option<&Rgba>,
    cases: &[&Case],
    window: &NSWindow,
) {
    let (Some(base), Some(rep)) = (base, rep) else {
        out::put(t, name, out::unavailable("a capture is missing"));
        return;
    };
    let frame = window.frame().size;
    let mut d = Table::new();
    for case in cases {
        let r = from_top_left(&case.control, frame);
        out::put(&mut d, &case.name, diff_table(base, rep, r, frame));
    }
    out::put_table(t, name, d);
}

/// Record a capture's file and size, and decode it.
fn read_capture(
    t: &mut Table,
    captured: Result<(), String>,
    path: &Path,
    file: &str,
) -> Option<Rgba> {
    if let Err(reason) = captured {
        out::put(t, "capture", out::unavailable(reason));
        return None;
    }
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(e) => {
            out::put(
                t,
                "capture",
                out::unavailable(format!("cannot read {}: {e}", path.display())),
            );
            return None;
        }
    };
    out::put(t, "file", out::text(file));
    let Some(rep) = NSBitmapImageRep::imageRepWithData(&NSData::with_bytes(&bytes)) else {
        out::put(
            t,
            "capture",
            out::unavailable("the capture is not a bitmap image"),
        );
        return None;
    };
    out::put(
        t,
        "pixels_wh",
        out::array(vec![
            out::int_isize(rep.pixelsWide()),
            out::int_isize(rep.pixelsHigh()),
        ]),
    );
    match Rgba::from_rep(&rep) {
        Ok(rgba) => Some(rgba),
        Err(reason) => {
            out::put(t, "decode", out::unavailable(reason));
            None
        }
    }
}

/// The pixels of `frame` (window points from the top left, grown by
/// `DIFF_MARGIN`) whose bytes differ between `a` and `b`.
fn diff_table(a: &Rgba, b: &Rgba, frame: NSRect, window: NSSize) -> Value {
    if (a.width, a.height) != (b.width, b.height) {
        return out::unavailable("the captures differ in size");
    }
    let Some(ratio) = px_ratio(b.width, window.width) else {
        return out::unavailable("no pixel ratio");
    };
    let Some(r) = PxRect::from_points(frame, DIFF_MARGIN, ratio, b.width, b.height) else {
        return out::unavailable("the frame lies outside the capture");
    };
    let mut differing: i64 = 0;
    let mut max_delta: u8 = 0;
    for y in r.y0..r.y1 {
        for x in r.x0..r.x1 {
            let (Some(pa), Some(pb)) = (a.pixel(x, y), b.pixel(x, y)) else {
                return out::unavailable(format!("({x}, {y}) lies outside the capture"));
            };
            let delta = pa
                .iter()
                .zip(pb.iter())
                .fold(0_u8, |m, (p, q)| m.max(p.abs_diff(*q)));
            if delta > 0 {
                differing = differing.saturating_add(1);
                max_delta = max_delta.max(delta);
            }
        }
    }
    let mut t = Table::new();
    out::put(&mut t, "rect_px_xywh", r.value());
    out::put(&mut t, "pixels", raw(r.width().saturating_mul(r.height())));
    out::put(&mut t, "differing_pixels", out::int(differing));
    out::put(&mut t, "max_byte_delta", out::int(i64::from(max_delta)));
    Value::Table(t)
}

/// Pump the run loop `steps` steps of `PUMP_STEP`, as `pump` does, counting
/// the mouse events it dispatches.
fn pump_counting(app: &NSApplication, steps: u32) -> Table {
    let mode = ffi::default_run_loop_mode();
    let counted = [
        ("mouseMoved", NSEventType::MouseMoved),
        ("mouseEntered", NSEventType::MouseEntered),
        ("mouseExited", NSEventType::MouseExited),
        ("cursorUpdate", NSEventType::CursorUpdate),
    ];
    let mut counts = [0_i64; 4];
    let mut other: i64 = 0;
    for _ in 0..steps {
        let until = NSDate::dateWithTimeIntervalSinceNow(PUMP_STEP.as_secs_f64());
        while let Some(event) = app.nextEventMatchingMask_untilDate_inMode_dequeue(
            NSEventMask::Any,
            Some(&until),
            mode,
            true,
        ) {
            let kind = event.r#type();
            match counted
                .iter()
                .position(|(_, k)| *k == kind)
                .and_then(|i| counts.get_mut(i))
            {
                Some(n) => *n = n.saturating_add(1),
                None => other = other.saturating_add(1),
            }
            app.sendEvent(&event);
        }
        app.updateWindows();
    }
    let mut t = Table::new();
    for ((name, _), n) in counted.iter().zip(counts) {
        out::put(&mut t, name, out::int(n));
    }
    out::put(&mut t, "other", out::int(other));
    t
}
