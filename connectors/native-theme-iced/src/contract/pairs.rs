//! The declared contrast pairs: which two colors of each function the report
//! prints and the assertion compares, and the palette pairs beside them.
//!
//! Test-only, like the rest of `contract`. The row kinds these consts are
//! written in, the `check_*` helper that runs them and the tests that call it
//! live in the parent module.

use super::*;

/// The palette pairs the report covers. Printed, never asserted (section 7).
pub(super) const PAIRS: &[ReportPair] = &[
    ReportPair {
        what: "placeholder",
        // iced paints a text input on the window background, whatever the
        // platform's field color is.
        emitted: |t| {
            let e = t.extended_palette();
            let window = e.background.base.color;
            (e.secondary.base.color, window, window)
        },
        native: |r| {
            (
                r.input.placeholder_color,
                r.input.background_color,
                r.defaults.background_color,
            )
        },
    },
    ReportPair {
        // The label a hovered `button::secondary` keeps, on the fill it swaps
        // in (`iced_widget::button::secondary`).
        what: "secondary.hover",
        emitted: |t| {
            let e = t.extended_palette();
            (
                e.secondary.base.text,
                e.secondary.strong.color,
                e.background.base.color,
            )
        },
        native: |r| {
            (
                r.button.font.color,
                r.button.hover_background,
                r.button.background_color,
            )
        },
    },
    ReportPair {
        what: "background.weak",
        emitted: |t| {
            let e = t.extended_palette();
            (
                e.background.weak.text,
                e.background.weak.color,
                e.background.base.color,
            )
        },
        native: |r| {
            (
                r.defaults.text_color,
                r.defaults.surface_color,
                r.defaults.background_color,
            )
        },
    },
    ReportPair {
        what: "primary.base",
        emitted: |t| {
            let e = t.extended_palette();
            (
                e.primary.base.text,
                e.primary.base.color,
                e.background.base.color,
            )
        },
        native: |r| {
            (
                r.defaults.accent_text_color,
                r.defaults.accent_color,
                r.defaults.background_color,
            )
        },
    },
    ReportPair {
        what: "success.base",
        emitted: |t| {
            let e = t.extended_palette();
            (
                e.success.base.text,
                e.success.base.color,
                e.background.base.color,
            )
        },
        native: |r| {
            (
                r.defaults.success_text_color,
                r.defaults.success_color,
                r.defaults.background_color,
            )
        },
    },
    ReportPair {
        what: "danger.base",
        emitted: |t| {
            let e = t.extended_palette();
            (
                e.danger.base.text,
                e.danger.base.color,
                e.background.base.color,
            )
        },
        native: |r| {
            (
                r.defaults.danger_text_color,
                r.defaults.danger_color,
                r.defaults.background_color,
            )
        },
    },
    ReportPair {
        what: "warning.base",
        emitted: |t| {
            let e = t.extended_palette();
            (
                e.warning.base.text,
                e.warning.base.color,
                e.background.base.color,
            )
        },
        native: |r| {
            (
                r.defaults.warning_text_color,
                r.defaults.warning_color,
                r.defaults.background_color,
            )
        },
    },
];

/// The `styles::button` pairs the assertion covers.
#[cfg(feature = "widgets")]
pub(super) const BUTTON_PAIRS: &[StylePair<button::Status>] = &[StylePair {
    what: "button label",
    indicator: false,
    statuses: BUTTON_STATUSES,
    emitted: |t, r, s| {
        let style = styles::button(r)(t, s);
        // A button is painted on the window, so that is what a translucent
        // fill shows through to.
        flat(style.background).map(|fill| {
            (
                style.text_color,
                fill,
                to_color(r.defaults.background_color),
            )
        })
    },
    native: |r, s| {
        (
            native_button_label(r, s),
            native_button_fill(r, s),
            to_color(r.defaults.background_color),
        )
    },
}];

// The four class buttons' pairs -- `BUTTON_PRIMARY_PAIRS`,
// `BUTTON_DANGER_PAIRS`, `BUTTON_SUCCESS_PAIRS`, `BUTTON_WARNING_PAIRS` --
// are declared by `class_button_contract!` in the parent module, beside their
// rows.

/// The `styles::button_link` pairs the assertion covers.
#[cfg(feature = "widgets")]
pub(super) const BUTTON_LINK_PAIRS: &[StylePair<button::Status>] = &[StylePair {
    what: "link button label",
    indicator: false,
    statuses: BUTTON_STATUSES,
    emitted: |t, r, s| {
        let style = styles::button_link(r)(t, s);
        flat(style.background).map(|fill| {
            (
                style.text_color,
                fill,
                to_color(r.defaults.background_color),
            )
        })
    },
    native: |r, s| {
        (
            native_button_link_label(r, s),
            native_button_link_fill(r, s),
            to_color(r.defaults.background_color),
        )
    },
}];

/// The `styles::text_input` pairs the assertion covers: both texts the
/// function paints, on the fill it paints them on.
#[cfg(feature = "widgets")]
pub(super) const TEXT_INPUT_PAIRS: &[StylePair<text_input::Status>] = &[
    StylePair {
        what: "input value",
        indicator: false,
        statuses: TEXT_INPUT_STATUSES,
        emitted: |t, r, s| {
            let style = styles::text_input(r)(t, s);
            // A field is painted on the window, so that is what a translucent
            // fill shows through to.
            fill(style.background).map(|background| {
                (
                    style.value,
                    background,
                    to_color(r.defaults.background_color),
                )
            })
        },
        native: |r, s| {
            (
                native_text_input_value(r, s),
                native_text_input_fill(r, s),
                to_color(r.defaults.background_color),
            )
        },
    },
    StylePair {
        what: "input placeholder",
        indicator: false,
        statuses: TEXT_INPUT_STATUSES,
        emitted: |t, r, s| {
            let style = styles::text_input(r)(t, s);
            fill(style.background).map(|background| {
                (
                    style.placeholder,
                    background,
                    to_color(r.defaults.background_color),
                )
            })
        },
        native: |r, s| {
            (
                to_color(r.input.placeholder_color),
                native_text_input_fill(r, s),
                to_color(r.defaults.background_color),
            )
        },
    },
];

/// The `styles::text_editor` pairs the assertion covers.
#[cfg(feature = "widgets")]
pub(super) const TEXT_EDITOR_PAIRS: &[StylePair<text_editor::Status>] = &[
    StylePair {
        what: "editor value",
        indicator: false,
        statuses: TEXT_EDITOR_STATUSES,
        emitted: |t, r, s| {
            let style = styles::text_editor(r)(t, s);
            fill(style.background).map(|background| {
                (
                    style.value,
                    background,
                    to_color(r.defaults.background_color),
                )
            })
        },
        native: |r, s| {
            (
                native_text_editor_value(r, s),
                native_text_editor_fill(r, s),
                to_color(r.defaults.background_color),
            )
        },
    },
    StylePair {
        what: "editor placeholder",
        indicator: false,
        statuses: TEXT_EDITOR_STATUSES,
        emitted: |t, r, s| {
            let style = styles::text_editor(r)(t, s);
            fill(style.background).map(|background| {
                (
                    style.placeholder,
                    background,
                    to_color(r.defaults.background_color),
                )
            })
        },
        native: |r, s| {
            (
                to_color(r.input.placeholder_color),
                native_text_editor_fill(r, s),
                to_color(r.defaults.background_color),
            )
        },
    },
];

/// The check mark on the box it is painted in, and the dot on its circle.
///
/// Both are indicators rather than text: WCAG asks 3:1 of one, not AA's 4.5:1,
/// so their below-AA lines are informational. What section 7 asserts of them
/// is the same as everywhere else -- that our ratio is not worse than the
/// platform's own pair.
///
/// The *label* pair is a text pair, and the only one of these functions where
/// the fill is not the function's own: a checkbox label sits beside the box,
/// on the window. Both colors are still native and both are still pinned --
/// the foreground by the row above, the window by the `palette.background`
/// row -- so section 7's rule has two native colors to compare, which is what
/// it asks for. It matters most in the disabled status, where the platform
/// dims the label on purpose.
#[cfg(feature = "widgets")]
pub(super) const CHECKBOX_PAIRS: &[StylePair<checkbox::Status>] = &[
    StylePair {
        what: "checkbox mark",
        indicator: true,
        statuses: CHECKBOX_CHECKED_STATUSES,
        emitted: |t, r, s| {
            let style = styles::checkbox(r)(t, s);
            // The box is painted on the window, so that is what a translucent
            // fill -- windows-11 states one -- shows through to.
            fill(style.background).map(|background| {
                (
                    style.icon_color,
                    background,
                    to_color(r.defaults.background_color),
                )
            })
        },
        native: |r, s| {
            (
                native_checkbox_mark(r, s),
                native_checkbox_fill(r, s),
                to_color(r.defaults.background_color),
            )
        },
    },
    StylePair {
        what: "checkbox label",
        indicator: false,
        statuses: CHECKBOX_STATUSES,
        // The fill is read from the *theme* the connector built, not from the
        // resolved field it was built from: otherwise both sides of the pair
        // would be the same expression and the fill half could not fail.
        // `palette.background`'s own row is what ties the two together.
        emitted: |t, r, s| {
            let window = t.extended_palette().background.base.color;
            stated(styles::checkbox(r)(t, s).text_color).map(|label| (label, window, window))
        },
        native: |r, s| {
            let window = to_color(r.defaults.background_color);
            (native_checkbox_label(r, s), window, window)
        },
    },
];

#[cfg(feature = "widgets")]
pub(super) const RADIO_PAIRS: &[StylePair<radio::Status>] = &[StylePair {
    what: "radio dot",
    indicator: true,
    statuses: RADIO_SELECTED_STATUSES,
    emitted: |t, r, s| {
        let style = styles::radio(r)(t, s);
        fill(style.background).map(|background| {
            (
                style.dot_color,
                background,
                to_color(r.defaults.background_color),
            )
        })
    },
    native: |r, s| {
        (
            to_color(r.checkbox.indicator_color),
            native_radio_fill(r, s),
            to_color(r.defaults.background_color),
        )
    },
}];

/// The thumb on the track it slides along -- an indicator pair again, and the
/// only one `toggler::Style` offers: its label is painted on the window.
#[cfg(feature = "widgets")]
pub(super) const TOGGLER_PAIRS: &[StylePair<toggler::Status>] = &[StylePair {
    what: "toggler thumb",
    indicator: true,
    statuses: TOGGLER_STATUSES,
    emitted: |t, r, s| {
        let style = styles::toggler(r)(t, s);
        let thumb = fill(style.foreground)?;
        fill(style.background).map(|track| (thumb, track, to_color(r.defaults.background_color)))
    },
    native: |r, s| {
        (
            native_toggler_thumb(r, s),
            native_toggler_track(r, s),
            to_color(r.defaults.background_color),
        )
    },
}];

/// Both texts a drop-down field paints, on the fill it paints them on.
#[cfg(feature = "widgets")]
pub(super) const PICK_LIST_PAIRS: &[StylePair<pick_list::Status>] = &[
    StylePair {
        what: "pick list label",
        indicator: false,
        statuses: PICK_LIST_STATUSES,
        emitted: |t, r, s| {
            let style = styles::pick_list(r)(t, s);
            fill(style.background).map(|background| {
                (
                    style.text_color,
                    background,
                    to_color(r.defaults.background_color),
                )
            })
        },
        native: |r, s| {
            (
                to_color(r.combo_box.font.color),
                native_pick_list_fill(r, s),
                to_color(r.defaults.background_color),
            )
        },
    },
    StylePair {
        what: "pick list placeholder",
        indicator: false,
        statuses: PICK_LIST_STATUSES,
        emitted: |t, r, s| {
            let style = styles::pick_list(r)(t, s);
            fill(style.background).map(|background| {
                (
                    style.placeholder_color,
                    background,
                    to_color(r.defaults.background_color),
                )
            })
        },
        native: |r, s| {
            (
                to_color(r.input.placeholder_color),
                native_pick_list_fill(r, s),
                to_color(r.defaults.background_color),
            )
        },
    },
];

/// A menu's label on its panel, and a selected row's label on the highlight.
/// The highlight is painted on the panel, so that -- not the window -- is the
/// surface a translucent one shows through to.
#[cfg(feature = "widgets")]
pub(super) const MENU_PAIRS: &[StylePair<()>] = &[
    StylePair {
        what: "menu label",
        indicator: false,
        statuses: MENU_STATUSES,
        emitted: |t, r, ()| {
            let style = styles::menu(r)(t);
            fill(style.background).map(|background| {
                (
                    style.text_color,
                    background,
                    to_color(r.defaults.background_color),
                )
            })
        },
        native: |r, ()| {
            (
                to_color(r.menu.font.color),
                to_color(r.menu.background_color),
                to_color(r.defaults.background_color),
            )
        },
    },
    StylePair {
        what: "menu selected label",
        indicator: false,
        statuses: MENU_STATUSES,
        emitted: |t, r, ()| {
            let style = styles::menu(r)(t);
            fill(style.selected_background).map(|highlight| {
                (
                    style.selected_text_color,
                    highlight,
                    to_color(r.menu.background_color),
                )
            })
        },
        native: |r, ()| {
            (
                to_color(r.menu.hover_text_color),
                to_color(r.menu.hover_background),
                to_color(r.menu.background_color),
            )
        },
    },
];

/// The handle on each half of the rail it slides along -- the handle sits over
/// the boundary between them, and both halves are the function's own. An
/// indicator pair, like the checkbox mark, and only for the statuses whose
/// handle *fill* the model states: a dragged handle's fill is iced's.
#[cfg(feature = "widgets")]
pub(super) const SLIDER_PAIRS: &[StylePair<slider::Status>] = &[
    StylePair {
        what: "slider handle on the filled rail",
        indicator: true,
        statuses: SLIDER_NATIVE_FILL_STATUSES,
        emitted: |t, r, s| {
            let style = styles::slider(r)(t, s);
            let handle = fill(style.handle.background)?;
            fill(style.rail.backgrounds.0)
                .map(|rail| (handle, rail, to_color(r.defaults.background_color)))
        },
        native: |r, s| {
            (
                native_slider_handle(r, s),
                to_color(r.slider.fill_color),
                to_color(r.defaults.background_color),
            )
        },
    },
    StylePair {
        what: "slider handle on the remaining rail",
        indicator: true,
        statuses: SLIDER_NATIVE_FILL_STATUSES,
        emitted: |t, r, s| {
            let style = styles::slider(r)(t, s);
            let handle = fill(style.handle.background)?;
            fill(style.rail.backgrounds.1)
                .map(|rail| (handle, rail, to_color(r.defaults.background_color)))
        },
        native: |r, s| {
            (
                native_slider_handle(r, s),
                to_color(r.slider.track_color),
                to_color(r.defaults.background_color),
            )
        },
    },
];

/// Each scroller on the rail it slides along -- both are the function's own,
/// in every status. An indicator pair, like the slider's handle: a scroller is
/// a shape, not text.
///
/// The surface under the rail is the window: iced draws a scrollbar over the
/// scrolled content, whose color is the application's, so the window is the
/// nearest thing the connector states -- the same choice every other pair
/// makes for a widget that floats over content.
#[cfg(feature = "widgets")]
pub(super) const SCROLLABLE_PAIRS: &[StylePair<scrollable::Status>] = &[
    StylePair {
        what: "scrollable vertical scroller on its rail",
        indicator: true,
        statuses: SCROLLABLE_STATUSES,
        emitted: |t, r, s| {
            let style = styles::scrollable(r)(t, s);
            let scroller = fill(style.vertical_rail.scroller.background)?;
            flat(style.vertical_rail.background)
                .map(|rail| (scroller, rail, to_color(r.defaults.background_color)))
        },
        native: |r, s| {
            (
                native_scroller(r, s, true),
                to_color(r.scrollbar.track_color),
                to_color(r.defaults.background_color),
            )
        },
    },
    StylePair {
        what: "scrollable horizontal scroller on its rail",
        indicator: true,
        statuses: SCROLLABLE_STATUSES,
        emitted: |t, r, s| {
            let style = styles::scrollable(r)(t, s);
            let scroller = fill(style.horizontal_rail.scroller.background)?;
            flat(style.horizontal_rail.background)
                .map(|rail| (scroller, rail, to_color(r.defaults.background_color)))
        },
        native: |r, s| {
            (
                native_scroller(r, s, false),
                to_color(r.scrollbar.track_color),
                to_color(r.defaults.background_color),
            )
        },
    },
];

/// The filled bar on the track it runs along. An indicator pair: the bar is a
/// shape, not text, and it is the one thing a progress bar says.
#[cfg(feature = "widgets")]
pub(super) const PROGRESS_BAR_PAIRS: &[StylePair<()>] = &[StylePair {
    what: "progress bar fill on its track",
    indicator: true,
    statuses: PROGRESS_BAR_STATUSES,
    emitted: |t, r, ()| {
        let style = styles::progress_bar(r)(t);
        let bar = fill(style.bar)?;
        fill(style.background).map(|track| (bar, track, to_color(r.defaults.background_color)))
    },
    native: |r, ()| {
        (
            to_color(r.progress_bar.fill_color),
            to_color(r.progress_bar.track_color),
            to_color(r.defaults.background_color),
        )
    },
}];

/// A tooltip's label on its panel. The panel floats over whatever is beneath
/// it, so a translucent one shows through to the window.
#[cfg(feature = "widgets")]
pub(super) const TOOLTIP_PAIRS: &[StylePair<()>] = &[StylePair {
    what: "tooltip label",
    indicator: false,
    statuses: TOOLTIP_STATUSES,
    emitted: |t, r, ()| {
        let style = styles::tooltip(r)(t);
        let label = stated(style.text_color)?;
        flat(style.background).map(|panel| (label, panel, to_color(r.defaults.background_color)))
    },
    native: |r, ()| {
        (
            to_color(r.tooltip.font.color),
            to_color(r.tooltip.background_color),
            to_color(r.defaults.background_color),
        )
    },
}];

// `styles::rule` and `styles::container_card` have no pair. A separator is one
// color with no fill of its own, and a card paints a fill but leaves the label
// on it to be inherited (`text_color: None`), so the connector does not
// control both sides of anything either one paints (section 7).

/// An `iced_aw` card states a label for each of its three sections and the
/// fill under it, so all three are pairs. The close icon is not: `iced_aw`
/// paints it with its default class's color, never the one this function
/// emits (`UNREACHABLE`, `styles::aw::card.close_color`).
#[cfg(feature = "iced_aw")]
pub(super) const AW_CARD_PAIRS: &[StylePair<AwStatus>] = &[
    StylePair {
        what: "aw card head label",
        indicator: false,
        statuses: AW_CARD_STATUSES,
        emitted: |t, r, s| {
            let style = styles::aw::card(r)(t, s);
            // A card floats on the window, so that is what a translucent fill
            // shows through to.
            fill(style.head_background).map(|head| {
                (
                    style.head_text_color,
                    head,
                    to_color(r.defaults.background_color),
                )
            })
        },
        native: |r, _| {
            (
                to_color(r.defaults.text_color),
                to_color(r.card.background_color),
                to_color(r.defaults.background_color),
            )
        },
    },
    StylePair {
        what: "aw card body label",
        indicator: false,
        statuses: AW_CARD_STATUSES,
        emitted: |t, r, s| {
            let style = styles::aw::card(r)(t, s);
            fill(style.body_background).map(|body| {
                (
                    style.body_text_color,
                    body,
                    to_color(r.defaults.background_color),
                )
            })
        },
        native: |r, _| {
            (
                to_color(r.defaults.text_color),
                to_color(r.card.background_color),
                to_color(r.defaults.background_color),
            )
        },
    },
    StylePair {
        what: "aw card foot label",
        indicator: false,
        statuses: AW_CARD_STATUSES,
        emitted: |t, r, s| {
            let style = styles::aw::card(r)(t, s);
            fill(style.foot_background).map(|foot| {
                (
                    style.foot_text_color,
                    foot,
                    to_color(r.defaults.background_color),
                )
            })
        },
        native: |r, _| {
            (
                to_color(r.defaults.text_color),
                to_color(r.card.background_color),
                to_color(r.defaults.background_color),
            )
        },
    },
];

// `styles::aw::menu` has no pair: `menu_bar::Style` states no text color at
// all -- an `iced_aw` menu's items are the consumer's own widgets -- so the
// connector controls one side of nothing it paints (section 7).

/// A tab's label on the tab, in the three statuses whose label this function
/// decides. The tab is painted over the strip, which this function also fills.
#[cfg(feature = "iced_aw")]
pub(super) const AW_TAB_BAR_PAIRS: &[StylePair<AwStatus>] = &[StylePair {
    what: "aw tab label",
    indicator: false,
    statuses: AW_TAB_BAR_NATIVE_STATUSES,
    emitted: |t, r, s| {
        let style = styles::aw::tab_bar(r)(t, s);
        fill(style.tab_label_background)
            .map(|tab| (style.text_color, tab, to_color(r.tab.bar_background)))
    },
    native: |r, s| {
        (
            native_aw_tab_label(r, s),
            native_aw_tab_fill(r, s),
            to_color(r.tab.bar_background),
        )
    },
}];

/// A sidebar item's label on the item, over the panel this function fills.
#[cfg(feature = "iced_aw")]
pub(super) const AW_SIDEBAR_PAIRS: &[StylePair<AwStatus>] = &[StylePair {
    what: "aw sidebar item label",
    indicator: false,
    statuses: AW_SIDEBAR_NATIVE_STATUSES,
    emitted: |t, r, s| {
        let style = styles::aw::sidebar(r)(t, s);
        fill(style.tab_label_background)
            .map(|item| (style.text_color, item, to_color(r.sidebar.background_color)))
    },
    native: |r, s| {
        (
            native_aw_sidebar_label(r, s),
            native_aw_sidebar_fill(r, s),
            to_color(r.sidebar.background_color),
        )
    },
}];

/// The spinner's circle on the window.
///
/// The connector states the circle's color and the window it is drawn on, so
/// both sides are ours; the fill is read from the *theme* rather than from the
/// resolved field it was built from, as the `checkbox label` pair's is, so
/// that half of the pair can fail. A spinner sits on whatever surface the
/// consumer puts it on, and the window is the one the connector can name.
#[cfg(feature = "iced_aw")]
pub(super) const AW_SPINNER_PAIRS: &[StylePair<()>] = &[StylePair {
    what: "aw spinner circle",
    indicator: true,
    statuses: AW_SPINNER_STATUSES,
    emitted: |t, r, ()| {
        let window = t.extended_palette().background.base.color;
        stated(styles::aw::spinner(r)(t).text_color).map(|arc| (arc, window, window))
    },
    native: |r, ()| {
        let window = to_color(r.defaults.background_color);
        (to_color(r.spinner.fill_color), window, window)
    },
}];

/// A selection list row's label on the row, over the list this function fills.
#[cfg(feature = "iced_aw")]
pub(super) const AW_SELECTION_LIST_PAIRS: &[StylePair<AwStatus>] = &[StylePair {
    what: "aw selection list row label",
    indicator: false,
    statuses: AW_SELECTION_LIST_NATIVE_STATUSES,
    emitted: |t, r, s| {
        let style = styles::aw::selection_list(r)(t, s);
        fill(style.background).map(|row| (style.text_color, row, to_color(r.list.background_color)))
    },
    native: |r, s| {
        (
            native_aw_list_label(r, s),
            native_aw_list_fill(r, s),
            to_color(r.list.background_color),
        )
    },
}];

/// The `what` of every contrast pair this file declares.
///
/// The coverage tripwire has nothing to say about pairs -- whether a
/// foreground and its fill are both ours is a judgment per function (section
/// 7) -- so there is no list that forces a pair to exist. What there is to
/// enforce is that a pair once written is actually run: one line here and one
/// `check_style_pairs` call are two hand-maintained lists, and
/// `style_contrast_never_degrades_the_native_pair` requires them to agree.
#[cfg(feature = "widgets")]
pub(super) fn style_pair_names() -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    out.extend(BUTTON_PAIRS.iter().map(|pair| pair.what.to_string()));
    out.extend(BUTTON_PRIMARY_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(BUTTON_DANGER_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(BUTTON_SUCCESS_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(BUTTON_WARNING_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(BUTTON_LINK_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(TEXT_INPUT_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(TEXT_EDITOR_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(CHECKBOX_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(RADIO_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(TOGGLER_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(PICK_LIST_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(MENU_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(SLIDER_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(SCROLLABLE_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(PROGRESS_BAR_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(TOOLTIP_PAIRS.iter().map(|p| p.what.to_string()));
    #[cfg(feature = "iced_aw")]
    {
        out.extend(AW_CARD_PAIRS.iter().map(|p| p.what.to_string()));
        out.extend(AW_TAB_BAR_PAIRS.iter().map(|p| p.what.to_string()));
        out.extend(AW_SIDEBAR_PAIRS.iter().map(|p| p.what.to_string()));
        out.extend(AW_SELECTION_LIST_PAIRS.iter().map(|p| p.what.to_string()));
        out.extend(AW_SPINNER_PAIRS.iter().map(|p| p.what.to_string()));
    }
    out
}
