//! The contract's export on the 0.8.0 model: per colour home of
//! [`native_theme::next::ResolvedTheme`], the rows of the 0.8.0 path
//! ([`crate::next`], its `styles::*` outputs, its palette and its colour
//! accessors) a colour stated there moves, under which statuses, and the
//! platform presets in which it does not, and the 0.7.2 colour leaves the
//! table of homes places there. Exported as `contract-homes.toml` at the
//! crate root and held to the walk by
//! `every_colour_home_moves_a_row_or_is_named`.
//!
//! The rows are named as the 0.7.2 walk names them ([`Reached`]): a palette
//! slot of [`ROWS`], `styles::<fn>.<field>` for a field of
//! `crate::next::styles::<fn>`'s output, `crate::<fn>` for the colour
//! accessor `crate::next::<fn>`. The export is written by the 0.7.2 walk's
//! renderers ([`render_moves`], [`render_named`], [`render_rows`]).

use super::*;

use crate::next::Mapped;
use crate::next::styles as next_styles;
use crate::styles::SegmentPosition;
use native_theme::next::{Stroke, TextStated, ThemeContext, ThemePatch, WidgetState};
use native_theme_showcase_support::next::patch::home_patch_toml;

/// The table of homes, whose rows name the homes of the 0.7.2 colour leaves.
const HOMES_TABLE: &str = include_str!("../../../../native-theme/tests/next_homes.toml");

/// The exported home-to-row map as committed: the iced showcase reads it to
/// name the rows a colour home moves in its Widget Info.
const CONTRACT_HOMES: &str = include_str!("../../contract-homes.toml");

/// The command that rewrites `contract-homes.toml`, which
/// `every_colour_home_moves_a_row_or_is_named` names when the file is stale.
const REGENERATE_HOMES: &str =
    "cargo test -p native-theme-iced --all-features --lib -- --ignored regenerate_contract_homes";

/// The state queries the 0.8.0 style functions ask that the table of homes
/// names for no colour leaf (the homes of `crate::next::styles`' `Unmapped`
/// sources): a variant's outline, a disabled or pressed state 0.7.2 had no
/// field for.
const QUERIED_HOMES: &[&str] = &[
    "button.body(HOVER; variant=Primary).stroke.color",
    "button.body(NONE; variant=Primary).stroke.color",
    "button.body(PRESSED; variant=Primary).stroke.color",
    "checkbox.indicator(DISABLED).stroke.color",
    "checkbox.indicator(DISABLED|CHECKED).stroke.color",
    "expander.header(PRESSED).fill",
    "link.container(PRESSED).fill",
    "menu.panel(NONE).stroke.color",
    "scrollbar.thumb(HOVER).stroke.color",
    "scrollbar.thumb(NONE).stroke.color",
    "scrollbar.thumb(PRESSED).stroke.color",
    "segmented_control.segment(PRESSED).fill",
    "segmented_control.separator(NONE).fill",
    "slider.active_track(HOVER).fill",
    "slider.active_track(PRESSED).fill",
    "slider.thumb(PRESSED).fill",
    "switch.thumb(HOVER).fill",
    "switch.thumb(HOVER|CHECKED).fill",
    "tab.tab(NONE).stroke.color",
    "toolbar.button(PRESSED|HOVER|CHECKED).stroke.color",
    "toolbar.button_label(CHECKED).color",
    "toolbar.button_label(DISABLED).color",
    "toolbar.button_label(HOVER).color",
    "toolbar.button_label(PRESSED).color",
];

/// A colour home that moves no row in any preset, and why: the kind of
/// reason, as the 0.7.2 tables give it, and the reason.
struct UnreadHome {
    home: &'static str,
    kind: &'static str,
    why: &'static str,
}

/// Why a home of two flags the 0.8.0 path reads through `flagged` moves no
/// row in any preset.
const TWO_FLAGS: &str = "read only where a rule for the first flag alone states the \
     property (`flagged`, src/next/borders.rs; spec §2, `from_state`), which no platform \
     preset does; a rule for both flags alone stays unseen by that query, the two-flag gap \
     SQ1 settles";

/// Why iced draws no disabled radio button.
const NO_DISABLED_RADIO: &str = "iced 0.14's radio has no disabled status (iced_widget \
     0.14.2 src/radio.rs, `Status`: Active and Hovered), so `next::styles::radio` draws none";

/// Why no row reads the switch's label colour.
const SWITCH_LABEL: &str = "the showcase colours the switch's label, the text it lays beside \
     iced's `Toggler`; no connector style reads it (`next::styles::toggler` keeps iced's own \
     `text_color`, src/styles.rs, `toggler_from`)";

/// The colour homes no row reads, or no patch states, that the 0.7.2 tables
/// do not already name by a leaf the table of homes places there.
const UNREAD_HOMES: &[UnreadHome] = &[
    UnreadHome {
        home: "radio.indicator(DISABLED).fill",
        kind: "no receiver",
        why: NO_DISABLED_RADIO,
    },
    UnreadHome {
        home: "radio.indicator(DISABLED|CHECKED).fill",
        kind: "no receiver",
        why: NO_DISABLED_RADIO,
    },
    UnreadHome {
        home: "radio.label(DISABLED).color",
        kind: "no receiver",
        why: NO_DISABLED_RADIO,
    },
    UnreadHome {
        home: "slider.thumb(HOVER|FOCUSED).stroke.color",
        kind: "no receiver",
        why: "iced 0.14's slider has no focused status (iced_widget 0.14.2 src/slider.rs, \
             `Status`: Active, Hovered and Dragged)",
    },
    UnreadHome {
        home: "switch.label(CHECKED).color",
        kind: "ruled",
        why: SWITCH_LABEL,
    },
    UnreadHome {
        home: "switch.label(DISABLED).color",
        kind: "ruled",
        why: SWITCH_LABEL,
    },
    UnreadHome {
        home: "switch.label(DISABLED|CHECKED).color",
        kind: "ruled",
        why: SWITCH_LABEL,
    },
    UnreadHome {
        home: "switch.label(NONE).color",
        kind: "ruled",
        why: SWITCH_LABEL,
    },
    UnreadHome {
        home: "switch.thumb(HOVER|CHECKED).fill",
        kind: "ruled",
        why: TWO_FLAGS,
    },
    UnreadHome {
        home: "window.frame(normal).fill",
        kind: "no patch",
        why: "`ThemePatch::from_toml` takes no `widgets.window` table (the walk's patch for it \
             does not parse) and `WindowChrome`'s parts are private, so the walk states no \
             colour there; `next::styles::application` reads it (src/next/styles.rs, \
             `application`)",
    },
];

/// Two opaque colours and a translucent one stated at a home in turn: a row
/// that reads the home tells them apart, one composited under an opaque
/// layer by the translucent one.
const COLOURS: [&str; 3] = ["#13579b", "#2468ac", "#2468ac80"];

/// `r` with `colour` stated at `home`: by a patch ([`home_patch_toml`]), or,
/// for the globals no patch states (`ThemePatch` holds no focus ring and no
/// text colour of a `Typography` slot), written into the resolved theme with
/// its stated bit, a ring's stroke 1 px wide where it states none. `None`
/// where neither reaches the home.
fn with_home(
    r: &native_theme::next::ResolvedTheme,
    home: &str,
    colour: &str,
) -> Option<native_theme::next::ResolvedTheme> {
    let mut patched = r.clone();
    let color: native_theme::next::Color = colour.parse().ok()?;
    match home {
        "focus_ring.stroke.color" => {
            let width = match patched.focus_ring.stroke {
                Stroke::Uniform { width, .. } => width,
                _ => 1.0,
            };
            patched.focus_ring.stroke = Stroke::uniform(width, color);
        }
        "typography.body.color" | "typography.monospace.color" => {
            let text = if home == "typography.body.color" {
                &mut patched.typography.body
            } else {
                &mut patched.typography.monospace
            };
            text.color = color;
            text.stated |= TextStated::COLOR;
        }
        _ => {
            let patch = ThemePatch::from_toml(&home_patch_toml(home, colour)?).ok()?;
            patched.apply_patch(&patch);
        }
    }
    Some(patched)
}

/// The name of a field under `prefix`, the row's own name where `prefix` is
/// empty.
fn named(prefix: &str, field: &str) -> String {
    if prefix.is_empty() {
        field.to_owned()
    } else {
        format!("{prefix}.{field}")
    }
}

/// Each field of `$value`, a `$ty` destructured with no `..`, under `$prefix`
/// by its name or the name `=>` gives it, with its value as `{:?}` writes it:
/// an upstream field added fails to compile here.
macro_rules! texts {
    (
        $out:ident, $value:expr, $prefix:expr,
        $($ty:ident)::+ { $($field:ident $(=> $name:literal)?),* $(,)? }
    ) => {
        let $($ty)::+ { $($field,)* } = $value;
        $($out.push((named($prefix, texts!(@name $field $(, $name)?)), format!("{:?}", $field)));)*
    };
    (@name $field:ident) => { stringify!($field) };
    (@name $field:ident, $name:literal) => { $name };
}

/// The fields of an output, each by the name its row takes after the
/// function's, with its value as text.
type Texts = Vec<(String, String)>;

fn button_texts(s: &button::Style) -> Texts {
    let mut out = Vec::new();
    texts!(
        out,
        s,
        "",
        button::Style {
            background,
            text_color,
            border,
            shadow,
            snap
        }
    );
    out
}

fn container_texts(s: &container::Style) -> Texts {
    let mut out = Vec::new();
    texts!(
        out,
        s,
        "",
        container::Style {
            text_color,
            background,
            border,
            shadow,
            snap
        }
    );
    out
}

fn text_input_texts(s: &text_input::Style) -> Texts {
    let mut out = Vec::new();
    texts!(
        out,
        s,
        "",
        text_input::Style {
            background,
            border,
            icon,
            placeholder,
            value,
            selection
        }
    );
    out
}

fn text_editor_texts(s: &text_editor::Style) -> Texts {
    let mut out = Vec::new();
    texts!(
        out,
        s,
        "",
        text_editor::Style {
            background,
            border,
            placeholder,
            value,
            selection
        }
    );
    out
}

fn checkbox_texts(s: &checkbox::Style) -> Texts {
    let mut out = Vec::new();
    texts!(
        out,
        s,
        "",
        checkbox::Style {
            background,
            icon_color,
            border,
            text_color
        }
    );
    out
}

fn radio_texts(s: &radio::Style) -> Texts {
    let mut out = Vec::new();
    texts!(
        out,
        s,
        "",
        radio::Style {
            background,
            dot_color,
            border_width,
            border_color,
            text_color
        }
    );
    out
}

fn toggler_texts(s: &toggler::Style) -> Texts {
    let mut out = Vec::new();
    texts!(
        out,
        s,
        "",
        toggler::Style {
            background,
            background_border_width,
            background_border_color,
            foreground,
            foreground_border_width,
            foreground_border_color,
            text_color,
            border_radius,
            padding_ratio
        }
    );
    out
}

fn pick_list_texts(s: &pick_list::Style) -> Texts {
    let mut out = Vec::new();
    texts!(
        out,
        s,
        "",
        pick_list::Style {
            text_color,
            placeholder_color,
            handle_color,
            background,
            border
        }
    );
    out
}

fn menu_texts(s: &menu::Style) -> Texts {
    let mut out = Vec::new();
    texts!(
        out,
        s,
        "",
        menu::Style {
            background,
            border,
            text_color,
            selected_text_color,
            selected_background,
            shadow
        }
    );
    out
}

fn slider_texts(s: &slider::Style) -> Texts {
    let mut out = Vec::new();
    let slider::Style { rail, handle } = s;
    let slider::Rail {
        backgrounds: (filled, remaining),
        width,
        border,
    } = rail;
    out.push(("rail.backgrounds.0".to_owned(), format!("{filled:?}")));
    out.push(("rail.backgrounds.1".to_owned(), format!("{remaining:?}")));
    out.push(("rail.width".to_owned(), format!("{width:?}")));
    out.push(("rail.border".to_owned(), format!("{border:?}")));
    texts!(
        out,
        handle,
        "handle",
        slider::Handle {
            shape,
            background,
            border_width,
            border_color
        }
    );
    out
}

fn scrollable_texts(s: &scrollable::Style) -> Texts {
    let mut out = Vec::new();
    let scrollable::Style {
        container,
        vertical_rail,
        horizontal_rail,
        gap,
        auto_scroll,
    } = s;
    out.extend(
        container_texts(container)
            .into_iter()
            .map(|(field, text)| (named("container", &field), text)),
    );
    for (rail, prefix) in [
        (vertical_rail, "vertical_rail"),
        (horizontal_rail, "horizontal_rail"),
    ] {
        let scrollable::Rail {
            background,
            border,
            scroller,
        } = rail;
        out.push((named(prefix, "background"), format!("{background:?}")));
        out.push((named(prefix, "border"), format!("{border:?}")));
        texts!(
            out,
            scroller,
            &named(prefix, "scroller"),
            scrollable::Scroller { background, border }
        );
    }
    out.push(("gap".to_owned(), format!("{gap:?}")));
    texts!(
        out,
        auto_scroll,
        "auto_scroll",
        scrollable::AutoScroll {
            background,
            border,
            shadow,
            icon
        }
    );
    out
}

fn progress_bar_texts(s: &progress_bar::Style) -> Texts {
    let mut out = Vec::new();
    texts!(
        out,
        s,
        "",
        progress_bar::Style {
            background,
            bar,
            border
        }
    );
    out
}

fn rule_texts(s: &rule::Style) -> Texts {
    let mut out = Vec::new();
    texts!(
        out,
        s,
        "",
        rule::Style {
            color,
            radius,
            fill_mode,
            snap
        }
    );
    out
}

fn pane_grid_texts(s: &pane_grid::Style) -> Texts {
    let mut out = Vec::new();
    let pane_grid::Style {
        hovered_region,
        picked_split,
        hovered_split,
    } = s;
    texts!(
        out,
        hovered_region,
        "hovered_region",
        pane_grid::Highlight { background, border }
    );
    for (line, prefix) in [
        (picked_split, "picked_split"),
        (hovered_split, "hovered_split"),
    ] {
        texts!(out, line, prefix, pane_grid::Line { color, width });
    }
    out
}

fn application_texts(s: &iced_core::theme::Style) -> Texts {
    let mut out = Vec::new();
    texts!(
        out,
        s,
        "",
        iced_core::theme::Style {
            background_color,
            text_color
        }
    );
    out
}

fn aw_card_texts(s: &card::Style) -> Texts {
    let mut out = Vec::new();
    texts!(
        out,
        s,
        "",
        card::Style {
            background,
            border_radius,
            border_width,
            border_color,
            head_background,
            head_text_color,
            body_background,
            body_text_color,
            foot_background,
            foot_text_color,
            close_color
        }
    );
    out
}

fn aw_menu_texts(s: &menu_bar::Style) -> Texts {
    let mut out = Vec::new();
    texts!(out, s, "", menu_bar::Style {
        bar_background, bar_border => "bar.border", bar_shadow, menu_background,
        menu_border => "menu.border", menu_shadow, path, path_border => "path.border"
    });
    out
}

fn aw_tab_bar_texts(s: &tab_bar::Style) -> Texts {
    let mut out = Vec::new();
    texts!(
        out,
        s,
        "",
        tab_bar::Style {
            background,
            border_color,
            border_width,
            tab_border_radius,
            tab_label_background,
            tab_label_border_color,
            tab_label_border_width,
            icon_color,
            icon_background,
            icon_border_radius,
            text_color
        }
    );
    out
}

fn aw_sidebar_texts(s: &sidebar::Style) -> Texts {
    let mut out = Vec::new();
    texts!(
        out,
        s,
        "",
        sidebar::Style {
            background,
            border_color,
            border_width,
            tab_label_background,
            tab_label_border_color,
            tab_label_border_width,
            icon_color,
            icon_background,
            icon_border_radius,
            text_color
        }
    );
    out
}

fn aw_selection_list_texts(s: &selection_list::Style) -> Texts {
    let mut out = Vec::new();
    texts!(
        out,
        s,
        "",
        selection_list::Style {
            text_color,
            background,
            border_width,
            border_color
        }
    );
    out
}

/// A style function that takes no status, as one whose status is `()`: its
/// rows are recorded with none.
fn statusless<O>(style: impl Fn(&Theme) -> O) -> impl Fn(&Theme, ()) -> O {
    move |theme, ()| style(theme)
}

/// A style function built for a flag (`checked`, `selected`), as one whose
/// status is the flag with the widget's own: the 0.7.2 rows' `(bool, Status)`.
fn flagged<S, O, F: Fn(&Theme, S) -> O>(
    style: impl Fn(bool) -> Mapped<F>,
) -> Mapped<impl Fn(&Theme, (bool, S)) -> O> {
    let (on, off) = (style(true)?, style(false)?);
    Ok(move |theme: &Theme, (flag, status): (bool, S)| {
        if flag {
            on(theme, status)
        } else {
            off(theme, status)
        }
    })
}

/// A status-less style function built for a flag (`disabled`), as one whose
/// status is the flag.
fn by_flag<O, F: Fn(&Theme) -> O>(
    style: impl Fn(bool) -> Mapped<F>,
) -> Mapped<impl Fn(&Theme, bool) -> O> {
    let (on, off) = (style(true)?, style(false)?);
    Ok(move |theme: &Theme, flag: bool| if flag { on(theme) } else { off(theme) })
}

/// Every row of the 0.8.0 path on one theme, as text keyed by the row's name
/// and status: the palette slots [`ROWS`] names, each field of each
/// `crate::next::styles` output under each status, and each colour accessor.
/// A colour stated at a home moves a row in a status when that text changes.
#[derive(Default)]
struct NextReading(BTreeMap<Reached, String>);

impl NextReading {
    fn style<S: Copy + std::fmt::Debug, O>(
        &mut self,
        t: &Theme,
        function: &str,
        style: Mapped<impl Fn(&Theme, S) -> O>,
        statuses: &[S],
        texts: fn(&O) -> Texts,
    ) {
        let style = match style {
            Ok(style) => style,
            Err(unmapped) => {
                self.0.insert(Reached::row(function), unmapped.to_string());
                return;
            }
        };
        for &status in statuses {
            for (field, text) in texts(&style(t, status)) {
                self.0
                    .insert(Reached::of(&named(function, &field), status), text);
            }
        }
    }

    fn accessor(&mut self, accessor: &str, value: impl std::fmt::Debug) {
        self.0.insert(Reached::row(accessor), format!("{value:?}"));
    }

    fn of(r: &native_theme::next::ResolvedTheme) -> Self {
        let mut x = Self::default();
        let t = match crate::next::to_theme(r) {
            Ok(t) => t,
            Err(unmapped) => {
                x.0.insert(Reached::row("crate::to_theme"), unmapped.to_string());
                return x;
            }
        };
        let p = t.palette();
        let mut slots: Vec<(String, Color)> = [
            ("palette.background", p.background),
            ("palette.text", p.text),
            ("palette.primary", p.primary),
            ("palette.success", p.success),
            ("palette.warning", p.warning),
            ("palette.danger", p.danger),
        ]
        .map(|(slot, color)| (slot.to_owned(), color))
        .into();
        slots.extend(extended_slots(t.extended_palette()));
        for row in ROWS {
            if let Some((_, color)) = slots.iter().find(|(slot, _)| slot == row.slot) {
                x.accessor(row.slot, color);
            }
        }

        x.accessor("crate::border_color", crate::next::border_color(r));
        x.accessor("crate::focus_ring_color", crate::next::focus_ring_color(r));
        x.accessor("crate::link_color", crate::next::link_color(r));
        x.accessor(
            "crate::expander_arrow_color",
            crate::next::expander_arrow_color(r),
        );
        x.accessor(
            "crate::dialog_title_font_color",
            crate::next::dialog_title_font_color(r),
        );
        x.accessor(
            "crate::selection_background",
            crate::next::selection_background(r),
        );
        x.accessor(
            "crate::list_header_hover_background",
            crate::next::list_header_hover_background(r),
        );
        x.accessor(
            "crate::list_header_active_background",
            crate::next::list_header_active_background(r),
        );
        x.accessor("crate::info_color", crate::next::info_color(r));
        x.accessor("crate::info_text_color", crate::next::info_text_color(r));
        x.accessor(
            "crate::warning_text_color",
            crate::next::warning_text_color(r),
        );
        x.accessor(
            "crate::disabled_text_color",
            crate::next::disabled_text_color(r),
        );
        x.accessor(
            "crate::tab_indicator_line",
            crate::next::tab_indicator_look(r).map(|look| look.map(|look| look.color)),
        );
        let flares = crate::next::tab_flares_look(r);
        x.accessor(
            "crate::tab_flares (fill)",
            flares
                .as_ref()
                .map(|look| look.as_ref().map(|look| look.fill)),
        );
        x.accessor(
            "crate::tab_flares (outline)",
            flares.as_ref().map(|look| {
                look.as_ref()
                    .filter(|look| look.width > 0.0)
                    .map(|look| look.outline)
            }),
        );
        // The ring round a field of the theme's corner radius, as the
        // root parity reads it: its colour and its inner ring's.
        x.accessor(
            "crate::focus_ring_with",
            crate::next::ring_look(r, crate::next::border_corner_radius(r))
                .map(|look| look.map(|look| (look.color, look.inner.map(|(color, _)| color)))),
        );
        x.accessor(
            "crate::radio (dot)",
            crate::next::radio_look(r).map(|look| look.dot.map(|(_, color)| color)),
        );
        #[cfg(feature = "spinner")]
        {
            let spinner = crate::next::spinner_look(r);
            x.accessor(
                "crate::Spinner (arc, bundled tint)",
                spinner.as_ref().map(|look| look.color),
            );
            x.accessor(
                "crate::Spinner (freedesktop currentColor)",
                spinner.as_ref().map(|look| look.current_color),
            );
        }

        let s = &t;
        x.style(
            s,
            "styles::button",
            next_styles::button(r),
            BUTTON_STATUSES,
            button_texts,
        );
        x.style(
            s,
            "styles::button_primary",
            next_styles::button_primary(r),
            BUTTON_STATUSES,
            button_texts,
        );
        x.style(
            s,
            "styles::button_default",
            next_styles::button_default(r),
            BUTTON_STATUSES,
            button_texts,
        );
        x.style(
            s,
            "styles::button_danger",
            next_styles::button_danger(r),
            BUTTON_STATUSES,
            button_texts,
        );
        x.style(
            s,
            "styles::button_success",
            next_styles::button_success(r),
            BUTTON_STATUSES,
            button_texts,
        );
        x.style(
            s,
            "styles::button_warning",
            next_styles::button_warning(r),
            BUTTON_STATUSES,
            button_texts,
        );
        x.style(
            s,
            "styles::button_link",
            next_styles::button_link(r),
            BUTTON_STATUSES,
            button_texts,
        );
        x.style(
            s,
            "styles::button_toggle",
            flagged(|checked| next_styles::button_toggle(r, checked)),
            BUTTON_TOGGLE_STATUSES,
            button_texts,
        );
        x.style(
            s,
            "styles::toolbar_button",
            flagged(|checked| next_styles::toolbar_button(r, checked)),
            TOOLBAR_BUTTON_STATUSES,
            button_texts,
        );
        x.style(
            s,
            "styles::expander",
            next_styles::expander(r),
            BUTTON_STATUSES,
            button_texts,
        );
        x.style(
            s,
            "styles::segment",
            flagged(|selected| next_styles::segment(r, selected, SegmentPosition::Middle)),
            SEGMENT_STATUSES,
            button_texts,
        );
        x.style(
            s,
            "styles::text_input",
            next_styles::text_input(r),
            TEXT_INPUT_STATUSES,
            text_input_texts,
        );
        x.style(
            s,
            "styles::text_editor",
            next_styles::text_editor(r),
            TEXT_EDITOR_STATUSES,
            text_editor_texts,
        );
        x.style(
            s,
            "styles::checkbox",
            next_styles::checkbox(r),
            CHECKBOX_STATUSES,
            checkbox_texts,
        );
        x.style(
            s,
            "styles::radio",
            next_styles::radio(r),
            RADIO_STATUSES,
            radio_texts,
        );
        x.style(
            s,
            "styles::toggler",
            next_styles::toggler(r),
            TOGGLER_STATUSES,
            toggler_texts,
        );
        x.style(
            s,
            "styles::pick_list",
            next_styles::pick_list(r),
            PICK_LIST_STATUSES,
            pick_list_texts,
        );
        x.style(
            s,
            "styles::menu",
            next_styles::menu(r).map(statusless),
            &[()],
            menu_texts,
        );
        x.style(
            s,
            "styles::slider",
            next_styles::slider(r),
            SLIDER_STATUSES,
            slider_texts,
        );
        x.style(
            s,
            "styles::vertical_slider",
            next_styles::vertical_slider(r),
            SLIDER_STATUSES,
            slider_texts,
        );
        x.style(
            s,
            "styles::scrollable",
            next_styles::scrollable(r, WidgetState::NONE),
            SCROLLABLE_STATUSES,
            scrollable_texts,
        );
        x.style(
            s,
            "styles::scrollable(UNFOCUSED_VIEW)",
            next_styles::scrollable(r, WidgetState::UNFOCUSED_VIEW),
            SCROLLABLE_STATUSES,
            scrollable_texts,
        );
        x.style(
            s,
            "styles::segmented_control",
            by_flag(|disabled| next_styles::segmented_control(r, disabled)),
            SEGMENTED_CONTROL_STATUSES,
            container_texts,
        );
        x.style(
            s,
            "styles::segment_separator",
            by_flag(|disabled| next_styles::segment_separator(r, disabled)),
            SEGMENTED_CONTROL_STATUSES,
            container_texts,
        );
        x.style(
            s,
            "styles::expander_frame",
            next_styles::expander_frame(r).map(statusless),
            &[()],
            container_texts,
        );
        x.style(
            s,
            "styles::tooltip",
            next_styles::tooltip(r).map(statusless),
            &[()],
            container_texts,
        );
        x.style(
            s,
            "styles::container_card",
            next_styles::container_card(r).map(statusless),
            &[()],
            container_texts,
        );
        x.style(
            s,
            "styles::dialog",
            next_styles::dialog(r).map(statusless),
            &[()],
            container_texts,
        );
        x.style(
            s,
            "styles::toolbar",
            next_styles::toolbar(r).map(statusless),
            &[()],
            container_texts,
        );
        x.style(
            s,
            "styles::progress_bar",
            next_styles::progress_bar(r).map(statusless),
            &[()],
            progress_bar_texts,
        );
        x.style(
            s,
            "styles::rule",
            Ok(statusless(next_styles::rule(r))),
            &[()],
            rule_texts,
        );
        x.style(
            s,
            "styles::toolbar_rule",
            next_styles::toolbar_rule(r).map(statusless),
            &[()],
            rule_texts,
        );
        x.style(
            s,
            "styles::pane_grid",
            next_styles::pane_grid(r).map(statusless),
            &[()],
            pane_grid_texts,
        );
        x.style(
            s,
            "styles::application",
            next_styles::application(r).map(statusless),
            &[()],
            application_texts,
        );
        x.style(
            s,
            "styles::aw::card",
            next_styles::aw::card(r),
            AW_STATUSES,
            aw_card_texts,
        );
        x.style(
            s,
            "styles::aw::menu",
            next_styles::aw::menu(r),
            AW_STATUSES,
            aw_menu_texts,
        );
        x.style(
            s,
            "styles::aw::tab_bar",
            next_styles::aw::tab_bar(r),
            AW_STATUSES,
            aw_tab_bar_texts,
        );
        x.style(
            s,
            "styles::aw::sidebar",
            next_styles::aw::sidebar(r),
            AW_STATUSES,
            aw_sidebar_texts,
        );
        x.style(
            s,
            "styles::aw::selection_list",
            next_styles::aw::selection_list(r),
            AW_STATUSES,
            aw_selection_list_texts,
        );
        x.style(
            s,
            "styles::aw::spinner",
            Ok(statusless(next_styles::aw::spinner(r))),
            &[()],
            container_texts,
        );
        x
    }

    /// The rows, under each status, whose text differs between two readings.
    fn moved(&self, other: &Self) -> BTreeSet<Reached> {
        self.0
            .iter()
            .filter(|(key, text)| other.0.get(*key) != Some(*text))
            .map(|(key, _)| key.clone())
            .chain(
                other
                    .0
                    .keys()
                    .filter(|key| !self.0.contains_key(*key))
                    .cloned(),
            )
            .collect()
    }
}

/// What the colour-home walk finds over the platform presets in both
/// schemes.
struct HomeReach {
    /// The combinations' names, `<preset>/<scheme>`, in the walk's order.
    labels: Vec<String>,
    /// Every colour home, with the 0.7.2 colour leaves the table of homes
    /// places there (none for a [`QUERIED_HOMES`] entry).
    homes: BTreeMap<String, Vec<String>>,
    /// Per home, each row and status a colour there moves, with the bases
    /// (indices into `labels`) it moves it in.
    moves: BTreeMap<String, BTreeMap<Reached, BTreeSet<usize>>>,
    /// The homes no patch states a colour at, with the base it failed in.
    unpatched: BTreeMap<String, String>,
}

/// The colour homes: per 0.7.2 colour leaf (a colour leaf of a bundled
/// combination's serialised `ResolvedTheme`, as the 0.7.2 walk splices
/// them) the homes its row of the table of homes names, `homes_by_preset`'s
/// too, and [`QUERIED_HOMES`].
fn colour_homes() -> Result<BTreeMap<String, Vec<String>>, String> {
    let mut leaves: BTreeSet<String> = BTreeSet::new();
    for c in combinations().map_err(|e| format!("{e}"))? {
        let json = serde_json::to_value(&c.resolved)
            .map_err(|e| format!("{} does not serialise: {e}", c.label()))?;
        leaves.extend(colour_leaves(&json));
    }
    let table: toml::Table = HOMES_TABLE.parse().map_err(|e| format!("{e}"))?;
    let rows = table
        .get("row")
        .and_then(toml::Value::as_array)
        .ok_or("next_homes.toml has no rows")?;
    let strings = |value: Option<&toml::Value>| -> Vec<String> {
        value
            .and_then(toml::Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(toml::Value::as_str)
                    .map(str::to_owned)
                    .collect()
            })
            .unwrap_or_default()
    };
    let mut homes: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for row in rows {
        let Some(key) = row.get("key").and_then(toml::Value::as_str) else {
            return Err("a row of next_homes.toml has no key".to_owned());
        };
        if !leaves.contains(key) {
            continue;
        }
        let mut named = strings(row.get("homes"));
        if let Some(by_preset) = row.get("homes_by_preset").and_then(toml::Value::as_table) {
            for value in by_preset.values() {
                named.extend(strings(Some(value)));
            }
        }
        for home in named {
            let keys = homes.entry(home).or_default();
            if !keys.iter().any(|k| k == key) {
                keys.push(key.to_owned());
            }
        }
    }
    for home in QUERIED_HOMES {
        homes.entry((*home).to_owned()).or_default();
    }
    Ok(homes)
}

/// State each colour of [`COLOURS`] at every colour home in every base, and
/// record the rows that tell them apart.
///
/// Each of the seven platform presets in both schemes is a base. A row a
/// home reaches in one base is not reached in every base (a soft state's
/// rule the preset states, a stroke it leaves unstated), which the export
/// records. The bases are independent, so each is walked on a thread of its
/// own.
fn reach_homes() -> Result<HomeReach, String> {
    let homes = colour_homes()?;
    let bases: Vec<(String, native_theme::next::ResolvedTheme)> = crate::next::parity::PRESETS
        .iter()
        .flat_map(|&(key, preset)| {
            [
                ("light", ThemeContext::light()),
                ("dark", ThemeContext::dark()),
            ]
            .map(|(scheme, ctx)| {
                (
                    format!("{key}/{scheme}"),
                    native_theme::next::Theme::preset(preset).resolve(&ctx),
                )
            })
        })
        .collect();

    type Found = (
        BTreeMap<String, BTreeSet<Reached>>,
        BTreeMap<String, String>,
    );
    let per_base: Vec<Found> = std::thread::scope(|scope| {
        let walks: Vec<_> = bases
            .iter()
            .map(|(label, r)| {
                let homes = &homes;
                scope.spawn(move || {
                    let mut moved = BTreeMap::new();
                    let mut unpatched = BTreeMap::new();
                    for home in homes.keys() {
                        let readings: Option<Vec<NextReading>> = COLOURS
                            .iter()
                            .map(|colour| with_home(r, home, colour).map(|p| NextReading::of(&p)))
                            .collect();
                        let Some(readings) = readings else {
                            unpatched.insert(home.clone(), label.clone());
                            continue;
                        };
                        let rows: BTreeSet<Reached> = readings
                            .split_first()
                            .map(|(first, rest)| {
                                rest.iter().flat_map(|other| first.moved(other)).collect()
                            })
                            .unwrap_or_default();
                        if !rows.is_empty() {
                            moved.insert(home.clone(), rows);
                        }
                    }
                    (moved, unpatched)
                })
            })
            .collect();
        walks
            .into_iter()
            .map(|walk| {
                walk.join()
                    .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
            })
            .collect()
    });

    let mut moves: BTreeMap<String, BTreeMap<Reached, BTreeSet<usize>>> = BTreeMap::new();
    let mut unpatched = BTreeMap::new();
    for (i, (moved, failed)) in per_base.into_iter().enumerate() {
        for (home, rows) in moved {
            let entry = moves.entry(home).or_default();
            for row in rows {
                entry.entry(row).or_default().insert(i);
            }
        }
        for (home, label) in failed {
            unpatched.entry(home).or_insert(label);
        }
    }
    Ok(HomeReach {
        labels: bases.into_iter().map(|(label, _)| label).collect(),
        homes,
        moves,
        unpatched,
    })
}

/// The entry that names a home no row reads: [`UNREAD_HOMES`]'s, else the
/// 0.7.2 tables' entry for a leaf the table of homes places there, as the
/// 0.7.2 export records it (the table, the entry, the kind, the reason).
fn home_naming(
    home: &str,
    keys: &[String],
) -> Option<(&'static str, String, String, &'static str)> {
    if let Some(entry) = UNREAD_HOMES.iter().find(|e| e.home == home) {
        return Some((
            "UNREAD_HOMES",
            entry.home.to_owned(),
            entry.kind.to_owned(),
            entry.why,
        ));
    }
    keys.iter().find_map(|key| {
        naming(key).map(|(table, entry, kind, why)| (table, entry.to_owned(), kind, why))
    })
}

/// `contract-homes.toml` as the walk finds it.
fn render_contract_homes(reach: &HomeReach) -> String {
    use std::fmt::Write as _;

    let mut out = String::new();
    let _ = write!(
        out,
        "\
# connectors/native-theme-iced/contract-homes.toml -- generated; do not edit.
# Written by `{REGENERATE_HOMES}`
# from the contract's colour-home walk (src/contract/homes.rs, `reach_homes`), and held to it
# by `every_colour_home_moves_a_row_or_is_named`, which fails while the two differ. That test
# exists only with `--features iced_aw` (check_release's and CI's `iced_aw,watch,native`
# run); a default `cargo test` does not compare the file.
# The iced showcase reads it for the colour rows of its Widget Info.
#
# Every colour home of the 0.8.0 model -- the homes native-theme/tests/next_homes.toml names
# for the 0.7.2 model's colour leaves (those of a bundled preset's serialised ResolvedTheme),
# and the state queries the 0.8.0 style functions ask that it names for none -- stated in
# each of the {} platform preset/scheme combinations in two opaque colours and a translucent
# one in turn (a patch as showcase-support's `home_patch_toml` writes it; the focus ring's
# and a text style's colour written into the resolved theme), is a [homes.\"<home>\"] table:
#   leaves: the 0.7.2 colour leaves next_homes.toml places at the home; absent for a state
#           query it names for none;
#   moves: each row of the 0.8.0 path (`native_theme_iced::next`) the colours tell apart, as
#          {{ row, statuses, except }}:
#     row       a palette slot (`palette.*`, `extended.*`), a colour accessor or the
#               connector's own widget (`crate::<fn>`: `next::<fn>`), or a field of a style
#               function's output (`styles::<fn>.<field>`: `next::styles::<fn>`; a border is
#               the whole border);
#     statuses  the widget statuses, as `{{:?}}` writes them, it moves the row under;
#               absent for a row that takes no status;
#     except    the combinations (`<preset>/<scheme>`) in which it does not move the row
#               under those statuses; absent where it moves it in all {}.
#   A home that moves no row in any combination, or that no patch states a colour at, has
#   instead the entry that names it:
#     named     UNREAD_HOMES (src/contract/homes.rs), or UNREAD_COLOURS or UNREACHABLE
#               (src/contract/derived.rs) for a 0.7.2 leaf the home holds;
#     entry     the entry's name;
#     kind      why: no receiver, inherited, no widget, ruled, fallback of <option>, read at
#               another query, not read, or no patch (the walk cannot state a colour there);
#     why       the entry's evidence.
# Every row a home moves is a [rows.\"<row>\"] table: homes, the homes that move it, and cfg,
# the feature it exists under where a build can lack it.
# combinations lists the {} combinations' names, in the walk's order.

combinations = {}
",
        reach.labels.len(),
        reach.labels.len(),
        reach.labels.len(),
        toml_strings(reach.labels.iter().map(String::as_str))
    );
    for (home, keys) in &reach.homes {
        let _ = write!(out, "\n[homes.{}]\n", toml_string(home));
        if !keys.is_empty() {
            let _ = writeln!(
                out,
                "leaves = {}",
                toml_strings(keys.iter().map(String::as_str))
            );
        }
        match reach.moves.get(home) {
            Some(rows) => render_moves(&mut out, rows, &reach.labels),
            None => {
                if let Some((named, entry, kind, why)) = home_naming(home, keys) {
                    render_named(&mut out, named, &entry, &kind, why);
                }
            }
        }
    }
    render_rows(&mut out, &reach.moves, "homes");
    out
}

/// Rewrites `contract-homes.toml` from the walk; run on purpose, with every
/// feature, as `REGENERATE_HOMES` spells it.
#[test]
#[ignore = "rewrites contract-homes.toml; run on purpose"]
fn regenerate_contract_homes() -> Result<(), String> {
    let reach = reach_homes()?;
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/contract-homes.toml");
    std::fs::write(path, render_contract_homes(&reach)).map_err(|e| format!("write {path}: {e}"))
}

/// The 0.8.0 half of M1-T7-03: every colour home of the 0.8.0 model moves a
/// row of the 0.8.0 path, or a table says why it does not.
///
/// A home is stated in two opaque colours and a translucent one in turn on
/// each platform preset in both schemes ([`reach_homes`]); it moves a row in
/// a status where the row's text tells two of them apart. Every home that
/// moves no row must be named, by [`UNREAD_HOMES`] or by the 0.7.2 tables'
/// entry for a leaf it holds, and so must every home no patch reaches, by
/// the 0.7.2 tables or an [`UNREAD_HOMES`] entry of the kind "no patch". An
/// [`UNREAD_HOMES`] entry that names a home a row reads, or "no patch" for a
/// home a patch reaches, is stale, and so is one that names no colour home.
///
/// And `contract-homes.toml`, the walk's export, must be what the walk
/// finds: a file that differs fails, naming the command that rewrites it.
/// The test exists only with `--features iced_aw`, and compares the file
/// only with `spinner` as well, the default feature the `Spinner` rows need.
#[test]
fn every_colour_home_moves_a_row_or_is_named() -> Result<(), String> {
    let started = std::time::Instant::now();
    let reach = reach_homes()?;
    let mut failures = Vec::new();
    let (mut moving, mut named) = (0, 0);
    for (home, keys) in &reach.homes {
        let entry = UNREAD_HOMES.iter().find(|e| e.home == home.as_str());
        let listed = entry.is_some();
        let unpatched = reach.unpatched.get(home);
        match (unpatched, entry.map(|e| e.kind)) {
            (Some(label), Some(kind)) if kind != "no patch" => failures.push(format!(
                "{home}: no patch states a colour there (on {label}), yet UNREAD_HOMES \
                 names it {kind}"
            )),
            (Some(label), None) if home_naming(home, keys).is_none() => failures.push(format!(
                "{home}: no patch states a colour there (on {label}), and no table names it"
            )),
            (None, Some("no patch")) => failures.push(format!(
                "{home}: UNREAD_HOMES says no patch states a colour there, yet one does"
            )),
            _ => {}
        }
        if unpatched.is_some() {
            named += 1;
            continue;
        }
        match (reach.moves.get(home), home_naming(home, keys)) {
            (Some(rows), _) => {
                moving += 1;
                if listed {
                    failures.push(format!(
                        "{home}: moves {} yet UNREAD_HOMES names it",
                        rows.keys()
                            .map(Reached::to_string)
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
            }
            (None, Some(_)) => named += 1,
            (None, None) => failures.push(format!(
                "{home} (holding {}): moves no row, and no table names it",
                if keys.is_empty() {
                    "no 0.7.2 leaf".to_owned()
                } else {
                    keys.join(", ")
                }
            )),
        }
    }
    for entry in UNREAD_HOMES {
        if !reach.homes.contains_key(entry.home) {
            failures.push(format!(
                "UNREAD_HOMES names {}, which is no colour home",
                entry.home
            ));
        }
    }
    #[cfg(feature = "spinner")]
    if CONTRACT_HOMES != render_contract_homes(&reach) {
        failures.push(format!(
            "contract-homes.toml differs from what the colour-home walk finds; run \
             `{REGENERATE_HOMES}`"
        ));
    }
    println!(
        "{} colour homes: {moving} move a row, {named} named; {} rows; {:.2?}",
        reach.homes.len(),
        reach
            .moves
            .values()
            .flat_map(BTreeMap::keys)
            .map(|k| &k.row)
            .collect::<BTreeSet<_>>()
            .len(),
        started.elapsed()
    );
    if failures.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{} colour-home classification(s) do not hold:\n{}",
            failures.len(),
            failures.join("\n")
        ))
    }
}
