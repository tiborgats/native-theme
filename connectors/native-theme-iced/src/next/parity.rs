//! Connector parity of the 0.8.0 work (docs/v0.8.0-plan.md, section 1, rule
//! 2; M4, step 1): what the connector hands to iced, computed from the 0.7.2
//! model by the connector's public mapping and from the 0.8.0 model by
//! [`super`], is equal: the iced theme and the style each ported style
//! function returns, in every status of its widget.
//!
//! Presets covered: kde-breeze, adwaita, windows-11, macos-monterey,
//! macos-tahoe, material-3 and ios-13, light and dark ([`PRESETS`]). The
//! 0.7.2 side is resolved as the connector's tests resolve it
//! (`ResolutionContext::for_tests`), with no accessibility scaling; the
//! 0.8.0 side for the scheme's default context, whose text scale is 1.
//!
//! The form of comparison per iced type the connector produces:
//!
//! - `iced_core::Theme` (with its `Palette` and `palette::Extended`) derives
//!   `PartialEq`: the palette, the extended palette and the name are each
//!   compared by it, the name being the theme's own on either side.
//! - Each widget's `Style` that a ported style function returns derives
//!   `PartialEq` and is compared by it, the function called with its side's
//!   own iced theme in every `Status` of the widget (the style functions
//!   read iced's own fields from the theme in effect).
//!
//! A difference is allowed only as an entry of [`DIFFERENCES`], with its
//! reason: a style function whose 0.8.0 stroke lies inside or across the
//! widget's edge, where 0.7.2 stated no alignment; one whose hovered fill or
//! stroke the 0.8.0 theme states nothing for, which is iced's own (spec §2,
//! `from_state`). The entry asserts the value the 0.8.0 side must give: the
//! 0.7.2 side's own output with each of its strokes laid where the entry
//! says ([`crate::styles::Line::aligned`]), so the stroke is composited over
//! the fill each status paints, or over it at half its alpha, and with the
//! hover options the entry names unset ([`Listed`]); any other value fails. Each (style function, preset)
//! pair an entry lists must differ in some status, or the entry is stale and
//! fails.

use std::fmt::Debug;

use iced_core::theme::Theme;
use native_theme::next::{Preset, ThemeContext};

use super::LineAlign;
use crate::{ColorMode, ResolvedTheme};

/// The presets the test covers: the 0.7.2 key and the 0.8.0 preset.
pub(crate) const PRESETS: &[(&str, Preset)] = &[
    ("kde-breeze", Preset::KdeBreeze),
    ("adwaita", Preset::Adwaita),
    ("windows-11", Preset::Windows11),
    ("macos-monterey", Preset::MacosMonterey),
    ("macos-tahoe", Preset::MacosTahoe),
    ("material-3", Preset::Material3),
    ("ios-13", Preset::Ios13),
];

/// A listed difference (plan, section 1, rule 2): the style function, the
/// presets on which it differs, how the 0.8.0 side differs, and why.
struct Difference {
    leaf: &'static str,
    presets: &'static [&'static str],
    listed: Listed,
    reason: &'static [&'static str],
}

/// How a listed style function's 0.8.0 side differs from the 0.7.2 side:
/// where its strokes lie, and which of its hover options the 0.8.0 theme
/// leaves to iced. The expected value is the 0.7.2 side's output with its
/// strokes laid there and those options unset.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Listed {
    pub align: LineAlign,
    pub hover: Hover,
}

/// The hover options of a style function's source that the 0.8.0 theme
/// states nothing for (spec §2, `from_state`), each then iced's own for the
/// status: the hovered fill, the hovered stroke.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Hover {
    pub fill: bool,
    pub stroke: bool,
}

impl Hover {
    /// No hover option left to iced.
    const STATED: Self = Self {
        fill: false,
        stroke: false,
    };
}

/// The reason of a translucent stroke the theme lays inside the box.
const INSIDE: &str = "the part's border is a translucent stroke the theme lays inside the \
     box, over the fill (StrokeAlign::Inside, the alignment Stroke::uniform gives); iced on \
     wgpu ends the fill at the border's inner edge, so the connector hands iced the stroke \
     composited over the fill the status paints, where 0.7.2 handed it the stated colour, \
     which showed the window through the band (docs/v0.8.0-spec.md §2, StrokeAlign; the M2 \
     fill/stroke decision, connector table, iced row)";

/// The reason of a translucent stroke centred on the box's edge.
const CENTER: &str = "the part's border is a translucent pen centred on the edge \
     (StrokeAlign::Center); iced on wgpu ends the fill at the border's inner edge, so the \
     connector hands iced the band's mean, the stroke over the fill at half its alpha, where \
     0.7.2 handed it the stated colour (docs/v0.8.0-spec.md §2, StrokeAlign; the M2 \
     fill/stroke decision, connector table, iced row)";

/// The macOS presets, whose control strokes are translucent and inside the
/// box.
const MACOS: &[&str] = &["macos-monterey", "macos-tahoe"];

/// The macOS presets and windows-11, whose surface strokes are translucent
/// and inside the box (windows-11's push button and text field strokes lie
/// outside it, `StrokeAlign::Outside`, and do not differ).
const MACOS_WINDOWS: &[&str] = &["windows-11", "macos-monterey", "macos-tahoe"];

/// The reason of a hovered status the 0.8.0 theme states nothing for.
const HOVER: &str = "the theme states no value for the hovered widget: 0.7.2 leaves the \
     soft option unset (docs/inheritance-rules.toml, [no_inheritance]: no rule fills it, \
     None keeps the look at rest), which its connector drew as the widget at rest; the \
     0.8.0 theme has no rule for HOVER (the from_state bit clear when queried with HOVER \
     alone), and the connector keeps iced's own style for that status (docs/v0.8.0-spec.md \
     §2, from_state; §10, rule 0)";

/// A style function whose stroke lies inside the box on `presets`.
const fn inside(leaf: &'static str, presets: &'static [&'static str]) -> Difference {
    Difference {
        leaf,
        presets,
        listed: Listed {
            align: LineAlign::Inside,
            hover: Hover::STATED,
        },
        reason: &[INSIDE],
    }
}

/// A style function whose hovered fill (`fill`) or stroke (`stroke`) the
/// theme states nothing for on `presets`, its strokes laid at `align`.
const fn hover(
    leaf: &'static str,
    presets: &'static [&'static str],
    align: LineAlign,
    fill: bool,
    stroke: bool,
) -> Difference {
    Difference {
        leaf,
        presets,
        listed: Listed {
            align,
            hover: Hover { fill, stroke },
        },
        reason: match align {
            LineAlign::Outside => &[HOVER],
            LineAlign::Inside => &[INSIDE, HOVER],
            LineAlign::Center => &[CENTER, HOVER],
        },
    }
}

/// The presets on which a check box and a radio state a hover fill and no
/// hover stroke, the stroke outside the box.
const HOVER_FILL_ONLY: &[&str] = &["adwaita", "material-3", "ios-13"];

/// The presets on which a check box, a radio and a drop-down field state a
/// hover fill and no hover stroke, the stroke inside the box.
const HOVER_FILL_ONLY_INSIDE: &[&str] = &["windows-11", "macos-monterey"];

/// The listed differences.
const DIFFERENCES: &[Difference] = &[
    // button.body(NONE), its states and its variants.
    inside("button", MACOS),
    inside("button_toggle", MACOS),
    // windows-11's primary variant lays its stroke inside the box, the plain
    // button's outside (`button.body(NONE; variant=Primary).stroke`); its
    // default button is the primary one.
    inside(
        "button_primary",
        &["windows-11", "macos-monterey", "macos-tahoe"],
    ),
    inside(
        "button_default",
        &["windows-11", "macos-monterey", "macos-tahoe"],
    ),
    inside("button_danger", MACOS),
    inside("button_success", MACOS),
    inside("button_warning", MACOS),
    // input.field(NONE) and text_area.field(NONE).
    inside("text_input", MACOS),
    inside("text_editor", MACOS),
    // tooltip.panel, dialog.panel, progress_bar.track(NONE).
    inside("tooltip", MACOS_WINDOWS),
    inside("dialog", MACOS_WINDOWS),
    inside("progress_bar", MACOS_WINDOWS),
    // card.container(NONE).
    inside("container_card", &["windows-11"]),
    // Breeze's progress groove, a pen centred on its edge
    // (progress_bar.track(NONE).stroke).
    Difference {
        leaf: "progress_bar",
        presets: &["kde-breeze"],
        listed: Listed {
            align: LineAlign::Center,
            hover: Hover::STATED,
        },
        reason: &[CENTER],
    },
    // checkbox.indicator and radio.indicator: the hovered fill and stroke
    // (`indicator(HOVER)`, `indicator(HOVER|CHECKED)`), the strokes of
    // windows-11 and the macOS presets inside the box. On the macOS presets
    // that is the transparent unchecked stroke too, which iced is handed as
    // the fill under it (0.7.2's `checkbox.unchecked_border_color` is
    // `#00000000`, the home's stroke the same colour, Inside).
    hover("checkbox", &["kde-breeze"], LineAlign::Outside, true, false),
    hover("checkbox", HOVER_FILL_ONLY, LineAlign::Outside, false, true),
    hover(
        "checkbox",
        HOVER_FILL_ONLY_INSIDE,
        LineAlign::Inside,
        false,
        true,
    ),
    hover("checkbox", &["macos-tahoe"], LineAlign::Inside, true, true),
    hover("radio", &["kde-breeze"], LineAlign::Outside, true, false),
    hover("radio", HOVER_FILL_ONLY, LineAlign::Outside, false, true),
    hover(
        "radio",
        HOVER_FILL_ONLY_INSIDE,
        LineAlign::Inside,
        false,
        true,
    ),
    hover("radio", &["macos-tahoe"], LineAlign::Inside, true, true),
    // combo_box.field: the hovered fill and stroke (`field(HOVER)`).
    hover(
        "pick_list",
        &["kde-breeze"],
        LineAlign::Outside,
        true,
        false,
    ),
    hover(
        "pick_list",
        &["adwaita", "ios-13"],
        LineAlign::Outside,
        false,
        true,
    ),
    hover(
        "pick_list",
        HOVER_FILL_ONLY_INSIDE,
        LineAlign::Inside,
        false,
        true,
    ),
    hover("pick_list", &["macos-tahoe"], LineAlign::Inside, true, true),
    // switch.track(HOVER): macos-tahoe states no hovered track.
    hover("toggler", &["macos-tahoe"], LineAlign::Outside, true, false),
];

/// The leaves compared and the differences found, for one preset and scheme.
struct Report {
    label: String,
    preset: &'static str,
    compared: usize,
    /// The listed differences found differing: the entry's index, the style
    /// function and the preset.
    listed_seen: Vec<(usize, String, &'static str)>,
    failures: Vec<String>,
}

impl Report {
    fn new(preset: &'static str, is_dark: bool) -> Self {
        let scheme = if is_dark { "dark" } else { "light" };
        Self {
            label: format!("{preset}/{scheme}"),
            preset,
            compared: 0,
            listed_seen: Vec::new(),
            failures: Vec::new(),
        }
    }

    /// Compare one leaf; one that differs fails unless an entry lists its
    /// style function on this preset and the 0.8.0 value is the one
    /// `expected` gives for the entry's alignment.
    fn leaf<T: PartialEq + Debug>(
        &mut self,
        leaf: &str,
        old: T,
        new: T,
        expected: impl Fn(Listed) -> T,
    ) {
        self.compared += 1;
        if old == new {
            return;
        }
        let function = leaf.split('(').next().unwrap_or(leaf);
        let listed = DIFFERENCES
            .iter()
            .position(|d| d.leaf == function && d.presets.contains(&self.preset));
        let Some(i) = listed else {
            let diff = debug_diff(&format!("{old:?}"), &format!("{new:?}"));
            self.failures
                .push(format!("{}: {leaf}: {diff}", self.label));
            return;
        };
        let Some(listed) = DIFFERENCES.get(i).map(|d| d.listed) else {
            return;
        };
        let expected = expected(listed);
        if expected == new {
            self.listed_seen.push((i, function.to_owned(), self.preset));
        } else {
            let diff = debug_diff(&format!("{expected:?}"), &format!("{new:?}"));
            self.failures.push(format!(
                "{}: {leaf}: listed as {listed:?}, but not the expected value (expected -> \
                 0.8.0): {diff}",
                self.label
            ));
        }
    }

    /// The 0.8.0 side's value, or a failure where it is unmapped.
    fn mapped<T>(&mut self, leaf: &str, new: super::Mapped<T>) -> Option<T> {
        match new {
            Ok(v) => Some(v),
            Err(super::Unmapped(home)) => {
                self.failures
                    .push(format!("{}: {leaf}: 0.8.0 lacks {home}", self.label));
                None
            }
        }
    }
}

/// Where two `Debug` forms differ: each differing field, as the path of
/// the struct names and field names that lead to it, the first value and
/// the second.
fn debug_diff(old: &str, new: &str) -> String {
    let split = |s: &str| -> Vec<String> {
        let mut path: Vec<String> = Vec::new();
        let mut out = Vec::new();
        let mut word = String::new();
        for ch in s.chars() {
            match ch {
                '{' | '(' => {
                    path.push(word.trim().to_owned());
                    word.clear();
                }
                '}' | ')' | ',' => {
                    if !word.trim().is_empty() {
                        out.push(format!("{}.{}", path.join("."), word.trim()));
                    }
                    word.clear();
                    if ch != ',' {
                        path.pop();
                    }
                }
                _ => word.push(ch),
            }
        }
        if !word.trim().is_empty() {
            out.push(word.trim().to_owned());
        }
        out
    };
    let (o, n) = (split(old), split(new));
    if o.len() != n.len() {
        return format!("{old}, {new}");
    }
    o.iter()
        .zip(&n)
        .filter(|(a, b)| a != b)
        .map(|(a, b)| format!("{a} -> {}", b.rsplit(':').next().unwrap_or(b).trim()))
        .collect::<Vec<_>>()
        .join("; ")
}

/// The two resolved themes of `preset` for `is_dark`, and 0.7.2's name and
/// mode, which its theme holds beside the variants.
fn resolve(
    key: &str,
    preset: Preset,
    is_dark: bool,
) -> crate::Result<(
    ResolvedTheme,
    String,
    ColorMode,
    native_theme::next::ResolvedTheme,
)> {
    let (mode, ctx) = if is_dark {
        (ColorMode::Dark, ThemeContext::dark())
    } else {
        (ColorMode::Light, ThemeContext::light())
    };
    let theme = crate::Theme::preset(key)?;
    let name = theme.name.to_string();
    let old = theme
        .into_variant(mode)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
    let new = native_theme::next::Theme::preset(preset).resolve(&ctx);
    Ok((old, name, mode, new))
}

/// The iced theme's leaves.
fn compare_theme(report: &mut Report, old: &Theme, new: &super::Mapped<Theme>) {
    let Some(new) = report.mapped("to_theme", new.clone()) else {
        return;
    };
    let unlisted = |_| old.palette();
    report.leaf("to_theme.palette", old.palette(), new.palette(), unlisted);
    report.leaf(
        "to_theme.extended_palette",
        *old.extended_palette(),
        *new.extended_palette(),
        |_| *old.extended_palette(),
    );
    report.leaf("to_theme.name", old.to_string(), new.to_string(), |_| {
        old.to_string()
    });
}

/// Compare a style function's two outputs in each of `$statuses`, each side
/// called with its own iced theme; `$aligned` gives the 0.7.2 side's
/// function with its strokes laid at an alignment, the expected value of a
/// listed difference. A function of no status is called through `surface`.
#[cfg(feature = "widgets")]
macro_rules! style {
    ($report:expr, $themes:expr, $name:literal, $old:expr, $new:expr, $aligned:expr, $statuses:expr) => {{
        let aligned = $aligned;
        listed_style!(
            $report,
            $themes,
            $name,
            $old,
            $new,
            move |listed: Listed| aligned(listed.align),
            $statuses
        )
    }};
}

/// [`style`] of a style function that has hover options the 0.8.0 theme
/// can leave to iced: `$listed` gives the 0.7.2 side's function as the
/// whole [`Listed`] says.
#[cfg(feature = "widgets")]
macro_rules! listed_style {
    ($report:expr, $themes:expr, $name:literal, $old:expr, $new:expr, $listed:expr, $statuses:expr) => {{
        let (old_theme, new_theme): (&Theme, &Theme) = $themes;
        let old = $old;
        let listed = $listed;
        if let Some(new) = $report.mapped($name, $new) {
            for status in $statuses {
                $report.leaf(
                    &format!("{}({status:?})", $name),
                    old(old_theme, status),
                    new(new_theme, status),
                    |l: Listed| listed(l)(old_theme, status),
                );
            }
        }
    }};
}

/// A style function of no status as one of a single status.
#[cfg(feature = "widgets")]
fn surface<T>(f: impl Fn(&Theme) -> T + Clone) -> impl Fn(&Theme, ()) -> T + Clone {
    move |theme, ()| f(theme)
}

/// Compare a style function that has no stroke the theme aligns: an
/// unlisted difference fails.
#[cfg(feature = "widgets")]
macro_rules! plain {
    ($report:expr, $themes:expr, $name:literal, $old:expr, $new:expr, $statuses:expr) => {{
        let old = $old;
        let unaligned = old.clone();
        style!(
            $report,
            $themes,
            $name,
            old,
            $new,
            move |_| unaligned.clone(),
            $statuses
        )
    }};
}

/// The style functions ported so far.
#[cfg(feature = "widgets")]
fn compare_styles(
    report: &mut Report,
    themes: (&Theme, &Theme),
    old: &ResolvedTheme,
    new: &native_theme::next::ResolvedTheme,
) {
    use crate::palette::to_color;
    use crate::styles::{
        self as o, ButtonSource, CheckFills, CheckboxSource, InputSource, Outline, PickListSource,
        SwitchSource, ToolbarButtonSource,
    };
    use iced_widget::button::Status::{Active, Disabled, Hovered, Pressed};

    use super::styles as n;

    let button = [Active, Hovered, Pressed, Disabled];
    let b = ButtonSource::of(old);
    let at = move |a| b.aligned(a);
    style!(
        report,
        themes,
        "button",
        o::button(old),
        n::button(new),
        |a| o::button_from(&at(a)),
        button
    );
    for checked in [false, true] {
        style!(
            report,
            themes,
            "button_toggle",
            o::button_toggle(old, checked),
            n::button_toggle(new, checked),
            |a| o::button_toggle_from(&at(a), checked),
            button
        );
        let t = ToolbarButtonSource::of(old);
        style!(
            report,
            themes,
            "toolbar_button",
            o::toolbar_button(old, checked),
            n::toolbar_button(new, checked),
            |a| o::toolbar_button_from(&at(a), &t.aligned(a), checked),
            button
        );
    }
    style!(
        report,
        themes,
        "button_primary",
        o::button_primary(old),
        n::button_primary(new),
        |a| o::button_primary_from(&at(a)),
        button
    );
    style!(
        report,
        themes,
        "button_default",
        o::button_default(old),
        n::button_default(new),
        |a| o::button_default_from(&at(a)),
        button
    );
    let d = &old.defaults;
    for (name, fill, label, class, new_fn) in [
        (
            "button_danger",
            d.danger_color,
            d.danger_text_color,
            iced_widget::button::danger as fn(&Theme, _) -> _,
            n::button_danger(new).map(|f| Box::new(f) as Box<dyn Fn(&Theme, _) -> _>),
        ),
        (
            "button_success",
            d.success_color,
            d.success_text_color,
            iced_widget::button::success,
            n::button_success(new).map(|f| Box::new(f) as Box<dyn Fn(&Theme, _) -> _>),
        ),
        (
            "button_warning",
            d.warning_color,
            d.warning_text_color,
            iced_widget::button::warning,
            n::button_warning(new).map(|f| Box::new(f) as Box<dyn Fn(&Theme, _) -> _>),
        ),
    ] {
        let (old_theme, new_theme) = themes;
        let class_old = |a: Option<LineAlign>| {
            let source = a.map_or(b, |a| b.aligned(a));
            o::class_button(&source, to_color(fill), to_color(label), class)
        };
        let old_fn = class_old(None);
        if let Some(new_fn) = report.mapped(name, new_fn) {
            for status in button {
                report.leaf(
                    &format!("{name}({status:?})"),
                    old_fn(old_theme, status),
                    new_fn(new_theme, status),
                    |l: Listed| class_old(Some(l.align))(old_theme, status),
                );
            }
        }
    }
    plain!(
        report,
        themes,
        "button_link",
        o::button_link(old),
        n::button_link(new),
        button
    );

    {
        use iced_widget::text_input::Status;
        let field = [
            Status::Active,
            Status::Hovered,
            Status::Focused { is_hovered: false },
            Status::Focused { is_hovered: true },
            Status::Disabled,
        ];
        let i = InputSource::of(&old.input, &old.input.border, &old.defaults);
        style!(
            report,
            themes,
            "text_input",
            o::text_input(old),
            n::text_input(new),
            |a| o::text_input_from(&i.aligned(a)),
            field
        );
    }
    {
        use iced_widget::text_editor::Status;
        let field = [
            Status::Active,
            Status::Hovered,
            Status::Focused { is_hovered: false },
            Status::Focused { is_hovered: true },
            Status::Disabled,
        ];
        let i = InputSource::of(&old.input, &old.text_area.border, &old.defaults);
        style!(
            report,
            themes,
            "text_editor",
            o::text_editor(old),
            n::text_editor(new),
            |a| o::text_editor_from(&i.aligned(a)),
            field
        );
    }

    {
        use iced_widget::checkbox::Status;
        let boxes = [false, true].into_iter().flat_map(|is_checked| {
            [
                Status::Active { is_checked },
                Status::Hovered { is_checked },
                Status::Disabled { is_checked },
            ]
        });
        let c = CheckboxSource::of(old);
        let window = to_color(old.defaults.background_color);
        listed_style!(
            report,
            themes,
            "checkbox",
            o::checkbox(old),
            n::checkbox(new),
            |l: Listed| o::checkbox_from(&c.listed(l.align, l.hover), window),
            boxes
        );
    }
    {
        use iced_widget::radio::Status;
        let circles = [false, true].into_iter().flat_map(|is_selected| {
            [
                Status::Active { is_selected },
                Status::Hovered { is_selected },
            ]
        });
        let c = &old.checkbox;
        let fills = CheckFills::of(c);
        listed_style!(
            report,
            themes,
            "radio",
            o::radio(old),
            n::radio(new),
            |l: Listed| o::radio_from(
                fills.listed(l.align, l.hover),
                to_color(c.indicator_color),
                to_color(c.font.color),
                c.border.line_width,
            ),
            circles
        );
    }
    {
        use iced_widget::toggler::Status;
        let switches = [false, true].into_iter().flat_map(|is_toggled| {
            [
                Status::Active { is_toggled },
                Status::Hovered { is_toggled },
                Status::Disabled { is_toggled },
            ]
        });
        let s = SwitchSource::of(old);
        let window = to_color(old.defaults.background_color);
        listed_style!(
            report,
            themes,
            "toggler",
            o::toggler(old),
            n::toggler(new),
            |l: Listed| o::toggler_from(&s.listed(l.align, l.hover), window),
            switches
        );
    }
    {
        use iced_widget::pick_list::Status;
        let fields = [
            Status::Active,
            Status::Hovered,
            Status::Opened { is_hovered: false },
            Status::Opened { is_hovered: true },
        ];
        let c = PickListSource::of(old);
        listed_style!(
            report,
            themes,
            "pick_list",
            o::pick_list(old),
            n::pick_list(new),
            |l: Listed| o::pick_list_from(&c.listed(l.align, l.hover)),
            fields
        );
    }

    let p = &old.progress_bar;
    style!(
        report,
        themes,
        "progress_bar",
        surface(o::progress_bar(old)),
        n::progress_bar(new).map(surface),
        |a| surface(o::progress_bar_from(
            to_color(p.track_color),
            to_color(p.fill_color),
            Outline::of(&p.border).aligned(a),
        )),
        [()]
    );
    plain!(
        report,
        themes,
        "rule",
        surface(o::rule(old)),
        Ok(surface(n::rule(new))),
        [()]
    );
    plain!(
        report,
        themes,
        "toolbar_rule",
        surface(o::toolbar_rule(old)),
        n::toolbar_rule(new).map(surface),
        [()]
    );
    plain!(
        report,
        themes,
        "pane_grid",
        surface(o::pane_grid(old)),
        n::pane_grid(new).map(surface),
        [()]
    );
    let panel = |fill, label: Option<_>, border| {
        move |a: LineAlign| {
            surface(o::panel_from(
                to_color(fill),
                label.map(to_color),
                Some(Outline::of(border).aligned(a)),
            ))
        }
    };
    let t = &old.tooltip;
    style!(
        report,
        themes,
        "tooltip",
        surface(o::tooltip(old)),
        n::tooltip(new).map(surface),
        panel(t.background_color, Some(t.font.color), &t.border),
        [()]
    );
    let c = &old.card;
    style!(
        report,
        themes,
        "container_card",
        surface(o::container_card(old)),
        n::container_card(new).map(surface),
        panel(c.background_color, None, &c.border),
        [()]
    );
    let dl = &old.dialog;
    style!(
        report,
        themes,
        "dialog",
        surface(o::dialog(old)),
        n::dialog(new).map(surface),
        panel(dl.background_color, Some(dl.body_font.color), &dl.border),
        [()]
    );
    plain!(
        report,
        themes,
        "toolbar",
        surface(o::toolbar(old)),
        n::toolbar(new).map(surface),
        [()]
    );
    plain!(
        report,
        themes,
        "application",
        surface(o::application(old)),
        n::application(new).map(surface),
        [()]
    );
}

/// Every leaf of `preset` for `is_dark`, compared.
fn compare_all(key: &'static str, preset: Preset, is_dark: bool) -> crate::Result<Report> {
    let (old, name, mode, new) = resolve(key, preset, is_dark)?;
    let mut report = Report::new(key, is_dark);
    let old_theme = crate::to_theme_in(&old, &name, mode);
    let new_theme = super::to_theme(&new);
    compare_theme(&mut report, &old_theme, &new_theme);
    #[cfg(feature = "widgets")]
    if let Ok(new_theme) = &new_theme {
        compare_styles(&mut report, (&old_theme, new_theme), &old, &new);
    }
    Ok(report)
}

#[test]
fn the_connector_hands_iced_the_same_from_both_models() -> crate::Result<()> {
    let mut failures = Vec::new();
    let mut listed_seen = Vec::new();
    for &(key, preset) in PRESETS {
        for is_dark in [false, true] {
            let report = compare_all(key, preset, is_dark)?;
            println!("{}: {} leaves compared", report.label, report.compared);
            failures.extend(report.failures);
            listed_seen.extend(report.listed_seen);
        }
    }
    for (i, d) in DIFFERENCES.iter().enumerate() {
        for &p in d.presets {
            if !listed_seen
                .iter()
                .any(|(j, leaf, q)| *j == i && leaf == d.leaf && *q == p)
            {
                failures.push(format!(
                    "{} on {p}: listed, but differs nowhere: remove the preset from the entry",
                    d.leaf
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} leaves differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}

#[test]
fn every_listed_difference_names_a_covered_preset_once() {
    for (i, d) in DIFFERENCES.iter().enumerate() {
        assert!(!d.reason.is_empty(), "{}: no reason", d.leaf);
        for p in d.presets {
            assert!(
                PRESETS.iter().any(|(key, _)| key == p),
                "{}: {p} is not covered",
                d.leaf
            );
            assert!(
                !DIFFERENCES
                    .iter()
                    .skip(i + 1)
                    .any(|e| e.leaf == d.leaf && e.presets.contains(p)),
                "{} on {p}: listed twice",
                d.leaf
            );
        }
    }
}
