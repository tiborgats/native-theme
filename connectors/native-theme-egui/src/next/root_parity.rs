//! Connector parity of the crate root's functions (docs/v0.8.0-plan.md,
//! section 1, rule 2): each function of [`super`] gives on the 0.8.0 model
//! what its twin at the crate root gives on the 0.7.2 model, on every
//! preset of [`PRESETS`], light and dark.
//!
//! The two sides are compared by their `Debug` forms (egui's `Color32`,
//! `FontId` and the durations have `PartialEq` and `Debug`; `Debug` lets one
//! table hold them all), a value of the 0.8.0 side that may be unstated as
//! `Ok` of the 0.7.2 side's value, and a 0.7.2 type the 0.8.0 model names
//! anew (`DialogButtonOrder`) by its variant's name. The 0.7.2 side is called
//! with the default preferences, whose text scale is 1, as the 0.8.0 theme is
//! resolved for. A difference is allowed only as an entry of
//! [`DIFFERENCES`], with its reason; an entry that differs nowhere is stale
//! and fails.

use std::fmt::Debug;

use native_theme::next::{Preset, ThemeContext};

use super::{Mapped, Unmapped};
use crate::{AccessibilityPreferences, ResolvedTheme, Role, TextRole};

/// The presets compared, each by its 0.7.2 key and its 0.8.0 preset.
pub(crate) const PRESETS: &[(&str, Preset)] = &[
    ("kde-breeze", Preset::KdeBreeze),
    ("adwaita", Preset::Adwaita),
    ("windows-11", Preset::Windows11),
    ("macos-monterey", Preset::MacosMonterey),
    ("macos-tahoe", Preset::MacosTahoe),
    ("material-3", Preset::Material3),
    ("ios-13", Preset::Ios13),
];

/// A listed difference: the function, the preset's key, the 0.8.0 side's
/// value it asserts there (its `Debug` form), and why.
struct Difference {
    name: &'static str,
    preset: &'static str,
    new: &'static str,
    reason: &'static str,
}

const DIFFERENCES: &[Difference] = &[
    Difference {
        name: "toolbar_button_checked_background",
        preset: "macos-monterey",
        new: "Ok(Some(#DF_DF_DF_FF))",
        reason: TOOLBAR_CHECKED,
    },
    Difference {
        name: "toolbar_button_checked_background",
        preset: "macos-monterey",
        new: "Ok(Some(#41_41_41_FF))",
        reason: TOOLBAR_CHECKED,
    },
    Difference {
        name: "toolbar_button_checked_background",
        preset: "macos-tahoe",
        new: "Ok(Some(#00_7A_FF_FF))",
        reason: TOOLBAR_CHECKED,
    },
    Difference {
        name: "toolbar_button_checked_border_color",
        preset: "windows-11",
        new: "Ok(Some(#14_14_14_14))",
        reason: "0.7.2's windows-11 states no checked toolbar button outline; the 0.8.0 preset \
                 states the three sides of WinUI's AccentControlElevationBorderBrush, \
                 ControlStrokeColorOnAccentDefault #14FFFFFF in both schemes, \
                 toolbar.button(CHECKED).stroke (AppBarToggleButton_themeresources.xaml:160, :30; \
                 Common_themeresources_any.xaml:245, :41 at microsoft-ui-xaml 8463f45; \
                 native-theme/tests/next_homes.toml, row toolbar.button_checked_border_color, a \
                 listed difference), here premultiplied",
    },
];

/// The macOS checked toolbar button's fill, which 0.7.2 does not state.
const TOOLBAR_CHECKED: &str = "0.7.2's macOS toolbar states no checked button fill; the 0.8.0 \
     macOS presets state the real toolbar's toolbar-bezel button as an on \
     NSButtonTypePushOnPushOff toggle, toolbar.button(CHECKED).fill: macos-monterey #dfdfdf / \
     #414141, macos-tahoe the accent, #007aff (os-probe run 45634379188, macOS 15 and 26, \
     toolbar states sheet, `*.normal.toolbar_toggle_on` `fill`; \
     native-theme/tests/next_homes.toml, row toolbar.button_checked_background, a listed \
     difference)";

/// A field's frame the 0.8.0 preset outlines in a border of its own: the
/// function, the preset's key, the field's part and its homes, and why. The
/// 0.8.0 frame is compared with the 0.7.2 frame whose stroke takes the
/// width at `width_home` and, unfocused, the colour at `color_home`, as the
/// 0.8.0 function reads them ([`super::line_width`], [`super::line_color`]);
/// focused, its colour stays the focused one, which both models state alike.
struct FrameDifference {
    name: &'static str,
    preset: &'static str,
    field: fn(&native_theme::next::ResolvedTheme) -> native_theme::next::PartStyle,
    width_home: &'static str,
    color_home: &'static str,
    reason: &'static str,
}

const FRAME_DIFFERENCES: &[FrameDifference] = &[
    FrameDifference {
        name: "text_area_frame",
        preset: "ios-13",
        field: |r| r.text_area.field(native_theme::next::WidgetState::NONE),
        width_home: "text_area.field(NONE).stroke.width",
        color_home: "text_area.field(NONE).stroke.color",
        reason: "0.7.2's ios-13 outlines the text area in the default line (0.5 px of the \
                 separator, uncited); the 0.8.0 preset states WebKit's `border: 1px solid \
                 -apple-system-secondary-label` (html.css:457-466 at WebKit 31adf9d; \
                 native-theme/tests/next_homes.toml, rows text_area.border.color and \
                 text_area.border.line_width, listed differences)",
    },
    FrameDifference {
        name: "text_area_frame",
        preset: "windows-11",
        field: |r| r.text_area.field(native_theme::next::WidgetState::NONE),
        width_home: "text_area.field(NONE).stroke.width",
        color_home: "text_area.field(NONE).stroke.color",
        reason: "0.7.2's windows-11 outlines the text area in the default border, \
                 CardStrokeColorDefault (#19000000 in dark); the 0.8.0 preset states the three \
                 sides of WinUI's TextControlElevationBorderBrush, ControlStrokeColorDefault \
                 #0F000000 / #12FFFFFF (TextBox_themeresources.xaml:134, :27; \
                 Common_themeresources_any.xaml:243, :39 at microsoft-ui-xaml 8463f45; \
                 native-theme/tests/next_homes.toml, row text_area.border.color, a listed \
                 difference): the same in light",
    },
];

/// `old`, the 0.7.2 frame, with its stroke as [`FrameDifference`] `d`
/// states it on `new`.
fn frame_as_stated(
    d: &FrameDifference,
    old: Mapped<egui::Frame>,
    focused: bool,
    new: &native_theme::next::ResolvedTheme,
) -> Mapped<egui::Frame> {
    let mut frame = old?;
    let field = (d.field)(new);
    if let Some(width) = super::line_width(&field, d.width_home)? {
        frame.stroke.width = width;
    }
    if !focused {
        frame.stroke.color = super::line_color(&field, d.color_home)?;
    }
    Ok(frame)
}

/// Both models of `key`, light or dark: the 0.7.2 theme resolved as the
/// 0.7.2 presets are (the Apple presets at 72 font DPI, the others at 96),
/// and the 0.8.0 preset resolved for the scheme.
pub(crate) fn resolve(
    key: &str,
    preset: Preset,
    is_dark: bool,
) -> crate::Result<(ResolvedTheme, native_theme::next::ResolvedTheme)> {
    let (mode, ctx) = if is_dark {
        (native_theme::theme::ColorMode::Dark, ThemeContext::dark())
    } else {
        (native_theme::theme::ColorMode::Light, ThemeContext::light())
    };
    let apple = matches!(
        preset,
        Preset::MacosMonterey | Preset::MacosTahoe | Preset::Ios13
    );
    let context = native_theme::ResolutionContext {
        font_dpi: if apple { 72.0 } else { 96.0 },
        ..native_theme::ResolutionContext::for_tests()
    };
    let old = native_theme::theme::Theme::preset(key)?
        .into_variant(mode)?
        .into_resolved(&context)?;
    let new = native_theme::next::Theme::preset(preset).resolve(&ctx);
    Ok((old, new))
}

/// The pairs compared on one preset and scheme.
struct Pairs {
    label: String,
    preset: &'static str,
    failures: Vec<String>,
    seen: Vec<(&'static str, &'static str)>,
}

impl Pairs {
    fn pair(&mut self, name: &'static str, old: impl Debug, new: impl Debug) {
        let (old, new) = (format!("{old:?}"), format!("{new:?}"));
        if old == new {
            return;
        }
        if DIFFERENCES.iter().any(|d| {
            d.name == name && d.preset == self.preset && d.new == new && !d.reason.is_empty()
        }) {
            self.seen.push((name, self.preset));
            return;
        }
        self.failures
            .push(format!("{}: {name}: 0.7.2 {old}, 0.8.0 {new}", self.label));
    }
}

/// `v` as the 0.8.0 side gives a value it may find unstated.
fn ok<T>(v: T) -> Mapped<T> {
    Ok::<T, Unmapped>(v)
}

/// The colours, the focus ring and the widgets' values no `Style` holds.
fn values(p: &mut Pairs, old: &ResolvedTheme, new: &native_theme::next::ResolvedTheme) {
    p.pair("info_color", crate::info_color(old), super::info_color(new));
    p.pair(
        "info_text_color",
        crate::info_text_color(old),
        super::info_text_color(new),
    );
    p.pair(
        "warning_text_color",
        crate::warning_text_color(old),
        super::warning_text_color(new),
    );
    p.pair(
        "disabled_text_color",
        crate::disabled_text_color(old),
        super::disabled_text_color(new),
    );
    p.pair(
        "selection_inactive_background",
        ok(crate::selection_inactive_background(old)),
        super::selection_inactive_background(new),
    );
    p.pair(
        "selection_inactive_text_color",
        ok(crate::selection_inactive_text_color(old)),
        super::selection_inactive_text_color(new),
    );
    p.pair(
        "toolbar_button_checked_background",
        ok(crate::toolbar_button_checked_background(old)),
        super::toolbar_button_checked_background(new),
    );
    p.pair(
        "toolbar_button_checked_border_color",
        ok(crate::toolbar_button_checked_border_color(old)),
        super::toolbar_button_checked_border_color(new),
    );
    p.pair(
        "focus_ring_color",
        crate::focus_ring_color(old),
        super::focus_ring_color(new),
    );
    p.pair(
        "focus_ring_width",
        ok(crate::focus_ring_width(old)),
        super::focus_ring_width(new),
    );
    p.pair(
        "focus_ring_offset",
        crate::focus_ring_offset(old),
        super::focus_ring_offset(new),
    );
    p.pair(
        "dialog_button_order",
        ok(format!("{:?}", crate::dialog_button_order(old))),
        super::dialog_button_order(new).map(|o| format!("{o:?}")),
    );
    p.pair(
        "list_header_height",
        crate::list_header_height(old),
        super::list_header_height(new),
    );
    p.pair(
        "list_header_hover_background",
        ok(crate::list_header_hover_background(old)),
        super::list_header_hover_background(new),
    );
    p.pair(
        "list_header_active_background",
        ok(crate::list_header_active_background(old)),
        super::list_header_active_background(new),
    );
    p.pair(
        "tooltip_show_delay",
        crate::tooltip_show_delay(old),
        super::tooltip_show_delay(new),
    );
    p.pair(
        "scrollbar_groove_width",
        ok(crate::scrollbar_groove_width(old)),
        super::scrollbar_groove_width(new),
    );
    p.pair(
        "border_corner_radius",
        crate::border_corner_radius(old),
        super::border_corner_radius(new),
    );
    p.pair(
        "border_corner_radius_lg",
        crate::border_corner_radius_lg(old),
        super::border_corner_radius_lg(new),
    );
    p.pair(
        "border_color",
        ok(crate::border_color(old)),
        super::border_color(new),
    );
    p.pair(
        "disabled_opacity",
        crate::disabled_opacity(old),
        super::disabled_opacity(new),
    );
}

/// The fonts and the text roles.
fn fonts(p: &mut Pairs, old: &ResolvedTheme, new: &native_theme::next::ResolvedTheme) {
    let prefs = AccessibilityPreferences::default();
    for role in [
        TextRole::Caption,
        TextRole::SectionHeading,
        TextRole::DialogTitle,
        TextRole::Display,
    ] {
        p.pair(
            "text_role_font",
            (role, crate::text_role_font(old, role, &prefs)),
            (role, super::text_role_font(new, role)),
        );
        p.pair(
            "text_role_line_height",
            (role, crate::text_role_line_height(old, role, &prefs)),
            (role, super::text_role_line_height(new, role)),
        );
        p.pair(
            "text_role_weight",
            (role, crate::text_role_weight(old, role)),
            (role, super::text_role_weight(new, role)),
        );
    }
    for &role in Role::all() {
        p.pair(
            "role_font_weight",
            (role, ok(crate::role_font_weight(old, role))),
            (role, super::role_font_weight(new, role)),
        );
        p.pair(
            "role_font_is_italic",
            (role, crate::role_font_is_italic(old, role)),
            (role, super::role_font_is_italic(new, role)),
        );
    }
    p.pair(
        "line_height",
        crate::line_height(old),
        super::line_height(new),
    );
    p.pair(
        "font_weight",
        ok(crate::font_weight(old)),
        super::font_weight(new),
    );
    p.pair(
        "mono_font_weight",
        ok(crate::mono_font_weight(old)),
        super::mono_font_weight(new),
    );
    p.pair(
        "window_title_bar_font",
        ok(crate::window_title_bar_font(old, &prefs)),
        super::window_title_bar_font(new),
    );
    p.pair(
        "window_title_bar_text_color",
        ok(crate::window_title_bar_text_color(old, true)),
        super::window_title_bar_text_color(new),
    );
    p.pair(
        "list_header_font",
        ok(crate::list_header_font(old, &prefs)),
        super::list_header_font(new),
    );
    p.pair(
        "font_family",
        crate::font_family(old),
        super::font_family(new),
    );
    p.pair(
        "font_size",
        crate::font_size(old, &prefs),
        super::font_size(new),
    );
    p.pair(
        "mono_font_family",
        crate::mono_font_family(old),
        super::mono_font_family(new),
    );
    p.pair(
        "mono_font_size",
        crate::mono_font_size(old, &prefs),
        super::mono_font_size(new),
    );
}

/// The widgets' parts the application draws: the default button, the field
/// margins, the field frames at rest and focused (built in one pass of a
/// `Ui` in egui's own style), the colour the expander's arrow closure paints
/// in, and the icon sizes.
fn parts(p: &mut Pairs, old: &ResolvedTheme, new: &native_theme::next::ResolvedTheme) {
    use crate::icons::IconContext;

    p.pair(
        "default_button",
        ok(crate::default_button(old)),
        super::default_button(new),
    );
    p.pair(
        "input_margin",
        ok(crate::input_margin(old)),
        super::input_margin(new),
    );
    p.pair(
        "text_area_margin",
        ok(crate::text_area_margin(old)),
        super::text_area_margin(new),
    );
    p.pair(
        "expander_icon",
        crate::accessors::expander_arrow(old),
        super::expander_arrow(new),
    );
    for context in [
        IconContext::Small,
        IconContext::Toolbar,
        IconContext::Panel,
        IconContext::Dialog,
        IconContext::Large,
    ] {
        p.pair(
            "icons::icon_size",
            (context, ok(crate::icons::icon_size(old, context))),
            (context, super::icon_size(new, context)),
        );
    }
    let ctx = egui::Context::default();
    for focused in [false, true] {
        let id = egui::Id::unique(("root-parity-field", focused));
        if focused {
            ctx.memory_mut(|m| m.request_focus(id));
        }
        let mut frames = Vec::new();
        let _ = crate::install_tests::pass(&ctx, egui::RawInput::default(), |ui| {
            frames.push((
                "input_frame",
                ok(crate::input_frame(ui, id, old)),
                super::input_frame(ui, id, new),
            ));
            frames.push((
                "text_area_frame",
                ok(crate::text_area_frame(ui, id, old)),
                super::text_area_frame(ui, id, new).map(|(frame, _)| frame),
            ));
        });
        for (name, o, n) in frames {
            let o = match FRAME_DIFFERENCES
                .iter()
                .find(|d| d.name == name && d.preset == p.preset)
            {
                Some(d) => {
                    let expected = frame_as_stated(d, o, focused, new);
                    if format!("{expected:?}") != format!("{o:?}") {
                        p.seen.push((name, p.preset));
                    }
                    expected
                }
                None => o,
            };
            p.pair(name, (focused, o), (focused, n));
        }
    }
}

/// What a `FontDefinitions` a plan made holds, in a form `Debug` compares: each family's
/// chain, each face's key, index in its file, tweak and byte count, and the notes.
fn defs_form((defs, notes): &(egui::FontDefinitions, Vec<crate::Note>)) -> String {
    let faces: Vec<_> = defs
        .font_data
        .iter()
        .map(|(key, data)| (key, data.index, &data.tweak, data.bytes().len()))
        .collect();
    format!("{:?} {faces:?} {notes:?}", defs.families)
}

/// The fonts a plan installs: [`super::font_definitions`] beside
/// [`crate::fonts::font_definitions`], for a plan holding egui's `Hack` under the body text's
/// and the monospace text's family, weight and style as the 0.7.2 theme states them, and,
/// with feature `system-fonts`, for each model's plan of the OS's own faces.
fn font_definitions(p: &mut Pairs, old: &ResolvedTheme, new: &native_theme::next::ResolvedTheme) {
    use crate::fonts::{FontBytes, FontPlan};

    let hack = epaint_default_fonts::HACK_REGULAR;
    let plan = [&old.defaults.font, &old.defaults.mono_font]
        .into_iter()
        .fold(FontPlan::new(), |plan, spec| {
            plan.face(
                &spec.family,
                spec.weight,
                spec.style,
                FontBytes::Static(hack),
            )
        });
    p.pair(
        "fonts::font_definitions",
        ok(defs_form(&crate::fonts::font_definitions(old, &plan))),
        super::font_definitions(new, &plan).map(|d| defs_form(&d)),
    );
    #[cfg(feature = "system-fonts")]
    p.pair(
        "fonts::FontPlan::from_system",
        ok(defs_form(&crate::fonts::font_definitions(
            old,
            &FontPlan::from_system(old),
        ))),
        super::font_plan_from_system(new)
            .and_then(|plan| super::font_definitions(new, &plan))
            .map(|d| defs_form(&d)),
    );
}

/// The inactive window's title colour, from each model's inactive theme.
fn inactive_title(p: &mut Pairs, old: &ResolvedTheme, preset: Preset, is_dark: bool) {
    let mut ctx = if is_dark {
        ThemeContext::dark()
    } else {
        ThemeContext::light()
    };
    ctx.window_active = false;
    let inactive = native_theme::next::Theme::preset(preset).resolve(&ctx);
    p.pair(
        "window_title_bar_text_color(inactive)",
        ok(crate::window_title_bar_text_color(old, false)),
        super::window_title_bar_text_color(&inactive),
    );
}

#[test]
fn the_crate_roots_functions_agree_on_both_models() -> crate::Result<()> {
    let mut failures = Vec::new();
    let mut seen = Vec::new();
    for &(key, preset) in PRESETS {
        for is_dark in [false, true] {
            let (old, new) = resolve(key, preset, is_dark)?;
            let mut p = Pairs {
                label: format!("{key}/{}", if is_dark { "dark" } else { "light" }),
                preset: key,
                failures: Vec::new(),
                seen: Vec::new(),
            };
            values(&mut p, &old, &new);
            fonts(&mut p, &old, &new);
            font_definitions(&mut p, &old, &new);
            parts(&mut p, &old, &new);
            inactive_title(&mut p, &old, preset, is_dark);
            failures.append(&mut p.failures);
            seen.append(&mut p.seen);
        }
    }
    for d in DIFFERENCES {
        if !seen.contains(&(d.name, d.preset)) {
            failures.push(format!(
                "{} on {}: listed, but differs nowhere: remove the entry",
                d.name, d.preset
            ));
        }
    }
    for d in FRAME_DIFFERENCES {
        if d.reason.is_empty() || !seen.contains(&(d.name, d.preset)) {
            failures.push(format!(
                "{} on {}: listed, but differs nowhere: remove the entry",
                d.name, d.preset
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} differences:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}
