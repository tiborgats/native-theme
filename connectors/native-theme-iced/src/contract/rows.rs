//! The declared rows: every palette slot and every `styles::*` field the
//! contract maps, the native value behind each, and the status lists the rows
//! hold for.
//!
//! Test-only, like the rest of `contract`. The row kinds, the `check_*`
//! helpers that run these rows and the tests that call them live in the parent
//! module; what lives here is the data they read. The `native_*` helpers sit
//! beside the rows because both the rows and the contrast pairs compute their
//! expected values with them.

use super::*;

/// Every palette slot the connector writes, and the native field it carries.
///
/// The six base-palette slots come from `palette::to_palette()`, the
/// `extended.*` slots from `extended::apply_overrides()`. The `styles::*`
/// fields are in `STYLE_ROWS` and `SCALAR_ROWS`.
pub(super) const ROWS: &[Row] = &[
    Row {
        slot: "palette.background",
        native: |r| r.defaults.background_color,
        get: |t, _| t.palette().background,
        exceptions: &[],
    },
    Row {
        slot: "palette.text",
        native: |r| r.defaults.text_color,
        get: |t, _| t.palette().text,
        exceptions: &[],
    },
    Row {
        slot: "palette.primary",
        native: |r| r.defaults.accent_color,
        get: |t, _| t.palette().primary,
        exceptions: &[],
    },
    Row {
        slot: "palette.success",
        native: |r| r.defaults.success_color,
        get: |t, _| t.palette().success,
        exceptions: &[],
    },
    Row {
        slot: "palette.warning",
        native: |r| r.defaults.warning_color,
        get: |t, _| t.palette().warning,
        exceptions: &[],
    },
    Row {
        slot: "palette.danger",
        native: |r| r.defaults.danger_color,
        get: |t, _| t.palette().danger,
        exceptions: &[],
    },
    Row {
        slot: "extended.secondary.base.color",
        native: |r| r.input.placeholder_color,
        get: |t, _| t.extended_palette().secondary.base.color,
        exceptions: &[],
    },
    Row {
        // A hovered `button::secondary` keeps the base label and swaps only
        // the fill, so the fill is a copy of the base.
        slot: "extended.secondary.strong.color",
        native: |r| r.input.placeholder_color,
        get: |t, _| t.extended_palette().secondary.strong.color,
        exceptions: &[],
    },
    Row {
        slot: "extended.background.weak.color",
        native: |r| r.defaults.surface_color,
        get: |t, _| t.extended_palette().background.weak.color,
        exceptions: &[],
    },
    Row {
        slot: "extended.background.weak.text",
        native: |r| r.defaults.text_color,
        get: |t, _| t.extended_palette().background.weak.text,
        exceptions: &[],
    },
    Row {
        slot: "extended.primary.base.text",
        native: |r| r.defaults.accent_text_color,
        get: |t, _| t.extended_palette().primary.base.text,
        exceptions: &[],
    },
    Row {
        slot: "extended.success.base.text",
        native: |r| r.defaults.success_text_color,
        get: |t, _| t.extended_palette().success.base.text,
        exceptions: &[],
    },
    Row {
        slot: "extended.danger.base.text",
        native: |r| r.defaults.danger_text_color,
        get: |t, _| t.extended_palette().danger.base.text,
        exceptions: &[],
    },
    Row {
        slot: "extended.warning.base.text",
        native: |r| r.defaults.warning_text_color,
        get: |t, _| t.extended_palette().warning.base.text,
        exceptions: &[],
    },
];

/// Every `button::Status`, as values rather than names.
///
/// Each function declares its own such list, of its own widget's `Status`. A
/// shape-B or shape-C function, which takes no status, uses `&[()]`.
#[cfg(feature = "widgets")]
pub(super) const BUTTON_STATUSES: &[button::Status] = &[
    button::Status::Active,
    button::Status::Hovered,
    button::Status::Pressed,
    button::Status::Disabled,
];

/// The fill the native fields give a button in `status`, computed here from
/// the resolved theme alone -- never from the code under test.
///
/// The match has no catch-all arm, so a status added upstream fails to compile
/// in the contract as well as in `styles`.
#[cfg(feature = "widgets")]
pub(super) fn native_button_fill(r: &ResolvedTheme, status: button::Status) -> Color {
    let base = to_color(r.button.background_color);
    match status {
        button::Status::Active => base,
        // The platform layers its hover and pressed fills over the button's
        // own fill; iced replaces, so the layer is composited first (C17).
        button::Status::Hovered => over(to_color(r.button.hover_background), base),
        // `active_background` is a soft option: `None` copies the hover fill.
        button::Status::Pressed => over(
            to_color(
                r.button
                    .active_background
                    .unwrap_or(r.button.hover_background),
            ),
            base,
        ),
        // A translucent disabled fill replaces the idle one and lets the
        // window through, so it is emitted as given.
        button::Status::Disabled => to_color(
            r.button
                .disabled_background
                .unwrap_or(r.button.background_color),
        ),
    }
}

/// The label color the native fields give a button in `status`.
#[cfg(feature = "widgets")]
pub(super) fn native_button_label(r: &ResolvedTheme, status: button::Status) -> Color {
    to_color(match status {
        button::Status::Active => r.button.font.color,
        button::Status::Hovered => r.button.hover_text_color,
        button::Status::Pressed => r.button.active_text_color,
        button::Status::Disabled => r.button.disabled_text_color,
    })
}

/// The border every button function wears: the button's own, in every status.
#[cfg(feature = "widgets")]
pub(super) fn native_button_border(r: &ResolvedTheme) -> NativeBorder {
    NativeBorder {
        color: to_color(r.button.border.color),
        width: r.button.border.line_width,
        radius: r.button.border.corner_radius,
    }
}

/// Every color field of `styles::button`, and the native value it carries.
///
/// One such const per function; read with `BUTTON_BORDER_ROWS` and `DERIVED`
/// together, they name every field of the `Style` it emits.
#[cfg(feature = "widgets")]
pub(super) const BUTTON_ROWS: &[StyleRow<button::Status>] = &[
    StyleRow {
        field: "styles::button.background",
        statuses: BUTTON_STATUSES,
        native: native_button_fill,
        get: |t, r, s| flat(styles::button(r)(t, s).background),
    },
    StyleRow {
        field: "styles::button.text_color",
        statuses: BUTTON_STATUSES,
        native: native_button_label,
        get: |t, r, s| Ok(styles::button(r)(t, s).text_color),
    },
];

#[cfg(feature = "widgets")]
pub(super) const BUTTON_BORDER_ROWS: &[BorderRow<button::Status>] = &[BorderRow {
    field: "styles::button",
    statuses: BUTTON_STATUSES,
    native: |r, _| native_button_border(r),
    get: |t, r, s| styles::button(r)(t, s).border,
}];

/// The statuses of a class button whose fill and label the function itself
/// decides, and so the only ones its rows and its contrast pair cover.
#[cfg(feature = "widgets")]
pub(super) const CLASS_BUTTON_NATIVE_STATUSES: &[button::Status] =
    &[button::Status::Active, button::Status::Disabled];

/// The statuses a class button takes whole from iced's own class, because the
/// model states no hovered and no pressed variant of an accent or a status
/// color (spec section 3.3). Asserted by
/// `a_class_button_takes_its_hovered_and_pressed_states_from_iced`.
#[cfg(feature = "widgets")]
pub(super) const CLASS_BUTTON_ICED_STATUSES: &[button::Status] =
    &[button::Status::Hovered, button::Status::Pressed];

/// The fill the native fields give a class button painted `idle`.
///
/// The match has no catch-all, but only the two statuses of
/// `CLASS_BUTTON_NATIVE_STATUSES` ever reach a row: the other two are iced's
/// on both sides. Their arm still has to name a color, and the idle one is the
/// least surprising -- if a status is ever moved between the two lists, the row
/// compares it against that and fails rather than passing quietly.
#[cfg(feature = "widgets")]
pub(super) fn native_class_button_fill(
    r: &ResolvedTheme,
    status: button::Status,
    idle: Rgba,
) -> Color {
    match status {
        button::Status::Active | button::Status::Hovered | button::Status::Pressed => {
            to_color(idle)
        }
        button::Status::Disabled => to_color(
            r.button
                .disabled_background
                .unwrap_or(r.button.background_color),
        ),
    }
}

/// The label the native fields give a class button whose own label is `idle`.
#[cfg(feature = "widgets")]
pub(super) fn native_class_button_label(
    r: &ResolvedTheme,
    status: button::Status,
    idle: Rgba,
) -> Color {
    to_color(match status {
        button::Status::Active | button::Status::Hovered | button::Status::Pressed => idle,
        button::Status::Disabled => r.button.disabled_text_color,
    })
}

/// The fill the native fields give a link button in `status`.
#[cfg(feature = "widgets")]
pub(super) fn native_button_link_fill(r: &ResolvedTheme, status: button::Status) -> Color {
    let base = to_color(r.link.background_color);
    match status {
        // `LinkTheme` states no pressed and no disabled fill, so both keep the
        // idle one.
        button::Status::Active | button::Status::Pressed | button::Status::Disabled => base,
        button::Status::Hovered => over(to_color(r.link.hover_background), base),
    }
}

/// The label color the native fields give a link button in `status`.
#[cfg(feature = "widgets")]
pub(super) fn native_button_link_label(r: &ResolvedTheme, status: button::Status) -> Color {
    to_color(match status {
        button::Status::Active => r.link.font.color,
        button::Status::Hovered => r.link.hover_text_color,
        button::Status::Pressed => r.link.active_text_color,
        button::Status::Disabled => r.link.disabled_text_color,
    })
}

/// Every color field of `styles::button_link`.
///
/// It has no scalar rows: `LinkTheme` carries no border, so all six border
/// leaves are iced's own and sit in `DERIVED`.
#[cfg(feature = "widgets")]
pub(super) const BUTTON_LINK_ROWS: &[StyleRow<button::Status>] = &[
    StyleRow {
        field: "styles::button_link.background",
        statuses: BUTTON_STATUSES,
        native: native_button_link_fill,
        get: |t, r, s| flat(styles::button_link(r)(t, s).background),
    },
    StyleRow {
        field: "styles::button_link.text_color",
        statuses: BUTTON_STATUSES,
        native: native_button_link_label,
        get: |t, r, s| Ok(styles::button_link(r)(t, s).text_color),
    },
];

/// Every value of `text_input::Status`, not every variant: `Focused` carries a
/// `bool`, so it stands for two statuses and both are listed
/// (`text_input.rs:1697-1709`).
#[cfg(feature = "widgets")]
pub(super) const TEXT_INPUT_STATUSES: &[text_input::Status] = &[
    text_input::Status::Active,
    text_input::Status::Hovered,
    text_input::Status::Focused { is_hovered: false },
    text_input::Status::Focused { is_hovered: true },
    text_input::Status::Disabled,
];

/// The fill the native fields give a text input in `status`.
#[cfg(feature = "widgets")]
fn native_input_fill(r: &ResolvedTheme, disabled: bool) -> Color {
    if disabled {
        // A translucent disabled fill replaces the idle one, so it is emitted
        // as given.
        to_color(
            r.input
                .disabled_background
                .unwrap_or(r.input.background_color),
        )
    } else {
        to_color(r.input.background_color)
    }
}

/// The border color the native fields give a text input, by state.
///
/// Both soft options copy the border's own color, the base-state value of
/// spec section 3.2's table.
#[cfg(feature = "widgets")]
fn native_input_border(r: &ResolvedTheme, hovered: bool, focused: bool) -> Color {
    let i = &r.input;
    to_color(if focused {
        i.focus_border_color.unwrap_or(i.border.color)
    } else if hovered {
        i.hover_border_color.unwrap_or(i.border.color)
    } else {
        i.border.color
    })
}

/// The text color the native fields give a text input's value, by state.
#[cfg(feature = "widgets")]
fn native_input_value(r: &ResolvedTheme, disabled: bool) -> Color {
    to_color(if disabled {
        r.input.disabled_text_color
    } else {
        r.input.font.color
    })
}

/// The three `input.*` values a `text_input::Status` selects, each matched
/// exhaustively with no catch-all so an upstream variant fails to compile here.
#[cfg(feature = "widgets")]
pub(super) fn native_text_input_fill(r: &ResolvedTheme, status: text_input::Status) -> Color {
    match status {
        text_input::Status::Active
        | text_input::Status::Hovered
        | text_input::Status::Focused { is_hovered: _ } => native_input_fill(r, false),
        text_input::Status::Disabled => native_input_fill(r, true),
    }
}

#[cfg(feature = "widgets")]
fn native_text_input_border(r: &ResolvedTheme, status: text_input::Status) -> Color {
    match status {
        text_input::Status::Active | text_input::Status::Disabled => {
            native_input_border(r, false, false)
        }
        text_input::Status::Hovered => native_input_border(r, true, false),
        text_input::Status::Focused { is_hovered: _ } => native_input_border(r, false, true),
    }
}

#[cfg(feature = "widgets")]
pub(super) fn native_text_input_value(r: &ResolvedTheme, status: text_input::Status) -> Color {
    match status {
        text_input::Status::Active
        | text_input::Status::Hovered
        | text_input::Status::Focused { is_hovered: _ } => native_input_value(r, false),
        text_input::Status::Disabled => native_input_value(r, true),
    }
}

/// Every color field of `styles::text_input`.
#[cfg(feature = "widgets")]
pub(super) const TEXT_INPUT_ROWS: &[StyleRow<text_input::Status>] = &[
    StyleRow {
        field: "styles::text_input.background",
        statuses: TEXT_INPUT_STATUSES,
        native: native_text_input_fill,
        get: |t, r, s| fill(styles::text_input(r)(t, s).background),
    },
    StyleRow {
        field: "styles::text_input.placeholder",
        statuses: TEXT_INPUT_STATUSES,
        native: |r, _| to_color(r.input.placeholder_color),
        get: |t, r, s| Ok(styles::text_input(r)(t, s).placeholder),
    },
    StyleRow {
        field: "styles::text_input.value",
        statuses: TEXT_INPUT_STATUSES,
        native: native_text_input_value,
        get: |t, r, s| Ok(styles::text_input(r)(t, s).value),
    },
    StyleRow {
        field: "styles::text_input.selection",
        statuses: TEXT_INPUT_STATUSES,
        native: |r, _| to_color(r.input.selection_background),
        get: |t, r, s| Ok(styles::text_input(r)(t, s).selection),
    },
];

/// The input's border: its width and radius are status-independent, its color
/// is not.
#[cfg(feature = "widgets")]
pub(super) const TEXT_INPUT_BORDER_ROWS: &[BorderRow<text_input::Status>] = &[BorderRow {
    field: "styles::text_input",
    statuses: TEXT_INPUT_STATUSES,
    native: |r, s| NativeBorder {
        color: native_text_input_border(r, s),
        width: r.input.border.line_width,
        radius: r.input.border.corner_radius,
    },
    get: |t, r, s| styles::text_input(r)(t, s).border,
}];

/// Every value of `text_editor::Status`; `Focused` carries a `bool` there too
/// (`text_editor.rs:1409-1421`).
#[cfg(feature = "widgets")]
pub(super) const TEXT_EDITOR_STATUSES: &[text_editor::Status] = &[
    text_editor::Status::Active,
    text_editor::Status::Hovered,
    text_editor::Status::Focused { is_hovered: false },
    text_editor::Status::Focused { is_hovered: true },
    text_editor::Status::Disabled,
];

/// The same three `input.*` values, selected by a `text_editor::Status`. The
/// enum is a separate type from `text_input`'s, so the match is written again
/// rather than shared -- and again with no catch-all.
#[cfg(feature = "widgets")]
pub(super) fn native_text_editor_fill(r: &ResolvedTheme, status: text_editor::Status) -> Color {
    match status {
        text_editor::Status::Active
        | text_editor::Status::Hovered
        | text_editor::Status::Focused { is_hovered: _ } => native_input_fill(r, false),
        text_editor::Status::Disabled => native_input_fill(r, true),
    }
}

#[cfg(feature = "widgets")]
fn native_text_editor_border(r: &ResolvedTheme, status: text_editor::Status) -> Color {
    match status {
        text_editor::Status::Active | text_editor::Status::Disabled => {
            native_input_border(r, false, false)
        }
        text_editor::Status::Hovered => native_input_border(r, true, false),
        text_editor::Status::Focused { is_hovered: _ } => native_input_border(r, false, true),
    }
}

#[cfg(feature = "widgets")]
pub(super) fn native_text_editor_value(r: &ResolvedTheme, status: text_editor::Status) -> Color {
    match status {
        text_editor::Status::Active
        | text_editor::Status::Hovered
        | text_editor::Status::Focused { is_hovered: _ } => native_input_value(r, false),
        text_editor::Status::Disabled => native_input_value(r, true),
    }
}

/// Every color field of `styles::text_editor`. There is no `DERIVED` entry for
/// this function: `text_editor::Style` has no field the model cannot fill.
#[cfg(feature = "widgets")]
pub(super) const TEXT_EDITOR_ROWS: &[StyleRow<text_editor::Status>] = &[
    StyleRow {
        field: "styles::text_editor.background",
        statuses: TEXT_EDITOR_STATUSES,
        native: native_text_editor_fill,
        get: |t, r, s| fill(styles::text_editor(r)(t, s).background),
    },
    StyleRow {
        field: "styles::text_editor.placeholder",
        statuses: TEXT_EDITOR_STATUSES,
        native: |r, _| to_color(r.input.placeholder_color),
        get: |t, r, s| Ok(styles::text_editor(r)(t, s).placeholder),
    },
    StyleRow {
        field: "styles::text_editor.value",
        statuses: TEXT_EDITOR_STATUSES,
        native: native_text_editor_value,
        get: |t, r, s| Ok(styles::text_editor(r)(t, s).value),
    },
    StyleRow {
        field: "styles::text_editor.selection",
        statuses: TEXT_EDITOR_STATUSES,
        native: |r, _| to_color(r.input.selection_background),
        get: |t, r, s| Ok(styles::text_editor(r)(t, s).selection),
    },
];

/// The same border, through the editor's own `Status`.
#[cfg(feature = "widgets")]
pub(super) const TEXT_EDITOR_BORDER_ROWS: &[BorderRow<text_editor::Status>] = &[BorderRow {
    field: "styles::text_editor",
    statuses: TEXT_EDITOR_STATUSES,
    native: |r, s| NativeBorder {
        color: native_text_editor_border(r, s),
        width: r.input.border.line_width,
        radius: r.input.border.corner_radius,
    },
    get: |t, r, s| styles::text_editor(r)(t, s).border,
}];

/// Every value of `checkbox::Status`, not every variant: each of the three
/// carries whether the box is checked (`checkbox.rs:506-524`), so there are
/// six.
#[cfg(feature = "widgets")]
pub(super) const CHECKBOX_STATUSES: &[checkbox::Status] = &[
    checkbox::Status::Active { is_checked: false },
    checkbox::Status::Active { is_checked: true },
    checkbox::Status::Hovered { is_checked: false },
    checkbox::Status::Hovered { is_checked: true },
    checkbox::Status::Disabled { is_checked: false },
    checkbox::Status::Disabled { is_checked: true },
];

/// The three statuses in which the check mark is painted, and so the only ones
/// its contrast pair measures.
#[cfg(feature = "widgets")]
pub(super) const CHECKBOX_CHECKED_STATUSES: &[checkbox::Status] = &[
    checkbox::Status::Active { is_checked: true },
    checkbox::Status::Hovered { is_checked: true },
    checkbox::Status::Disabled { is_checked: true },
];

/// The other three. An unchecked box paints no mark, so the mark's color on
/// its fill is a ratio nothing shows; this list exists so that the two can be
/// required to partition the enum rather than the pair quietly covering half
/// of it.
#[cfg(feature = "widgets")]
pub(super) const CHECKBOX_UNCHECKED_STATUSES: &[checkbox::Status] = &[
    checkbox::Status::Active { is_checked: false },
    checkbox::Status::Hovered { is_checked: false },
    checkbox::Status::Disabled { is_checked: false },
];

/// The fill the native fields give a checkbox at rest, by whether it is
/// checked. `unchecked_background` is a soft option copying the checkbox's own
/// fill, and it is an idle fill, so it is emitted as given even where the
/// platform states it translucent -- windows-11 does, at `#0000000a`.
#[cfg(feature = "widgets")]
fn native_checkbox_idle(r: &ResolvedTheme, is_checked: bool) -> Color {
    let c = &r.checkbox;
    to_color(if is_checked {
        c.checked_background
    } else {
        c.unchecked_background.unwrap_or(c.background_color)
    })
}

/// The fill the native fields give a checkbox in `status`.
#[cfg(feature = "widgets")]
pub(super) fn native_checkbox_fill(r: &ResolvedTheme, status: checkbox::Status) -> Color {
    let c = &r.checkbox;
    match status {
        checkbox::Status::Active { is_checked } => native_checkbox_idle(r, is_checked),
        // The hover layer goes over whichever fill the box is showing (C17).
        checkbox::Status::Hovered { is_checked } => over(
            to_color(c.hover_background.unwrap_or(c.background_color)),
            native_checkbox_idle(r, is_checked),
        ),
        // One disabled fill for both, replacing the idle one, as given.
        checkbox::Status::Disabled { is_checked: _ } => {
            to_color(c.disabled_background.unwrap_or(c.background_color))
        }
    }
}

/// The outline the native fields give a checkbox or a radio button: an
/// unchecked one may state a color of its own, a checked one wears the
/// widget's border.
#[cfg(feature = "widgets")]
fn native_checkbox_outline(r: &ResolvedTheme, is_checked: bool) -> Color {
    let c = &r.checkbox;
    to_color(if is_checked {
        c.border.color
    } else {
        c.unchecked_border_color.unwrap_or(c.border.color)
    })
}

/// The label the native fields give a checkbox in `status`: the platform dims
/// a disabled one, and states the color it dims it to.
#[cfg(feature = "widgets")]
pub(super) fn native_checkbox_label(r: &ResolvedTheme, status: checkbox::Status) -> Color {
    let c = &r.checkbox;
    to_color(match status {
        checkbox::Status::Active { is_checked: _ }
        | checkbox::Status::Hovered { is_checked: _ } => c.font.color,
        checkbox::Status::Disabled { is_checked: _ } => c.disabled_text_color,
    })
}

#[cfg(feature = "widgets")]
fn native_checkbox_border(r: &ResolvedTheme, status: checkbox::Status) -> Color {
    match status {
        checkbox::Status::Active { is_checked }
        | checkbox::Status::Hovered { is_checked }
        | checkbox::Status::Disabled { is_checked } => native_checkbox_outline(r, is_checked),
    }
}

/// Every color field of `styles::checkbox`. There is no `DERIVED` entry for
/// this function: `checkbox::Style` has no field the model cannot fill.
#[cfg(feature = "widgets")]
pub(super) const CHECKBOX_ROWS: &[StyleRow<checkbox::Status>] = &[
    StyleRow {
        field: "styles::checkbox.background",
        statuses: CHECKBOX_STATUSES,
        native: native_checkbox_fill,
        get: |t, r, s| fill(styles::checkbox(r)(t, s).background),
    },
    StyleRow {
        field: "styles::checkbox.icon_color",
        statuses: CHECKBOX_STATUSES,
        native: |r, _| to_color(r.checkbox.indicator_color),
        get: |t, r, s| Ok(styles::checkbox(r)(t, s).icon_color),
    },
    StyleRow {
        field: "styles::checkbox.text_color",
        statuses: CHECKBOX_STATUSES,
        native: native_checkbox_label,
        get: |t, r, s| stated(styles::checkbox(r)(t, s).text_color),
    },
];

#[cfg(feature = "widgets")]
pub(super) const CHECKBOX_BORDER_ROWS: &[BorderRow<checkbox::Status>] = &[BorderRow {
    field: "styles::checkbox",
    statuses: CHECKBOX_STATUSES,
    native: |r, s| NativeBorder {
        color: native_checkbox_border(r, s),
        width: r.checkbox.border.line_width,
        radius: r.checkbox.border.corner_radius,
    },
    get: |t, r, s| styles::checkbox(r)(t, s).border,
}];

/// Every value of `radio::Status`: two variants carrying whether the button is
/// selected (`radio.rs:474-487`), so four. There is no disabled variant.
#[cfg(feature = "widgets")]
pub(super) const RADIO_STATUSES: &[radio::Status] = &[
    radio::Status::Active { is_selected: false },
    radio::Status::Active { is_selected: true },
    radio::Status::Hovered { is_selected: false },
    radio::Status::Hovered { is_selected: true },
];

/// The two statuses in which the dot is painted, and so the only ones its
/// contrast pair measures.
#[cfg(feature = "widgets")]
pub(super) const RADIO_SELECTED_STATUSES: &[radio::Status] = &[
    radio::Status::Active { is_selected: true },
    radio::Status::Hovered { is_selected: true },
];

/// The other two, for the same reason `CHECKBOX_UNCHECKED_STATUSES` exists.
#[cfg(feature = "widgets")]
pub(super) const RADIO_UNSELECTED_STATUSES: &[radio::Status] = &[
    radio::Status::Active { is_selected: false },
    radio::Status::Hovered { is_selected: false },
];

/// The fill the native fields give a radio button in `status`. The model
/// states one `CheckboxTheme` for both controls, so these are the checkbox's
/// own fills, selected by `is_selected`.
#[cfg(feature = "widgets")]
pub(super) fn native_radio_fill(r: &ResolvedTheme, status: radio::Status) -> Color {
    let c = &r.checkbox;
    match status {
        radio::Status::Active { is_selected } => native_checkbox_idle(r, is_selected),
        radio::Status::Hovered { is_selected } => over(
            to_color(c.hover_background.unwrap_or(c.background_color)),
            native_checkbox_idle(r, is_selected),
        ),
    }
}

#[cfg(feature = "widgets")]
fn native_radio_border(r: &ResolvedTheme, status: radio::Status) -> Color {
    match status {
        radio::Status::Active { is_selected } | radio::Status::Hovered { is_selected } => {
            native_checkbox_outline(r, is_selected)
        }
    }
}

/// Every color field of `styles::radio`. Nothing here is iced's either.
#[cfg(feature = "widgets")]
pub(super) const RADIO_ROWS: &[StyleRow<radio::Status>] = &[
    StyleRow {
        field: "styles::radio.background",
        statuses: RADIO_STATUSES,
        native: native_radio_fill,
        get: |t, r, s| fill(styles::radio(r)(t, s).background),
    },
    StyleRow {
        field: "styles::radio.dot_color",
        statuses: RADIO_STATUSES,
        native: |r, _| to_color(r.checkbox.indicator_color),
        get: |t, r, s| Ok(styles::radio(r)(t, s).dot_color),
    },
    StyleRow {
        field: "styles::radio.border_color",
        statuses: RADIO_STATUSES,
        native: native_radio_border,
        get: |t, r, s| Ok(styles::radio(r)(t, s).border_color),
    },
    StyleRow {
        field: "styles::radio.text_color",
        statuses: RADIO_STATUSES,
        native: |r, _| to_color(r.checkbox.font.color),
        get: |t, r, s| stated(styles::radio(r)(t, s).text_color),
    },
];

/// `radio::Style` states its border width on its own rather than inside an
/// iced `Border`, so it is a scalar row rather than part of a border row.
#[cfg(feature = "widgets")]
pub(super) const RADIO_SCALAR_ROWS: &[ScalarRow<radio::Status>] = &[ScalarRow {
    field: "styles::radio.border_width",
    statuses: RADIO_STATUSES,
    native: |_, r, _| r.checkbox.border.line_width,
    get: |t, r, s| Ok(styles::radio(r)(t, s).border_width),
}];

/// Every value of `toggler::Status`: three variants carrying whether the
/// switch is on (`toggler.rs:486-504`), so six.
#[cfg(feature = "widgets")]
pub(super) const TOGGLER_STATUSES: &[toggler::Status] = &[
    toggler::Status::Active { is_toggled: false },
    toggler::Status::Active { is_toggled: true },
    toggler::Status::Hovered { is_toggled: false },
    toggler::Status::Hovered { is_toggled: true },
    toggler::Status::Disabled { is_toggled: false },
    toggler::Status::Disabled { is_toggled: true },
];

/// The track the native fields give a switch at rest, by whether it is on.
#[cfg(feature = "widgets")]
fn native_toggler_idle(r: &ResolvedTheme, is_toggled: bool) -> Color {
    to_color(if is_toggled {
        r.switch.checked_background
    } else {
        r.switch.unchecked_background
    })
}

/// The track the native fields give a switch in `status`.
#[cfg(feature = "widgets")]
pub(super) fn native_toggler_track(r: &ResolvedTheme, status: toggler::Status) -> Color {
    let s = &r.switch;
    match status {
        toggler::Status::Active { is_toggled } => native_toggler_idle(r, is_toggled),
        // Each hover layer copies the track it covers when the platform
        // states none, and is composited over it (C17).
        toggler::Status::Hovered { is_toggled } => over(
            to_color(if is_toggled {
                s.hover_checked_background.unwrap_or(s.checked_background)
            } else {
                s.hover_unchecked_background
                    .unwrap_or(s.unchecked_background)
            }),
            native_toggler_idle(r, is_toggled),
        ),
        // A disabled track replaces the idle one, so it is as given.
        toggler::Status::Disabled { is_toggled } => to_color(if is_toggled {
            s.disabled_checked_background
                .unwrap_or(s.checked_background)
        } else {
            s.disabled_unchecked_background
                .unwrap_or(s.unchecked_background)
        }),
    }
}

/// The thumb the native fields give a switch in `status`. A thumb is painted
/// over a track the widget also paints, so it is emitted as given.
#[cfg(feature = "widgets")]
pub(super) fn native_toggler_thumb(r: &ResolvedTheme, status: toggler::Status) -> Color {
    let s = &r.switch;
    to_color(match status {
        toggler::Status::Active { is_toggled: _ } | toggler::Status::Hovered { is_toggled: _ } => {
            s.thumb_background
        }
        toggler::Status::Disabled { is_toggled: _ } => {
            s.disabled_thumb_color.unwrap_or(s.thumb_background)
        }
    })
}

/// Every color field of `styles::toggler`. Five of the remaining seven fields
/// are in `DERIVED`: `SwitchTheme` carries no border and no font. The other two,
/// `border_radius` and `padding_ratio`, are native (`switch.track_radius`, and
/// the inset the track and thumb heights state between them).
#[cfg(feature = "widgets")]
pub(super) const TOGGLER_ROWS: &[StyleRow<toggler::Status>] = &[
    StyleRow {
        field: "styles::toggler.background",
        statuses: TOGGLER_STATUSES,
        native: native_toggler_track,
        get: |t, r, s| fill(styles::toggler(r)(t, s).background),
    },
    StyleRow {
        field: "styles::toggler.foreground",
        statuses: TOGGLER_STATUSES,
        native: native_toggler_thumb,
        get: |t, r, s| fill(styles::toggler(r)(t, s).foreground),
    },
];

/// Whether the model's switch geometry can state the inset iced asks for: a
/// track with a height, and a thumb that fits inside it.
///
/// Written here as well as in `styles::toggler`, because the contract's side
/// of a guarded value has to be computed independently of the code under test.
#[cfg(feature = "widgets")]
pub(super) fn switch_geometry_is_usable(r: &ResolvedTheme) -> bool {
    let s = &r.switch;
    s.track_height > 0.0 && s.thumb_diameter >= 0.0 && s.thumb_diameter <= s.track_height
}

/// The inset the native fields state between the track and the thumb, as the
/// ratio iced's receiver takes.
///
/// iced multiplies this by the track's height to get the padding on each side
/// (`toggler.rs:444`) and gives the thumb what is left (`toggler.rs:453-454`),
/// so the model's two lengths state it exactly. Where the geometry cannot
/// state it, the field follows the no-source rule and the expected value is
/// iced's own, read at run time -- never a literal.
#[cfg(feature = "widgets")]
fn native_toggler_padding_ratio(t: &Theme, r: &ResolvedTheme, status: toggler::Status) -> f32 {
    let s = &r.switch;
    if switch_geometry_is_usable(r) {
        (s.track_height - s.thumb_diameter) / (2.0 * s.track_height)
    } else {
        toggler::default(t, status).padding_ratio
    }
}

/// The switch's thumb inset, the one scalar whose native value is guarded.
#[cfg(feature = "widgets")]
pub(super) const TOGGLER_SCALAR_ROWS: &[ScalarRow<toggler::Status>] = &[ScalarRow {
    field: "styles::toggler.padding_ratio",
    statuses: TOGGLER_STATUSES,
    native: native_toggler_padding_ratio,
    get: |t, r, s| Ok(styles::toggler(r)(t, s).padding_ratio),
}];

/// The switch's corner radius, `switch.track_radius`. It shapes the thumb as
/// well as the track: iced paints both quads with this one radius
/// (`toggler.rs:435`, `:461`).
#[cfg(feature = "widgets")]
pub(super) const TOGGLER_RADIUS_ROWS: &[RadiusRow<toggler::Status>] = &[RadiusRow {
    field: "styles::toggler.border_radius",
    statuses: TOGGLER_STATUSES,
    native: |r, _| r.switch.track_radius,
    get: |t, r, s| {
        styles::toggler(r)(t, s).border_radius.ok_or_else(|| {
            "no radius where the contract claims one; iced's None would make \
             the switch perfectly round whatever the platform states"
                .to_string()
        })
    },
}];

/// Every value of `pick_list::Status`: `Opened` carries whether the pointer is
/// on the field (`pick_list.rs:838-849`), so there are four.
#[cfg(feature = "widgets")]
pub(super) const PICK_LIST_STATUSES: &[pick_list::Status] = &[
    pick_list::Status::Active,
    pick_list::Status::Hovered,
    pick_list::Status::Opened { is_hovered: false },
    pick_list::Status::Opened { is_hovered: true },
];

/// The fill the native fields give a drop-down field in `status`. The model
/// states no open appearance, so an open field follows its own `is_hovered`.
#[cfg(feature = "widgets")]
pub(super) fn native_pick_list_fill(r: &ResolvedTheme, status: pick_list::Status) -> Color {
    let c = &r.combo_box;
    let idle = to_color(c.background_color);
    let hovered = || {
        over(
            to_color(c.hover_background.unwrap_or(c.background_color)),
            idle,
        )
    };
    match status {
        pick_list::Status::Active => idle,
        pick_list::Status::Hovered => hovered(),
        pick_list::Status::Opened { is_hovered } => {
            if is_hovered {
                hovered()
            } else {
                idle
            }
        }
    }
}

/// Every color field of `styles::pick_list` but `handle_color`, which is
/// iced's and sits in `DERIVED`.
#[cfg(feature = "widgets")]
pub(super) const PICK_LIST_ROWS: &[StyleRow<pick_list::Status>] = &[
    StyleRow {
        field: "styles::pick_list.background",
        statuses: PICK_LIST_STATUSES,
        native: native_pick_list_fill,
        get: |t, r, s| fill(styles::pick_list(r)(t, s).background),
    },
    StyleRow {
        field: "styles::pick_list.text_color",
        statuses: PICK_LIST_STATUSES,
        native: |r, _| to_color(r.combo_box.font.color),
        get: |t, r, s| Ok(styles::pick_list(r)(t, s).text_color),
    },
    StyleRow {
        field: "styles::pick_list.placeholder_color",
        statuses: PICK_LIST_STATUSES,
        native: |r, _| to_color(r.input.placeholder_color),
        get: |t, r, s| Ok(styles::pick_list(r)(t, s).placeholder_color),
    },
];

#[cfg(feature = "widgets")]
pub(super) const PICK_LIST_BORDER_ROWS: &[BorderRow<pick_list::Status>] = &[BorderRow {
    field: "styles::pick_list",
    statuses: PICK_LIST_STATUSES,
    native: |r, _| NativeBorder {
        color: to_color(r.combo_box.border.color),
        width: r.combo_box.border.line_width,
        radius: r.combo_box.border.corner_radius,
    },
    get: |t, r, s| styles::pick_list(r)(t, s).border,
}];

/// A menu has no `Status` (shape C), so its rows hold the unit value: the row
/// types are generic over the widget's status, and `()` is what a widget
/// without one has.
#[cfg(feature = "widgets")]
pub(super) const MENU_STATUSES: &[()] = &[()];

/// Every color field of `styles::menu`. Only `shadow` is iced's.
#[cfg(feature = "widgets")]
pub(super) const MENU_ROWS: &[StyleRow<()>] = &[
    StyleRow {
        field: "styles::menu.background",
        statuses: MENU_STATUSES,
        native: |r, ()| to_color(r.menu.background_color),
        get: |t, r, ()| fill(styles::menu(r)(t).background),
    },
    StyleRow {
        field: "styles::menu.text_color",
        statuses: MENU_STATUSES,
        native: |r, ()| to_color(r.menu.font.color),
        get: |t, r, ()| Ok(styles::menu(r)(t).text_color),
    },
    StyleRow {
        field: "styles::menu.selected_text_color",
        statuses: MENU_STATUSES,
        native: |r, ()| to_color(r.menu.hover_text_color),
        get: |t, r, ()| Ok(styles::menu(r)(t).selected_text_color),
    },
    StyleRow {
        // A row highlight, painted over a panel the widget also paints, so it
        // is emitted as given rather than composited.
        field: "styles::menu.selected_background",
        statuses: MENU_STATUSES,
        native: |r, ()| to_color(r.menu.hover_background),
        get: |t, r, ()| fill(styles::menu(r)(t).selected_background),
    },
];

#[cfg(feature = "widgets")]
pub(super) const MENU_BORDER_ROWS: &[BorderRow<()>] = &[BorderRow {
    field: "styles::menu",
    statuses: MENU_STATUSES,
    native: |r, ()| NativeBorder {
        color: to_color(r.menu.border.color),
        width: r.menu.border.line_width,
        radius: r.menu.border.corner_radius,
    },
    get: |t, r, ()| styles::menu(r)(t).border,
}];

/// Every `slider::Status` (`slider.rs:576-586`). The rail's rows hold for all
/// three; the handle's fill does not.
#[cfg(feature = "widgets")]
pub(super) const SLIDER_STATUSES: &[slider::Status] = &[
    slider::Status::Active,
    slider::Status::Hovered,
    slider::Status::Dragged,
];

/// The statuses whose handle the model states a *fill* for. Only the fill: the
/// handle's shape is `slider.thumb_diameter` in every status, dragged
/// included, and so is the rail under it.
#[cfg(feature = "widgets")]
pub(super) const SLIDER_NATIVE_FILL_STATUSES: &[slider::Status] =
    &[slider::Status::Active, slider::Status::Hovered];

/// The status whose handle fill is iced's, because `SliderTheme` states no
/// dragged thumb color. Asserted by
/// `a_dragged_slider_handle_takes_its_fill_from_iced`.
#[cfg(feature = "widgets")]
pub(super) const SLIDER_ICED_FILL_STATUSES: &[slider::Status] = &[slider::Status::Dragged];

/// The handle fill the native fields give a slider in `status`.
///
/// The match has no catch-all, but only `SLIDER_NATIVE_FILL_STATUSES` ever
/// reaches a row; `Dragged` still has to name a color, and the thumb's own is
/// the least surprising, so moving that status between the lists fails the row
/// rather than passing quietly.
#[cfg(feature = "widgets")]
pub(super) fn native_slider_handle(r: &ResolvedTheme, status: slider::Status) -> Color {
    let s = &r.slider;
    to_color(match status {
        slider::Status::Active | slider::Status::Dragged => s.thumb_color,
        // A thumb, so emitted as given rather than composited.
        slider::Status::Hovered => s.thumb_hover_color.unwrap_or(s.thumb_color),
    })
}

/// Every color field of `styles::slider`.
#[cfg(feature = "widgets")]
pub(super) const SLIDER_ROWS: &[StyleRow<slider::Status>] = &[
    StyleRow {
        field: "styles::slider.rail.backgrounds.0",
        statuses: SLIDER_STATUSES,
        native: |r, _| to_color(r.slider.fill_color),
        get: |t, r, s| fill(styles::slider(r)(t, s).rail.backgrounds.0),
    },
    StyleRow {
        field: "styles::slider.rail.backgrounds.1",
        statuses: SLIDER_STATUSES,
        native: |r, _| to_color(r.slider.track_color),
        get: |t, r, s| fill(styles::slider(r)(t, s).rail.backgrounds.1),
    },
    StyleRow {
        field: "styles::slider.handle.background",
        statuses: SLIDER_NATIVE_FILL_STATUSES,
        native: native_slider_handle,
        get: |t, r, s| fill(styles::slider(r)(t, s).handle.background),
    },
];

/// The slider's two lengths.
///
/// `handle.shape` is an enum rather than a number, and this row is what
/// `ScalarRow`'s `Result` exists for: the shape carries the size the model
/// states only while it is a circle, so a rectangular handle is reported as a
/// failure of the row instead of being unwrapped. Adding a row kind for an
/// enum whose one native value is a length would have said less.
#[cfg(feature = "widgets")]
pub(super) const SLIDER_SCALAR_ROWS: &[ScalarRow<slider::Status>] = &[
    ScalarRow {
        field: "styles::slider.rail.width",
        statuses: SLIDER_STATUSES,
        native: |_, r, _| r.slider.track_height,
        get: |t, r, s| Ok(styles::slider(r)(t, s).rail.width),
    },
    ScalarRow {
        field: "styles::slider.handle.shape",
        statuses: SLIDER_STATUSES,
        // iced states a circular handle by its radius, the model states the
        // thumb's diameter.
        native: |_, r, _| r.slider.thumb_diameter / 2.0,
        get: |t, r, s| match styles::slider(r)(t, s).handle.shape {
            slider::HandleShape::Circle { radius } => Ok(radius),
            slider::HandleShape::Rectangle {
                width: _,
                border_radius: _,
            } => Err("a rectangular handle where the contract claims a circle".to_string()),
        },
    },
];

/// One `scrollable::Status::Active` value, from the two booleans it carries,
/// in their declaration order (`scrollable.rs:2239-2244`): the horizontal
/// scrollbar's disabled flag, then the vertical one's.
#[cfg(feature = "widgets")]
pub(super) const fn scrollable_active(
    horizontal_off: bool,
    vertical_off: bool,
) -> scrollable::Status {
    scrollable::Status::Active {
        is_horizontal_scrollbar_disabled: horizontal_off,
        is_vertical_scrollbar_disabled: vertical_off,
    }
}

/// One `scrollable::Status::Hovered` value, from the four booleans it carries,
/// in their declaration order (`scrollable.rs:2246-2255`): horizontal hovered,
/// vertical hovered, horizontal disabled, vertical disabled.
#[cfg(feature = "widgets")]
pub(super) const fn scrollable_hovered(
    horizontal: bool,
    vertical: bool,
    horizontal_off: bool,
    vertical_off: bool,
) -> scrollable::Status {
    scrollable::Status::Hovered {
        is_horizontal_scrollbar_hovered: horizontal,
        is_vertical_scrollbar_hovered: vertical,
        is_horizontal_scrollbar_disabled: horizontal_off,
        is_vertical_scrollbar_disabled: vertical_off,
    }
}

/// One `scrollable::Status::Dragged` value, from the four booleans it carries,
/// in their declaration order (`scrollable.rs:2257-2266`): horizontal dragged,
/// vertical dragged, horizontal disabled, vertical disabled.
#[cfg(feature = "widgets")]
pub(super) const fn scrollable_dragged(
    horizontal: bool,
    vertical: bool,
    horizontal_off: bool,
    vertical_off: bool,
) -> scrollable::Status {
    scrollable::Status::Dragged {
        is_horizontal_scrollbar_dragged: horizontal,
        is_vertical_scrollbar_dragged: vertical,
        is_horizontal_scrollbar_disabled: horizontal_off,
        is_vertical_scrollbar_disabled: vertical_off,
    }
}

/// Every value of `scrollable::Status` (`scrollable.rs:2237-2267`).
///
/// This is the one status enum whose values are a *product* rather than a
/// short list: `Active` carries two independent booleans and the other two
/// variants carry four each, so the value space is `2^2 + 2^4 + 2^4 = 36`.
/// Writing thirty-six struct literals here would hide what the list is, so
/// each value is built by the small constructor for its variant -- which names
/// the fields once -- and the product is spelled out in the order the booleans
/// count up. `every_status_list_names_each_status_once` builds the same
/// thirty-six from nested loops and requires the two to agree, so a value
/// dropped from this list fails there.
#[cfg(feature = "widgets")]
pub(super) const SCROLLABLE_STATUSES: &[scrollable::Status] = &[
    scrollable_active(false, false),
    scrollable_active(false, true),
    scrollable_active(true, false),
    scrollable_active(true, true),
    scrollable_hovered(false, false, false, false),
    scrollable_hovered(false, false, false, true),
    scrollable_hovered(false, false, true, false),
    scrollable_hovered(false, false, true, true),
    scrollable_hovered(false, true, false, false),
    scrollable_hovered(false, true, false, true),
    scrollable_hovered(false, true, true, false),
    scrollable_hovered(false, true, true, true),
    scrollable_hovered(true, false, false, false),
    scrollable_hovered(true, false, false, true),
    scrollable_hovered(true, false, true, false),
    scrollable_hovered(true, false, true, true),
    scrollable_hovered(true, true, false, false),
    scrollable_hovered(true, true, false, true),
    scrollable_hovered(true, true, true, false),
    scrollable_hovered(true, true, true, true),
    scrollable_dragged(false, false, false, false),
    scrollable_dragged(false, false, false, true),
    scrollable_dragged(false, false, true, false),
    scrollable_dragged(false, false, true, true),
    scrollable_dragged(false, true, false, false),
    scrollable_dragged(false, true, false, true),
    scrollable_dragged(false, true, true, false),
    scrollable_dragged(false, true, true, true),
    scrollable_dragged(true, false, false, false),
    scrollable_dragged(true, false, false, true),
    scrollable_dragged(true, false, true, false),
    scrollable_dragged(true, false, true, true),
    scrollable_dragged(true, true, false, false),
    scrollable_dragged(true, true, false, true),
    scrollable_dragged(true, true, true, false),
    scrollable_dragged(true, true, true, true),
];

/// How many values `scrollable::Status` has: the product above, written as the
/// product so that a fifth boolean upstream is a changed number here rather
/// than a silently smaller list.
#[cfg(feature = "widgets")]
pub(super) const SCROLLABLE_STATUS_VALUES: usize = 2 * 2 + 2 * 2 * 2 * 2 + 2 * 2 * 2 * 2;

/// The scroller fill the native fields give one axis of a scrollable in
/// `status`.
///
/// `vertical` picks the axis, because iced states the two rails separately and
/// the status says which one the pointer is on: the other stays idle. The
/// disabled flags are not read -- the model states no appearance for a
/// scrollbar whose content does not overflow.
#[cfg(feature = "widgets")]
pub(super) fn native_scroller(
    r: &ResolvedTheme,
    status: scrollable::Status,
    vertical: bool,
) -> Color {
    let s = &r.scrollbar;
    to_color(match status {
        scrollable::Status::Active {
            is_horizontal_scrollbar_disabled: _,
            is_vertical_scrollbar_disabled: _,
        } => s.thumb_color,
        scrollable::Status::Hovered {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered,
            is_horizontal_scrollbar_disabled: _,
            is_vertical_scrollbar_disabled: _,
        } => {
            if vertical && is_vertical_scrollbar_hovered
                || !vertical && is_horizontal_scrollbar_hovered
            {
                s.thumb_hover_color
            } else {
                s.thumb_color
            }
        }
        scrollable::Status::Dragged {
            is_horizontal_scrollbar_dragged,
            is_vertical_scrollbar_dragged,
            is_horizontal_scrollbar_disabled: _,
            is_vertical_scrollbar_disabled: _,
        } => {
            if vertical && is_vertical_scrollbar_dragged
                || !vertical && is_horizontal_scrollbar_dragged
            {
                // A soft option, copying the hovered scroller's color.
                s.thumb_active_color.unwrap_or(s.thumb_hover_color)
            } else {
                s.thumb_color
            }
        }
    })
}

/// Every color field of `styles::scrollable`: each rail's own fill and the
/// scroller on it. Everything else the struct carries is iced's.
#[cfg(feature = "widgets")]
pub(super) const SCROLLABLE_ROWS: &[StyleRow<scrollable::Status>] = &[
    StyleRow {
        field: "styles::scrollable.vertical_rail.background",
        statuses: SCROLLABLE_STATUSES,
        native: |r, _| to_color(r.scrollbar.track_color),
        get: |t, r, s| flat(styles::scrollable(r)(t, s).vertical_rail.background),
    },
    StyleRow {
        // A thumb, so emitted as given rather than composited: iced paints it
        // over a rail it also paints.
        field: "styles::scrollable.vertical_rail.scroller.background",
        statuses: SCROLLABLE_STATUSES,
        native: |r, s| native_scroller(r, s, true),
        get: |t, r, s| {
            fill(
                styles::scrollable(r)(t, s)
                    .vertical_rail
                    .scroller
                    .background,
            )
        },
    },
    StyleRow {
        field: "styles::scrollable.horizontal_rail.background",
        statuses: SCROLLABLE_STATUSES,
        native: |r, _| to_color(r.scrollbar.track_color),
        get: |t, r, s| flat(styles::scrollable(r)(t, s).horizontal_rail.background),
    },
    StyleRow {
        field: "styles::scrollable.horizontal_rail.scroller.background",
        statuses: SCROLLABLE_STATUSES,
        native: |r, s| native_scroller(r, s, false),
        get: |t, r, s| {
            fill(
                styles::scrollable(r)(t, s)
                    .horizontal_rail
                    .scroller
                    .background,
            )
        },
    },
];

/// A progress bar has no `Status` (shape B), so its rows hold the unit value,
/// as a menu's do.
#[cfg(feature = "widgets")]
pub(super) const PROGRESS_BAR_STATUSES: &[()] = &[()];

/// Every color field of `styles::progress_bar`. Nothing here is iced's: the
/// struct is a track, a bar and a border, and the model states all three.
#[cfg(feature = "widgets")]
pub(super) const PROGRESS_BAR_ROWS: &[StyleRow<()>] = &[
    StyleRow {
        field: "styles::progress_bar.background",
        statuses: PROGRESS_BAR_STATUSES,
        native: |r, ()| to_color(r.progress_bar.track_color),
        get: |t, r, ()| fill(styles::progress_bar(r)(t).background),
    },
    StyleRow {
        field: "styles::progress_bar.bar",
        statuses: PROGRESS_BAR_STATUSES,
        native: |r, ()| to_color(r.progress_bar.fill_color),
        get: |t, r, ()| fill(styles::progress_bar(r)(t).bar),
    },
];

#[cfg(feature = "widgets")]
pub(super) const PROGRESS_BAR_BORDER_ROWS: &[BorderRow<()>] = &[BorderRow {
    field: "styles::progress_bar",
    statuses: PROGRESS_BAR_STATUSES,
    native: |r, ()| NativeBorder {
        color: to_color(r.progress_bar.border.color),
        width: r.progress_bar.border.line_width,
        radius: r.progress_bar.border.corner_radius,
    },
    get: |t, r, ()| styles::progress_bar(r)(t).border,
}];

/// A rule has no `Status` (shape B).
#[cfg(feature = "widgets")]
pub(super) const RULE_STATUSES: &[()] = &[()];

/// The one color field of `styles::rule`; its other three are iced's.
#[cfg(feature = "widgets")]
pub(super) const RULE_ROWS: &[StyleRow<()>] = &[StyleRow {
    field: "styles::rule.color",
    statuses: RULE_STATUSES,
    native: |r, ()| to_color(r.separator.line_color),
    get: |t, r, ()| Ok(styles::rule(r)(t).color),
}];

/// A tooltip has no `Status` (shape B).
#[cfg(feature = "widgets")]
pub(super) const TOOLTIP_STATUSES: &[()] = &[()];

/// Every color field of `styles::tooltip`. It emits a `container::Style`, so
/// its label is an `Option<Color>` the model does fill -- a `None` would be a
/// tip that inherits a label color from whatever it floats over.
#[cfg(feature = "widgets")]
pub(super) const TOOLTIP_ROWS: &[StyleRow<()>] = &[
    StyleRow {
        field: "styles::tooltip.text_color",
        statuses: TOOLTIP_STATUSES,
        native: |r, ()| to_color(r.tooltip.font.color),
        get: |t, r, ()| stated(styles::tooltip(r)(t).text_color),
    },
    StyleRow {
        field: "styles::tooltip.background",
        statuses: TOOLTIP_STATUSES,
        native: |r, ()| to_color(r.tooltip.background_color),
        get: |t, r, ()| flat(styles::tooltip(r)(t).background),
    },
];

#[cfg(feature = "widgets")]
pub(super) const TOOLTIP_BORDER_ROWS: &[BorderRow<()>] = &[BorderRow {
    field: "styles::tooltip",
    statuses: TOOLTIP_STATUSES,
    native: |r, ()| NativeBorder {
        color: to_color(r.tooltip.border.color),
        width: r.tooltip.border.line_width,
        radius: r.tooltip.border.corner_radius,
    },
    get: |t, r, ()| styles::tooltip(r)(t).border,
}];

/// A card has no `Status` (shape B).
#[cfg(feature = "widgets")]
pub(super) const CONTAINER_CARD_STATUSES: &[()] = &[()];

/// The one color field of `styles::container_card`: `CardTheme` is a fill and
/// a border, and the label inside a card is inherited rather than set.
#[cfg(feature = "widgets")]
pub(super) const CONTAINER_CARD_ROWS: &[StyleRow<()>] = &[StyleRow {
    field: "styles::container_card.background",
    statuses: CONTAINER_CARD_STATUSES,
    native: |r, ()| to_color(r.card.background_color),
    get: |t, r, ()| flat(styles::container_card(r)(t).background),
}];

#[cfg(feature = "widgets")]
pub(super) const CONTAINER_CARD_BORDER_ROWS: &[BorderRow<()>] = &[BorderRow {
    field: "styles::container_card",
    statuses: CONTAINER_CARD_STATUSES,
    native: |r, ()| NativeBorder {
        color: to_color(r.card.border.color),
        width: r.card.border.line_width,
        radius: r.card.border.corner_radius,
    },
    get: |t, r, ()| styles::container_card(r)(t).border,
}];

// ---- `styles::aw`: the six `iced_aw` widgets (section 3a) ----

/// Every value of `iced_aw`'s shared `Status`, which each of its widgets
/// requests only the values it has a meaning for (`style/status.rs:5-18`).
#[cfg(feature = "iced_aw")]
pub(super) const AW_STATUSES: &[AwStatus] = &[
    AwStatus::Active,
    AwStatus::Hovered,
    AwStatus::Pressed,
    AwStatus::Disabled,
    AwStatus::Focused,
    AwStatus::Selected,
];

/// A card and a menu bar look the same in every status -- the model states one
/// appearance for each, and `iced_aw`'s own classes ignore the status too
/// (`style/card.rs:92`, `style/menu_bar.rs:83`) -- so their rows cover every
/// value of the enum.
#[cfg(feature = "iced_aw")]
pub(super) const AW_CARD_STATUSES: &[AwStatus] = AW_STATUSES;

#[cfg(feature = "iced_aw")]
pub(super) const AW_MENU_STATUSES: &[AwStatus] = AW_STATUSES;

/// The three statuses a `TabBar` asks for, and the only ones its label rows
/// cover: `Hovered` is the tab under the pointer, `Active` the selected tab,
/// `Disabled` a tab that is merely not selected (`tab_bar.rs:588-594`).
#[cfg(feature = "iced_aw")]
pub(super) const AW_TAB_BAR_NATIVE_STATUSES: &[AwStatus] =
    &[AwStatus::Active, AwStatus::Hovered, AwStatus::Disabled];

/// The three a `TabBar` never asks for, whose labels are `iced_aw`'s own.
/// Asserted by `an_aw_widget_takes_the_statuses_it_never_receives_from_iced_aw`.
#[cfg(feature = "iced_aw")]
pub(super) const AW_TAB_BAR_ICED_STATUSES: &[AwStatus] =
    &[AwStatus::Pressed, AwStatus::Focused, AwStatus::Selected];

/// The same three for a `Sidebar`, which reads the enum the way a `TabBar`
/// does (`sidebar/sidebar.rs:979-985`).
#[cfg(feature = "iced_aw")]
pub(super) const AW_SIDEBAR_NATIVE_STATUSES: &[AwStatus] = AW_TAB_BAR_NATIVE_STATUSES;

#[cfg(feature = "iced_aw")]
pub(super) const AW_SIDEBAR_ICED_STATUSES: &[AwStatus] = AW_TAB_BAR_ICED_STATUSES;

/// A `SelectionList` asks for `Active`, `Hovered` and `Selected`
/// (`selection_list.rs:302`, `selection_list/list.rs:241-258`); `Disabled` is
/// a row the model does describe, through `list.disabled_text_color`.
#[cfg(feature = "iced_aw")]
pub(super) const AW_SELECTION_LIST_NATIVE_STATUSES: &[AwStatus] = &[
    AwStatus::Active,
    AwStatus::Hovered,
    AwStatus::Selected,
    AwStatus::Disabled,
];

#[cfg(feature = "iced_aw")]
pub(super) const AW_SELECTION_LIST_ICED_STATUSES: &[AwStatus] =
    &[AwStatus::Pressed, AwStatus::Focused];

/// Every color field of `styles::aw::card`. The card is one surface, so its
/// three sections carry the same fill, and its labels are the platform's own
/// text color -- `CardTheme` carries no font.
#[cfg(feature = "iced_aw")]
pub(super) const AW_CARD_ROWS: &[StyleRow<AwStatus>] = &[
    StyleRow {
        field: "styles::aw::card.background",
        statuses: AW_CARD_STATUSES,
        native: |r, _| to_color(r.card.background_color),
        get: |t, r, s| fill(styles::aw::card(r)(t, s).background),
    },
    StyleRow {
        field: "styles::aw::card.border_color",
        statuses: AW_CARD_STATUSES,
        native: |r, _| to_color(r.card.border.color),
        get: |t, r, s| Ok(styles::aw::card(r)(t, s).border_color),
    },
    StyleRow {
        field: "styles::aw::card.head_background",
        statuses: AW_CARD_STATUSES,
        native: |r, _| to_color(r.card.background_color),
        get: |t, r, s| fill(styles::aw::card(r)(t, s).head_background),
    },
    StyleRow {
        field: "styles::aw::card.head_text_color",
        statuses: AW_CARD_STATUSES,
        native: |r, _| to_color(r.defaults.text_color),
        get: |t, r, s| Ok(styles::aw::card(r)(t, s).head_text_color),
    },
    StyleRow {
        field: "styles::aw::card.body_background",
        statuses: AW_CARD_STATUSES,
        native: |r, _| to_color(r.card.background_color),
        get: |t, r, s| fill(styles::aw::card(r)(t, s).body_background),
    },
    StyleRow {
        field: "styles::aw::card.body_text_color",
        statuses: AW_CARD_STATUSES,
        native: |r, _| to_color(r.defaults.text_color),
        get: |t, r, s| Ok(styles::aw::card(r)(t, s).body_text_color),
    },
    StyleRow {
        field: "styles::aw::card.foot_background",
        statuses: AW_CARD_STATUSES,
        native: |r, _| to_color(r.card.background_color),
        get: |t, r, s| fill(styles::aw::card(r)(t, s).foot_background),
    },
    StyleRow {
        field: "styles::aw::card.foot_text_color",
        statuses: AW_CARD_STATUSES,
        native: |r, _| to_color(r.defaults.text_color),
        get: |t, r, s| Ok(styles::aw::card(r)(t, s).foot_text_color),
    },
    StyleRow {
        field: "styles::aw::card.close_color",
        statuses: AW_CARD_STATUSES,
        native: |r, _| to_color(r.defaults.text_color),
        get: |t, r, s| Ok(styles::aw::card(r)(t, s).close_color),
    },
];

/// A card's border is three bare fields rather than an iced `Border`, as a
/// radio's is, so its two lengths are scalar rows.
#[cfg(feature = "iced_aw")]
pub(super) const AW_CARD_SCALAR_ROWS: &[ScalarRow<AwStatus>] = &[
    ScalarRow {
        field: "styles::aw::card.border_width",
        statuses: AW_CARD_STATUSES,
        native: |_, r, _| r.card.border.line_width,
        get: |t, r, s| Ok(styles::aw::card(r)(t, s).border_width),
    },
    ScalarRow {
        field: "styles::aw::card.border_radius",
        statuses: AW_CARD_STATUSES,
        native: |_, r, _| r.card.border.corner_radius,
        get: |t, r, s| Ok(styles::aw::card(r)(t, s).border_radius),
    },
];

/// Every color field of `styles::aw::menu`: the bar and the menus share the
/// one panel the model states, and the open path is the menu's hover fill.
#[cfg(feature = "iced_aw")]
pub(super) const AW_MENU_ROWS: &[StyleRow<AwStatus>] = &[
    StyleRow {
        field: "styles::aw::menu.bar_background",
        statuses: AW_MENU_STATUSES,
        native: |r, _| to_color(r.menu.background_color),
        get: |t, r, s| fill(styles::aw::menu(r)(t, s).bar_background),
    },
    StyleRow {
        field: "styles::aw::menu.menu_background",
        statuses: AW_MENU_STATUSES,
        native: |r, _| to_color(r.menu.background_color),
        get: |t, r, s| fill(styles::aw::menu(r)(t, s).menu_background),
    },
    StyleRow {
        field: "styles::aw::menu.path",
        statuses: AW_MENU_STATUSES,
        native: |r, _| to_color(r.menu.hover_background),
        get: |t, r, s| fill(styles::aw::menu(r)(t, s).path),
    },
];

/// The menu's own border, worn by the bar and by the menus alike. The third
/// border of the struct, `path_border`, has no native source and is in
/// `DERIVED`.
#[cfg(feature = "iced_aw")]
pub(super) const AW_MENU_BORDER_ROWS: &[BorderRow<AwStatus>] = &[
    BorderRow {
        field: "styles::aw::menu.bar",
        statuses: AW_MENU_STATUSES,
        native: |r, _| native_aw_menu_border(r),
        get: |t, r, s| styles::aw::menu(r)(t, s).bar_border,
    },
    BorderRow {
        field: "styles::aw::menu.menu",
        statuses: AW_MENU_STATUSES,
        native: |r, _| native_aw_menu_border(r),
        get: |t, r, s| styles::aw::menu(r)(t, s).menu_border,
    },
];

/// The one border `MenuTheme` states.
#[cfg(feature = "iced_aw")]
fn native_aw_menu_border(r: &ResolvedTheme) -> NativeBorder {
    NativeBorder {
        color: to_color(r.menu.border.color),
        width: r.menu.border.line_width,
        radius: r.menu.border.corner_radius,
    }
}

/// The fill the native fields give a tab in `status`.
///
/// The match has no catch-all, but only `AW_TAB_BAR_NATIVE_STATUSES` reaches a
/// row: in the other three the fill is `iced_aw`'s own. Their arm still has to
/// name a color, and the idle one is the least surprising -- if a status is
/// ever moved between the two lists, the row compares it against that and
/// fails rather than passing quietly.
#[cfg(feature = "iced_aw")]
pub(super) fn native_aw_tab_fill(r: &ResolvedTheme, status: AwStatus) -> Color {
    let t = &r.tab;
    to_color(match status {
        AwStatus::Active => t.active_background,
        // A tab's hover fill is a soft option, and a row highlight: emitted as
        // given, over the strip this same function fills (C17).
        AwStatus::Hovered => t.hover_background.unwrap_or(t.background_color),
        AwStatus::Disabled | AwStatus::Pressed | AwStatus::Focused | AwStatus::Selected => {
            t.background_color
        }
    })
}

/// The label color the native fields give a tab in `status`.
#[cfg(feature = "iced_aw")]
pub(super) fn native_aw_tab_label(r: &ResolvedTheme, status: AwStatus) -> Color {
    let t = &r.tab;
    to_color(match status {
        AwStatus::Active => t.active_text_color,
        AwStatus::Hovered => t.hover_text_color,
        AwStatus::Disabled | AwStatus::Pressed | AwStatus::Focused | AwStatus::Selected => {
            t.font.color
        }
    })
}

/// Every color field of `styles::aw::tab_bar`.
#[cfg(feature = "iced_aw")]
pub(super) const AW_TAB_BAR_ROWS: &[StyleRow<AwStatus>] = &[
    StyleRow {
        field: "styles::aw::tab_bar.background",
        statuses: AW_STATUSES,
        native: |r, _| to_color(r.tab.bar_background),
        get: |t, r, s| flat(styles::aw::tab_bar(r)(t, s).background),
    },
    StyleRow {
        field: "styles::aw::tab_bar.tab_label_border_color",
        statuses: AW_STATUSES,
        native: |r, _| to_color(r.tab.border.color),
        get: |t, r, s| Ok(styles::aw::tab_bar(r)(t, s).tab_label_border_color),
    },
    StyleRow {
        field: "styles::aw::tab_bar.tab_label_background",
        statuses: AW_TAB_BAR_NATIVE_STATUSES,
        native: native_aw_tab_fill,
        get: |t, r, s| fill(styles::aw::tab_bar(r)(t, s).tab_label_background),
    },
    StyleRow {
        field: "styles::aw::tab_bar.text_color",
        statuses: AW_TAB_BAR_NATIVE_STATUSES,
        native: native_aw_tab_label,
        get: |t, r, s| Ok(styles::aw::tab_bar(r)(t, s).text_color),
    },
    StyleRow {
        field: "styles::aw::tab_bar.icon_color",
        statuses: AW_TAB_BAR_NATIVE_STATUSES,
        native: native_aw_tab_label,
        get: |t, r, s| Ok(styles::aw::tab_bar(r)(t, s).icon_color),
    },
];

#[cfg(feature = "iced_aw")]
pub(super) const AW_TAB_BAR_SCALAR_ROWS: &[ScalarRow<AwStatus>] = &[ScalarRow {
    field: "styles::aw::tab_bar.tab_label_border_width",
    statuses: AW_STATUSES,
    native: |_, r, _| r.tab.border.line_width,
    get: |t, r, s| Ok(styles::aw::tab_bar(r)(t, s).tab_label_border_width),
}];

/// A tab's corner radius is a whole `Radius` rather than part of a `Border`,
/// as a toggler's is.
#[cfg(feature = "iced_aw")]
pub(super) const AW_TAB_BAR_RADIUS_ROWS: &[RadiusRow<AwStatus>] = &[RadiusRow {
    field: "styles::aw::tab_bar.tab_border_radius",
    statuses: AW_STATUSES,
    native: |r, _| r.tab.border.corner_radius,
    get: |t, r, s| Ok(styles::aw::tab_bar(r)(t, s).tab_border_radius),
}];

/// The fill the native fields give a sidebar item in `status`. The unreached
/// arm is the panel's own fill, for the reason `native_aw_tab_fill` gives.
#[cfg(feature = "iced_aw")]
pub(super) fn native_aw_sidebar_fill(r: &ResolvedTheme, status: AwStatus) -> Color {
    let s = &r.sidebar;
    to_color(match status {
        AwStatus::Active => s.selection_background,
        AwStatus::Hovered => s.hover_background,
        AwStatus::Disabled | AwStatus::Pressed | AwStatus::Focused | AwStatus::Selected => {
            s.background_color
        }
    })
}

/// The label color the native fields give a sidebar item in `status`.
/// `SidebarTheme` states no hovered label, so a hovered item keeps the
/// sidebar's own.
#[cfg(feature = "iced_aw")]
pub(super) fn native_aw_sidebar_label(r: &ResolvedTheme, status: AwStatus) -> Color {
    let s = &r.sidebar;
    to_color(match status {
        AwStatus::Active => s.selection_text_color,
        AwStatus::Hovered
        | AwStatus::Disabled
        | AwStatus::Pressed
        | AwStatus::Focused
        | AwStatus::Selected => s.font.color,
    })
}

/// Every color field of `styles::aw::sidebar`.
#[cfg(feature = "iced_aw")]
pub(super) const AW_SIDEBAR_ROWS: &[StyleRow<AwStatus>] = &[
    StyleRow {
        field: "styles::aw::sidebar.background",
        statuses: AW_STATUSES,
        native: |r, _| to_color(r.sidebar.background_color),
        get: |t, r, s| flat(styles::aw::sidebar(r)(t, s).background),
    },
    StyleRow {
        field: "styles::aw::sidebar.border_color",
        statuses: AW_STATUSES,
        native: |r, _| to_color(r.sidebar.border.color),
        get: |t, r, s| stated(styles::aw::sidebar(r)(t, s).border_color),
    },
    StyleRow {
        field: "styles::aw::sidebar.tab_label_background",
        statuses: AW_SIDEBAR_NATIVE_STATUSES,
        native: native_aw_sidebar_fill,
        get: |t, r, s| fill(styles::aw::sidebar(r)(t, s).tab_label_background),
    },
    StyleRow {
        field: "styles::aw::sidebar.text_color",
        statuses: AW_SIDEBAR_NATIVE_STATUSES,
        native: native_aw_sidebar_label,
        get: |t, r, s| Ok(styles::aw::sidebar(r)(t, s).text_color),
    },
    StyleRow {
        field: "styles::aw::sidebar.icon_color",
        statuses: AW_SIDEBAR_NATIVE_STATUSES,
        native: native_aw_sidebar_label,
        get: |t, r, s| Ok(styles::aw::sidebar(r)(t, s).icon_color),
    },
];

#[cfg(feature = "iced_aw")]
pub(super) const AW_SIDEBAR_SCALAR_ROWS: &[ScalarRow<AwStatus>] = &[ScalarRow {
    field: "styles::aw::sidebar.border_width",
    statuses: AW_STATUSES,
    native: |_, r, _| r.sidebar.border.line_width,
    get: |t, r, s| Ok(styles::aw::sidebar(r)(t, s).border_width),
}];

/// The fill the native fields give a selection list row in `status`. The
/// unreached arm is the list's own fill, for the reason `native_aw_tab_fill`
/// gives.
#[cfg(feature = "iced_aw")]
pub(super) fn native_aw_list_fill(r: &ResolvedTheme, status: AwStatus) -> Color {
    let l = &r.list;
    to_color(match status {
        AwStatus::Hovered => l.hover_background,
        AwStatus::Selected => l.selection_background,
        AwStatus::Active | AwStatus::Disabled | AwStatus::Pressed | AwStatus::Focused => {
            l.background_color
        }
    })
}

/// The label color the native fields give a selection list row in `status`.
#[cfg(feature = "iced_aw")]
pub(super) fn native_aw_list_label(r: &ResolvedTheme, status: AwStatus) -> Color {
    let l = &r.list;
    to_color(match status {
        AwStatus::Hovered => l.hover_text_color,
        AwStatus::Selected => l.selection_text_color,
        AwStatus::Disabled => l.disabled_text_color,
        AwStatus::Active | AwStatus::Pressed | AwStatus::Focused => l.item_font.color,
    })
}

/// Every color field of `styles::aw::selection_list`.
#[cfg(feature = "iced_aw")]
pub(super) const AW_SELECTION_LIST_ROWS: &[StyleRow<AwStatus>] = &[
    StyleRow {
        field: "styles::aw::selection_list.background",
        statuses: AW_SELECTION_LIST_NATIVE_STATUSES,
        native: native_aw_list_fill,
        get: |t, r, s| fill(styles::aw::selection_list(r)(t, s).background),
    },
    StyleRow {
        field: "styles::aw::selection_list.text_color",
        statuses: AW_SELECTION_LIST_NATIVE_STATUSES,
        native: native_aw_list_label,
        get: |t, r, s| Ok(styles::aw::selection_list(r)(t, s).text_color),
    },
    StyleRow {
        field: "styles::aw::selection_list.border_color",
        statuses: AW_STATUSES,
        native: |r, _| to_color(r.list.border.color),
        get: |t, r, s| Ok(styles::aw::selection_list(r)(t, s).border_color),
    },
];

/// `styles::aw::spinner` is shape B -- a spinner has no status, and the
/// container it styles takes none either -- so its rows hold the unit value.
#[cfg(feature = "iced_aw")]
pub(super) const AW_SPINNER_STATUSES: &[()] = &[()];

/// The one field of the wrapping container that carries a native value. The
/// spinner paints its circle in the color the container hands its children
/// (`container.rs:354-362`), so that color is the whole mapping;
/// `spinner.fill_color` is a required field after resolution, not a soft
/// option, so there is no fallback to write.
#[cfg(feature = "iced_aw")]
pub(super) const AW_SPINNER_ROWS: &[StyleRow<()>] = &[StyleRow {
    field: "styles::aw::spinner.text_color",
    statuses: AW_SPINNER_STATUSES,
    native: |r, ()| to_color(r.spinner.fill_color),
    get: |t, r, ()| stated(styles::aw::spinner(r)(t).text_color),
}];

#[cfg(feature = "iced_aw")]
pub(super) const AW_SELECTION_LIST_SCALAR_ROWS: &[ScalarRow<AwStatus>] = &[ScalarRow {
    field: "styles::aw::selection_list.border_width",
    statuses: AW_STATUSES,
    native: |_, r, _| r.list.border.line_width,
    get: |t, r, s| Ok(styles::aw::selection_list(r)(t, s).border_width),
}];

/// The `field` of every style row, from every function's consts.
///
/// One line per function; the coverage tripwire reads it, and `rows_claiming`
/// counts in it.
#[cfg(feature = "widgets")]
pub(super) fn style_row_fields() -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    out.extend(BUTTON_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(BUTTON_PRIMARY_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(BUTTON_DANGER_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(BUTTON_SUCCESS_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(BUTTON_WARNING_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(BUTTON_LINK_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(TEXT_INPUT_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(TEXT_EDITOR_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(CHECKBOX_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(RADIO_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(RADIO_SCALAR_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(TOGGLER_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(TOGGLER_SCALAR_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(TOGGLER_RADIUS_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(PICK_LIST_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(MENU_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(SLIDER_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(SLIDER_SCALAR_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(SCROLLABLE_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(PROGRESS_BAR_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(RULE_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(TOOLTIP_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(CONTAINER_CARD_ROWS.iter().map(|row| row.field.to_string()));
    // One border row claims six leaves, so its names are built rather than
    // written -- by the same walker the tripwire uses.
    out.extend(border_row_fields(BUTTON_BORDER_ROWS));
    out.extend(border_row_fields(BUTTON_PRIMARY_BORDER_ROWS));
    out.extend(border_row_fields(BUTTON_DANGER_BORDER_ROWS));
    out.extend(border_row_fields(BUTTON_SUCCESS_BORDER_ROWS));
    out.extend(border_row_fields(BUTTON_WARNING_BORDER_ROWS));
    out.extend(border_row_fields(TEXT_INPUT_BORDER_ROWS));
    out.extend(border_row_fields(TEXT_EDITOR_BORDER_ROWS));
    out.extend(border_row_fields(CHECKBOX_BORDER_ROWS));
    out.extend(border_row_fields(PICK_LIST_BORDER_ROWS));
    out.extend(border_row_fields(MENU_BORDER_ROWS));
    out.extend(border_row_fields(PROGRESS_BAR_BORDER_ROWS));
    out.extend(border_row_fields(TOOLTIP_BORDER_ROWS));
    out.extend(border_row_fields(CONTAINER_CARD_BORDER_ROWS));
    // `styles::aw`, gated on the feature that declares it: its rows, its
    // `check_*` calls, its walkers and its pairs all carry the same gate, so
    // the two directions of `Checked::covers` agree in every configuration.
    #[cfg(feature = "iced_aw")]
    {
        out.extend(AW_CARD_ROWS.iter().map(|row| row.field.to_string()));
        out.extend(AW_CARD_SCALAR_ROWS.iter().map(|row| row.field.to_string()));
        out.extend(AW_MENU_ROWS.iter().map(|row| row.field.to_string()));
        out.extend(AW_TAB_BAR_ROWS.iter().map(|row| row.field.to_string()));
        out.extend(
            AW_TAB_BAR_SCALAR_ROWS
                .iter()
                .map(|row| row.field.to_string()),
        );
        out.extend(
            AW_TAB_BAR_RADIUS_ROWS
                .iter()
                .map(|row| row.field.to_string()),
        );
        out.extend(AW_SIDEBAR_ROWS.iter().map(|row| row.field.to_string()));
        out.extend(
            AW_SIDEBAR_SCALAR_ROWS
                .iter()
                .map(|row| row.field.to_string()),
        );
        out.extend(
            AW_SELECTION_LIST_ROWS
                .iter()
                .map(|row| row.field.to_string()),
        );
        out.extend(
            AW_SELECTION_LIST_SCALAR_ROWS
                .iter()
                .map(|row| row.field.to_string()),
        );
        out.extend(AW_SPINNER_ROWS.iter().map(|row| row.field.to_string()));
        out.extend(border_row_fields(AW_MENU_BORDER_ROWS));
    }
    out
}
