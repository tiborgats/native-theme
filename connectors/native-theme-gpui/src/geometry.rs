//! Per-widget geometry for gpui-component 0.7.3 widgets
//! (`docs/archive/todo_v0.5.8_gpui-component-0.6-spec.md` §9, "the spec"
//! below).
//!
//! Every builder is a pure function of a [`Native`] view (the four layout
//! accessors, of a `LayoutTheme`) and returns a
//! [`StyleRefinement`] the application applies with
//! `gpui_component::StyledExt::refine_style`:
//!
//! ```ignore
//! use gpui_component::StyledExt;
//! use native_theme_gpui::{ActiveNativeTheme, geometry};
//!
//! let n = cx.native();
//! let button = Button::new("save").label("Save");
//! let button = match n { Some(n) => button.refine_style(&geometry::button(n)), None => button };
//! ```
//!
//! Each widget applies the caller's refinement after its own geometry at the
//! cited upstream line, so the values here win. Text sizes carry the
//! accessibility text-scaling factor; widths, paddings, radii and icon sizes
//! do not (spec §3.4). Every builder that sets a text size also sets the
//! platform's `defaults.line_height`. Every value is a `ResolvedTheme` field or the one
//! derivation in spec §9.4 (`scaled_text_size`), with two conversions the
//! receiving widget forces: [`tooltip_content`] turns the platform's outer
//! tooltip width into the inner box its text wraps in, and
//! [`scrollbar_gutter`] turns the scrollbar groove width the base layer
//! installs into the strip a non-overlay bar needs beside the content.
//!
//! **Padding is per side, and only what is stated.** A builder that pads sets
//! a side (`pt`, `pr`, `pb`, `pl`) only where the theme states it
//! (`ResolvedPadding`); an unstated side leaves the widget's own padding in
//! place.
//!
//! **Control heights.** [`button`], [`input`], [`select`], [`combobox`],
//! [`menu_item`] and [`list_item`] share one rule. Each applies the
//! platform's `defaults.line_height` as the control's line height, so text
//! lays out with the platform's metrics, and takes its stated height (`h`)
//! at a text-scaling factor of 1 or less, in place of upstream's own (for a
//! Select or Combobox trigger, `h_8` for `Size::Medium`, `sizing.rs:256-257`,
//! `:281-284`). Above 1 each takes the stated height as a minimum and an
//! automatic height, so layout grows the control around its drawn text and
//! padding. `tests/seams.rs` lays each control out under every native preset,
//! at its platform's DPI, and checks that the text lies inside it at factors
//! 1, 1.1 and 2. Where the theme states no height (`menu.row_height` and
//! `list.row_height` are optional, and KDE states neither), the builder sets
//! the line height alone and the row keeps the toolkit's own height. The rule
//! is for single-line controls. A `DataTable` row and header follow it through
//! [`data_table_size`] and [`list_header_height`], which compute the growth
//! themselves, a `Size` being a fixed height.
//!
//! Geometry, with one exception that is not geometry: nine builders also
//! carry the platform's text colour. A builder carries it only where the
//! carried colour actually reaches the text and upstream's state colours
//! (disabled, muted) still win — either because upstream labels the very
//! element the refinement lands on with a token of its own one refinement
//! earlier and applies its state colours after the refinement or on a child,
//! or because upstream sets no colour on that element at all, so there is
//! nothing to displace. Five of the nine displace a visibly different colour
//! today — [`status_bar`] and [`dialog_description`] displace
//! `muted_foreground`, [`tooltip`] displaces `popover_foreground`, and
//! [`input`] and [`text_area`] the window's `foreground`, which the field's
//! text inherits and the macOS presets' `input.font.color` differs from —
//! three displace `foreground`: [`list_item`], [`radio`] and [`select`], and
//! [`title_bar`] displaces nothing (`title_bar.rs:331-346` paints the fill and
//! the border, then refines). Those four change no pixel over the bundled
//! presets, because a widget that states no `font.color` inherits the window's
//! (`docs/inheritance-rules.toml`) and `foreground` is fed from the same
//! field; they carry it so a source that does state one is honoured rather
//! than silently overridden — the KDE reader states
//! `window.title_bar_font.color` from `[WM] activeForeground`, which a colour
//! scheme may contrast with the window's own text.
//!
//! [`checkbox`] and [`combobox`] are the two upstream paints from `foreground`
//! that still take no colour, because the second half of the rule fails for
//! them: a `Checkbox::label` is re-coloured by its own wrapper and both apply
//! their disabled colour before the refinement, so a carried colour would
//! either never arrive or displace the disabled one (`checkbox.rs:339-344` and
//! `:260-264`, `input/input.rs:99-103` through `combobox.rs:998`).
//!
//! `tests/seams.rs` lays real gpui-component widgets out headlessly, with and
//! without a builder's refinement, and checks what arrives: the heights of
//! [`button`], [`input`], [`select`], [`combobox`], [`list_item`] and
//! [`progress`]; [`tooltip`] and [`tooltip_content`] keeping a tooltip's text
//! inside its bubble, its wrapped lines at the platform's line height; and,
//! under every native preset at its own DPI, the drawn left padding of an
//! [`input`], a [`select`] and a [`combobox`], an [`input`]'s right padding,
//! a Textarea refined by [`input`] with its padding cleared, six single-line
//! controls -- a Button, an Input, a Select, a Combobox, a ListItem and an
//! application-drawn row under [`menu_item`] -- at their stated height with
//! their text inside them at text scales 1, 1.1 and 2, and a `DataTable` row
//! under [`data_table_size`] holding its cell's text at the same scales. It
//! also checks that [`select`] and [`combobox`]
//! keep upstream's right padding beside a stated arrow column and apply a
//! stated one without it. The other builders rest on the source citations in
//! their doc comments.
//!
//! Upstream citations in this module are verified against gpui-component 0.7.3,
//! gpui-base 0.7.3 and gpui-pre 0.3.9.

use gpui::{BoxShadow, FontWeight, Pixels, StyleRefinement, Styled, px, relative};
use gpui_component::Size;
use gpui_component::dialog::DialogEntrance;
use native_theme::theme::{DialogEntranceStyle, LayoutTheme, ResolvedFontSpec, ResolvedPadding};

use crate::Native;
use crate::colors::rgba_to_hsla;

/// `font.size × s` in pixels (spec §9.4).
fn scaled_text_size(font: &ResolvedFontSpec, n: Native<'_>) -> Pixels {
    px(crate::scaled_text_size(font.size, n.accessibility))
}

/// CSS weight (100–900) as GPUI's `FontWeight`.
fn weight_of(font: &ResolvedFontSpec) -> FontWeight {
    FontWeight(f32::from(font.weight))
}

/// Text size and weight from a font spec, at the platform's
/// `defaults.line_height`.
///
/// The line height goes wherever a text size does: gpui's own default is the
/// golden ratio (gpui-pre `src/style.rs:494`), which upstream leaves in place
/// on most text it sets (`src/tooltip.rs:110-133`, `src/status_bar.rs:86-96`)
/// and replaces with `relative(1.25)` on a dialog title before the caller's
/// refinement (`src/dialog/title.rs:44` → `:45`), so wrapped text would
/// otherwise be spaced by a metric of gpui's or upstream's own.
fn with_text(r: StyleRefinement, font: &ResolvedFontSpec, n: Native<'_>) -> StyleRefinement {
    with_line_height(r.text_size(scaled_text_size(font, n)), n).font_weight(weight_of(font))
}

/// The platform's `defaults.line_height` alone: [`with_text`]'s, for the two
/// builders that set a control's line height and no text size, [`button`]
/// and [`input_height`].
fn with_line_height(r: StyleRefinement, n: Native<'_>) -> StyleRefinement {
    r.line_height(relative(n.resolved.defaults.line_height))
}

/// Text size, weight and colour from a font spec.
///
/// For the widgets upstream labels with a token of its own *before* applying
/// the caller's refinement, on the very element the refinement lands on, and
/// whose state colours (disabled, muted) still win because upstream sets them
/// after it or on a child: there the refinement is the only carrier the
/// platform's own text colour has, because no `ThemeColor` field maps to it
/// and upstream's token would otherwise stand. Every other widget takes
/// [`with_text`], so a builder never hands an element a colour upstream did
/// not leave a place for, and never displaces a disabled colour.
fn with_coloured_text(
    r: StyleRefinement,
    font: &ResolvedFontSpec,
    n: Native<'_>,
) -> StyleRefinement {
    with_text(r, font, n).text_color(rgba_to_hsla(font.color))
}

/// Each side the theme states, and no other: an unstated side leaves the
/// receiving widget's own padding in place.
fn with_padding(r: StyleRefinement, p: &ResolvedPadding) -> StyleRefinement {
    let r = match p.top {
        Some(v) => r.pt(px(v)),
        None => r,
    };
    let r = match p.right {
        Some(v) => r.pr(px(v)),
        None => r,
    };
    let r = match p.bottom {
        Some(v) => r.pb(px(v)),
        None => r,
    };
    match p.left {
        Some(v) => r.pl(px(v)),
        None => r,
    }
}

/// The control-height rule (module doc, rationale §5 point 3).
///
/// The platform's `defaults.line_height` is the control's line height, which
/// [`with_text`] sets, or [`with_line_height`] where no text size is: each
/// receiving widget takes the refinement after its own line height
/// (`button/button.rs:743` → `:744`, `input/input.rs:783` → `:803`) or sets
/// none (`list/list_item.rs:198-209`, `select.rs:542-553`,
/// `combobox.rs:981-998`), so the platform's wins. `stated` is the control's
/// height at a text-scaling factor of 1 or less; above 1 it is its minimum
/// and the height is automatic, so layout grows the control around its drawn
/// text and padding. Without a stated height the control keeps the toolkit's
/// own height.
fn with_height_rule(r: StyleRefinement, stated: Option<f32>, n: Native<'_>) -> StyleRefinement {
    let Some(stated) = stated else {
        return r;
    };
    if n.accessibility.text_scale_factor() <= 1.0 {
        r.h(px(stated))
    } else {
        r.min_h(px(stated)).h_auto()
    }
}

/// `Button` (gpui-component `src/button/button.rs:680-717` → refined at `:744`).
/// The label's text size is set on an inner element (`:752-760`), Tier U.
///
/// Height by the control-height rule (module doc), through `h`.
#[must_use]
pub fn button(n: Native<'_>) -> StyleRefinement {
    let b = &n.resolved.button;
    let r = with_height_rule(
        with_line_height(StyleRefinement::default(), n),
        Some(b.min_height),
        n,
    );
    with_padding(r.min_w(px(b.min_width)), &b.border.padding)
        .rounded(px(b.border.corner_radius))
        .border(px(b.border.line_width))
        .border_color(rgba_to_hsla(b.border.color))
        // The weight, and deliberately not the size. This refinement lands on
        // the button's outer element (`button/button.rs:744`), and GPUI
        // cascades text style to descendants: the label is a child that sets
        // its own size from the `Size` enum (`button_text_size`, which maps
        // to `text_xs`/`text_sm`/`text_base`) and so would overrule a size
        // from here -- but it sets no weight, and nothing else on that path
        // does either, so the platform's weight arrives. A size set here would
        // be shadowed on the label and still apply to anything else inside,
        // which is worse than not setting it; that remainder is
        // `content_style`, `pub(crate)` upstream (Tier U). Since 0.7.1 the
        // size it would shadow is `text_sm` at Medium (`sizing.rs:337-343`);
        // [`button_label`] gives a child label the platform's size.
        .font_weight(weight_of(&b.font))
}

/// The text of a `Button`'s label, passed as the button's child: `button.font`'s
/// size and weight, and the platform's line height.
///
/// gpui-component sets the label's size from the `Size` enum on the content
/// element (`button_text_size`, `src/sizing.rs:337-343`, applied at
/// `src/button/button.rs:760`) after the button's own refinement (`:744`), so
/// a size given to [`button`] would be shadowed; since 0.7.1 that size is
/// `text_sm` at `Size::Medium`, 0.875 of the platform font. The content
/// element's children are the one public route under it (`:783`): a label
/// passed as a child, built like upstream's own label element (`:774-782`)
/// and refined with this. The name upstream derives from `.label()`
/// (`:748-751`) is then given explicitly:
///
/// ```ignore
/// use gpui::{ParentElement, Styled, div};
/// use gpui_component::{StyledExt, button::Button};
/// use native_theme_gpui::geometry;
///
/// Button::new("save")
///     .refine_style(&geometry::button(n))
///     .child(
///         div().min_w_0().whitespace_nowrap().text_ellipsis()
///             .refine_style(&geometry::button_label(n))
///             .child("Save"),
///     )
///     .accessibility_label("Save");
/// ```
///
/// It carries the line height [`button`] gives the button, so it is complete
/// on a Button that does not take [`button`] (a `ButtonGroup`'s child). No
/// colour: the label takes the button's state colours (idle, hovered,
/// pressed, disabled, selected) from the root, as `.label()`'s does. Labels of
/// Buttons that gpui-component's own widgets build are out of reach.
#[must_use]
pub fn button_label(n: Native<'_>) -> StyleRefinement {
    with_text(StyleRefinement::default(), &n.resolved.button.font, n)
}

/// `Toggle` (`src/button/toggle.rs:182-187` → `:222`): [`button_label`]'s
/// text — `button.font`'s size and weight and the platform's line height.
///
/// gpui-component sets `text_sm` on a `Size::Medium` Toggle's root before
/// the caller's refinement (0.875 of the platform font since 0.7.1), so this
/// size wins. Only the text: a `Toggle` is `Ghost` by default (`:15-16`), a
/// flat button, and the model states no flat-button frame, so upstream's own
/// stands. The platform's segmented control is the segmented `TabBar`, not a
/// `ToggleGroup`.
#[must_use]
pub fn toggle(n: Native<'_>) -> StyleRefinement {
    button_label(n)
}

/// `Input` root (`src/input/input.rs:788-798` → `:803`).
///
/// Height by the control-height rule (module doc), through `h`. The rule is
/// for a single-line field: a multi-line `Input` sets its own height before
/// this refinement (`input/input.rs:790-793`), which the rule's `h` would
/// replace, so a caller that wants a multi-line height sets it after this
/// builder, fully qualified as `Styled::h(input, height)`. Method syntax
/// reaches `Input::h` instead (`input/input.rs:292-296`, "Multi-line only"),
/// which shadows `Styled::h` and only records the height that upstream
/// applies at `:792`, before the caller's refinement at `:803`, so the
/// rule's `h` would still replace it. The padding is a single-line field's too:
/// upstream pads only a single-line root (`input/input.rs:784-786`), so a
/// caller that refines a multi-line `Input` or a `Textarea` uses
/// [`text_area`] instead, or clears this refinement's padding as well.
///
/// The stated padding sides reach the root: upstream pads a single-line
/// field (`input_px`/`input_py`, `input/input.rs:785`) before the refinement
/// at `:803`, so the refinement wins, and the field's text element has no
/// padding of its own to double it. One exception: an `Input` with a suffix
/// takes its right padding from upstream *after* the refinement
/// (`input/input.rs:820`, `this.pr(self.size.input_px())`), so there the
/// platform's right side does not arrive.
///
/// When refining an `InputGroup` or `NumberInput` frame, clear the padding
/// sides: the inner Input already pads (`input/group.rs:267-288` and
/// `input/number_input.rs:158-165` each render an `Input`, which pads itself
/// at `input/input.rs:784-786`).
///
/// The text is `input.font.color`. The root sets no text colour of its own
/// (`input/input.rs:707-803`; `input_style`'s foreground is dropped at
/// `:707`), and gpui-base paints the field's text in the colour the root
/// inherits (gpui-base `input/base/element.rs:2539-2542`), so without the
/// refinement it is the window's `foreground`. gpui-base halves that
/// colour's alpha in a disabled field (`:2541-2542`): [`input_fill`] for a
/// disabled field gives it `input.disabled_text_color` to halve.
#[must_use]
pub fn input(n: Native<'_>) -> StyleRefinement {
    let i = &n.resolved.input;
    let r = with_height_rule(StyleRefinement::default(), Some(i.min_height), n);
    with_coloured_text(
        with_padding(r, &i.border.padding)
            .rounded(px(i.border.corner_radius))
            .border(px(i.border.line_width)),
        &i.font,
        n,
    )
}

/// A multi-line `Input` or a `Textarea` root (`src/input/input.rs:788-798`
/// → `:803`).
///
/// [`input`]'s refinement for the multi-line field: `text_area.border`'s
/// padding, corner radius and line width in place of the input's (the
/// frame's inherit the input's, docs/platform-facts.md §2.29), and its edge in
/// `text_area.border.color`, which inherits `input.border.color`
/// (`docs/inheritance-rules.toml:173`), over the `input` token upstream draws
/// it in before the refinement (`input/input.rs:798` → `:803`); the input's
/// font at `defaults.line_height`, its colour included as in [`input`]; and no
/// height -- the rows the field's
/// state holds size it (gpui-base `input/base/state.rs`,
/// `InputBaseState::auto_grow`).
///
/// **Padding.** Upstream pads a multi-line field's editor, inside the root,
/// by its `Size`'s `input_px` and `input_py`, from render, where no caller
/// reaches it (`input/input.rs:591-605`), and pads no multi-line root
/// (`:784-786`). A `Textarea` renders its `Input` at its own `Size`
/// (`input/textarea.rs:172-177, 205`), `Size::Medium` unless the caller sizes
/// it (`:74`), whose editor padding is 10 across and 8 down
/// (`sizing.rs:147-166`); this refinement assumes that default, so a
/// textarea sized otherwise keeps its own size's editor padding under the
/// root's. This refinement pads the
/// root by the rest of each stated side, so a side at least the editor's
/// sits the stated distance inside the border; a stated side below it --
/// KDE's 5, GNOME's 0 -- stays the editor's, the one part of the platform's
/// padding no seam reaches; an unstated side is upstream's own.
#[must_use]
pub fn text_area(n: Native<'_>) -> StyleRefinement {
    let a = &n.resolved.text_area;
    let p = &a.border.padding;
    let (across, down) = (
        f32::from(Size::Medium.input_px()),
        f32::from(Size::Medium.input_py()),
    );
    let rest = |stated: Option<f32>, editor: f32| stated.map(|v| (v - editor).max(0.0));
    let root = ResolvedPadding {
        top: rest(p.top, down),
        right: rest(p.right, across),
        bottom: rest(p.bottom, down),
        left: rest(p.left, across),
    };
    with_coloured_text(
        with_padding(StyleRefinement::default(), &root)
            .rounded(px(a.border.corner_radius))
            .border(px(a.border.line_width))
            .border_color(rgba_to_hsla(a.border.color)),
        &n.resolved.input.font,
        n,
    )
}

/// A menu row the application draws with its own elements
/// (`src/menu/menu_item.rs:100-102` → `:108`).
///
/// No gpui-component widget takes this style: upstream's `MenuItemElement` is
/// crate-private in a private module (`src/menu/menu_item.rs:10-11`,
/// `src/menu/mod.rs:6`) and `PopupMenu` builds its own rows, so the receiver
/// the v0.5.8 documentation named does not exist.
///
/// Height by the control-height rule (module doc), through `h`, where
/// `menu.row_height` is stated.
#[must_use]
pub fn menu_item(n: Native<'_>) -> StyleRefinement {
    let m = &n.resolved.menu;
    let r = with_height_rule(StyleRefinement::default(), m.row_height, n);
    with_text(
        with_padding(r, &m.border.padding).gap_x(px(m.icon_text_gap)),
        &m.font,
        n,
    )
}

/// `ListItem` (`src/list/list_item.rs:201-203` → `:209`).
///
/// The colour is carried because upstream labels the row with `foreground`
/// (`:205`) one line before it applies this refinement. Today
/// `list.item_font.color` is the window's text colour in every bundled preset,
/// so this changes no pixel; it is what honours a preset that states a row
/// colour of its own.
///
/// Height by the control-height rule (module doc), through `h`, where
/// `list.row_height` is stated.
#[must_use]
pub fn list_item(n: Native<'_>) -> StyleRefinement {
    let l = &n.resolved.list;
    let r = with_height_rule(StyleRefinement::default(), l.row_height, n);
    with_coloured_text(with_padding(r, &l.border.padding), &l.item_font, n)
}

/// Application-built `Tooltip` (`src/tooltip.rs:121-126` → `:127`).
///
/// No width: `tooltip.max_width` is the bubble's outer width, and a bubble that
/// states it clamps itself without clamping its text, which then runs out of
/// it. [`tooltip_content`] carries the width to the element the application
/// passes to `Tooltip::element`, where it makes the text wrap; a tooltip built
/// from `Tooltip::new(text)` has no element to put it on and stays as wide as
/// its text.
///
/// The colours are carried because upstream paints a tooltip as a popover:
/// its text `popover_foreground` (`:116`) and its fill `tokens.popover`
/// (`:115`, `:117`), which this connector fills from `popover.font.color` and
/// `popover.background_color`, and its border `border` (`:119`), from
/// `defaults.border.color` -- all before this refinement. 18 of the 34
/// bundled preset/mode combinations state a different text colour for a
/// tooltip than for a popover, and more a different fill: kde-breeze's
/// tooltip is `#f7f7f7` on an `#eff0f1` popover, adwaita's dark on a light
/// one.
///
/// The edge is `tooltip.border.line_width` wide, over upstream's `border_1`
/// (`:118`). The drop shadow is upstream's `shadow_md` (`:120`) where
/// `tooltip.border.shadow_enabled` holds -- its two layers
/// (`TOOLTIP_SHADOW`), the model stating no shadow geometry -- in
/// `defaults.shadow_color`, and none where it does not.
#[must_use]
pub fn tooltip(n: Native<'_>) -> StyleRefinement {
    let t = &n.resolved.tooltip;
    let shadow = if t.border.shadow_enabled {
        let ink = rgba_to_hsla(n.resolved.defaults.shadow_color);
        TOOLTIP_SHADOW
            .iter()
            .map(|&(y, blur, spread)| {
                BoxShadow::new(px(0.), px(y), ink)
                    .blur_radius(px(blur))
                    .spread_radius(px(spread))
            })
            .collect()
    } else {
        Vec::new()
    };
    let mut r = with_coloured_text(
        with_padding(StyleRefinement::default(), &t.border.padding)
            .rounded(px(t.border.corner_radius))
            .bg(rgba_to_hsla(t.background_color))
            .border(px(t.border.line_width))
            .border_color(rgba_to_hsla(t.border.color)),
        &t.font,
        n,
    );
    r.box_shadow = Some(shadow);
    r
}

/// The layers of gpui's `shadow_md`, which upstream gives a tooltip
/// (`src/tooltip.rs:120`), as (y offset, blur radius, spread radius) in
/// pixels (gpui-pre-macros 0.3.9 `src/styles.rs:445-448`).
const TOOLTIP_SHADOW: [(f32, f32, f32); 2] = [(4., 6., -1.), (2., 4., -2.)];

/// The element an application passes to `Tooltip::element`
/// (`src/tooltip.rs:129-132`): `tooltip.max_width` less the bubble's own
/// horizontal paddings and its edge, `tooltip.border.line_width` on each
/// side ([`tooltip`]), which is the inner box the text wraps in.
///
/// The width has to land here and not on the bubble. The tooltip's content
/// sits in a bare `div()` inside upstream's `h_flex()`, so it is a flex item
/// with an automatic minimum size, and gpui measures text under
/// `AvailableSpace::MinContent` without wrapping it — a wrap width is taken
/// only from a *definite* available width (gpui-pre
/// `src/elements/text.rs:656-663`). The item's minimum is therefore the whole
/// unwrapped line, which a max width on the bubble cannot shrink: the bubble
/// stops at the platform's width and the text carries on past it. Given to the
/// content instead, the same width is what the text wraps at, and the bubble
/// grows to exactly `tooltip.max_width` around it.
///
/// A side the platform does not state is upstream's own `px_2`
/// (`src/tooltip.rs:124`), which is 0.5 rem (gpui-pre-macros
/// `src/styles.rs:949-950`), converted at the font size this connector
/// installs as the rem: `defaults.font.size` scaled by
/// [`crate::scaled_text_size`] (`theme.font_size` in [`crate::to_theme`],
/// which gpui-component's `WindowState` root plugin sets as the window's rem
/// size, `src/root.rs:440`).
///
/// Never negative: a platform that states a width narrower than its own
/// paddings leaves nothing for the text rather than a width gpui would reject.
///
/// Verified against a real `Tooltip` in `tests/seams.rs`.
#[must_use]
pub fn tooltip_content(n: Native<'_>) -> StyleRefinement {
    let t = &n.resolved.tooltip;
    let upstream_side = TOOLTIP_UPSTREAM_PX_REM
        * crate::scaled_text_size(n.resolved.defaults.font.size, n.accessibility);
    let left = t.border.padding.left.unwrap_or(upstream_side);
    let right = t.border.padding.right.unwrap_or(upstream_side);
    let inner = t.max_width - left - right - 2.0 * t.border.line_width;
    StyleRefinement::default().max_w(px(inner.max(0.0)))
}

/// Upstream's horizontal tooltip padding, `px_2` (`src/tooltip.rs:124`), in
/// rem: gpui's spacing suffix `2` is `rems(0.5)` (gpui-pre-macros
/// `src/styles.rs:949-950`).
const TOOLTIP_UPSTREAM_PX_REM: f32 = 0.5;

/// `Popover` (`src/popover.rs:308` → `:360`). An unstated side keeps
/// upstream's `p_3` (`:308`).
#[must_use]
pub fn popover(n: Native<'_>) -> StyleRefinement {
    let p = &n.resolved.popover;
    with_padding(StyleRefinement::default(), &p.border.padding).rounded(px(p.border.corner_radius))
}

/// `StatusBar` (`src/status_bar.rs:88-90` → `:96`).
///
/// The colour is carried because upstream labels the bar with
/// `muted_foreground` (`:95`), one line before it applies this refinement, and
/// every preset states a status-bar colour of its own that `ThemeColor` has no
/// field for. An unstated side keeps upstream's `px_2 py_1` (`:89-90`).
#[must_use]
pub fn status_bar(n: Native<'_>) -> StyleRefinement {
    let s = &n.resolved.status_bar;
    with_coloured_text(
        with_padding(StyleRefinement::default(), &s.border.padding),
        &s.font,
        n,
    )
}

/// `Dialog` (`src/dialog/dialog.rs:633-643, 723-724` → `:728`); width through
/// [`dialog_max_width`] and `Dialog::max_w`. The radius is the dialog's own
/// (`dialog.border.corner_radius`), which upstream would otherwise take from
/// `radius_lg` (`:723`); the two differ on Adwaita.
///
/// `max_h` no longer reaches upstream's `Dialog`: since 0.6.4 it clamps the
/// dialog to what is left of the viewport *after* applying this style -- in
/// 0.7.3 the view's height less twice the `lg` spacing token and the layer's
/// offset (`:616-630`), set by `max_h` at `:733`, after the refinement at
/// `:728`.
/// `min_h` and the paddings still arrive, because `min_h_24()` runs before the
/// refinement. Kept here for an application-drawn dialog and for the day
/// upstream takes a `max_h` prop
/// (`docs/archive/todo_v0.5.9_gpui-kit-0.6.4-spec.md` §4).
///
/// Upstream reads the four padding sides back out of this style
/// (`dialog/dialog.rs:635-647`), starting from 16px on each side, so
/// an unstated side keeps that 16px. It also reuses them as gaps: the gap
/// between the dialog's sections is `max(top, 8px)` (`:727`), and a
/// `DialogContent`'s gap is the bottom padding (`:758`). A platform that
/// states 32 top and 24 bottom, as GNOME does (docs/platform-facts.md §2.22),
/// therefore also spaces the sections by 32 and the content by 24.
#[must_use]
pub fn dialog(n: Native<'_>) -> StyleRefinement {
    let d = &n.resolved.dialog;
    with_padding(StyleRefinement::default(), &d.border.padding)
        .min_h(px(d.min_height))
        .max_h(px(d.max_height))
        .rounded(px(d.border.corner_radius))
}

/// For `Dialog::entrance` and `AlertDialog::entrance`
/// (`src/dialog/dialog.rs:526`, `src/dialog/alert_dialog.rs:277`): the
/// entrance closest to `dialog.entrance` that gpui-component draws, or `None`
/// where the theme states none, so the dialog keeps upstream's default,
/// `DialogEntrance::SlideDown`.
///
/// Both styles are approximations, and both become `DialogEntrance::Fade`, a
/// fade in place over the motion tokens' `duration_normal`
/// (`src/dialog/dialog.rs:653-666, 864-867`):
///
/// - `ScaleFade` (Windows, GNOME, Material) loses its scale: gpui-component
///   has no scaling entrance (`src/dialog/entrance.rs:12-25`).
/// - `FadeRise` (KDE) loses its rise: gpui-component has no rising entrance,
///   and its `FadeSlide` settles from just above the final position, the
///   opposite direction.
///
/// Reduced motion skips every entrance upstream (`:621-623`).
#[must_use]
pub fn dialog_entrance(n: Native<'_>) -> Option<DialogEntrance> {
    n.resolved.dialog.entrance.map(|style| match style {
        DialogEntranceStyle::ScaleFade | DialogEntranceStyle::FadeRise => DialogEntrance::Fade,
    })
}

/// `InputGroupButton` (`src/input/group.rs:597-600` → `:603`): the platform's
/// button radius, and nothing else.
///
/// gpui-component scales a control's radius with its size — an in-group
/// button is `XSmall` and gets `radius / 2`, `Button` does the same for its
/// small and large roundings (`src/button/button.rs:647-649`) — where
/// native-theme records one radius per widget, whatever its size. [`button`]
/// already restores it for a `Button`; this does the same for the button
/// nested in a field, whose height, width and padding the group sets so that
/// it fits, and which [`button`]'s own height would push out of the field.
#[must_use]
pub fn input_group_button(n: Native<'_>) -> StyleRefinement {
    StyleRefinement::default().rounded(px(n.resolved.button.border.corner_radius))
}

/// `DialogFooter` (`src/dialog/footer.rs:52` → `:56`).
#[must_use]
pub fn dialog_footer(n: Native<'_>) -> StyleRefinement {
    StyleRefinement::default().gap(px(n.resolved.dialog.button_gap))
}

/// `DialogTitle` (`src/dialog/title.rs:42-43` → `:45`): the platform's line
/// height in place of upstream's `relative(1.25)` (`:44`).
#[must_use]
pub fn dialog_title(n: Native<'_>) -> StyleRefinement {
    with_text(StyleRefinement::default(), &n.resolved.dialog.title_font, n)
}

/// `DialogDescription` (`src/dialog/description.rs:49` → `:51`).
///
/// The colour is carried because upstream labels the description with
/// `muted_foreground` (`:50`), and every preset states a dialog body colour
/// that is not the window's muted colour.
#[must_use]
pub fn dialog_description(n: Native<'_>) -> StyleRefinement {
    with_coloured_text(StyleRefinement::default(), &n.resolved.dialog.body_font, n)
}

/// The frame of a list view: `list.border`'s line width, colour and radius,
/// and a clip to that radius.
///
/// Neither `List` (`src/list/list.rs`, `RenderOnce for List<D>`) nor `Tree`
/// (`src/tree.rs`, `RenderOnce for Tree`) paints a border of its own: each
/// refines a plain `div()` and leaves the frame to the application, where
/// `DataTable` draws one from `Theme::radius` and `Theme::border` when it is
/// `bordered` (`src/table/data_table.rs:181-185`). Apply this to a `List` or a
/// `Tree` — both are `Styled` and the refinement lands on that outer `div` —
/// or to the box an application draws around one, and the three agree. There
/// is no tree theme in the model: a tree is a list view, and reads
/// `resolved.list`.
///
/// The clip is part of the frame, not decoration: a row's selected or hovered
/// fill is a square that would otherwise show through the rounded corners.
#[must_use]
pub fn list(n: Native<'_>) -> StyleRefinement {
    let b = &n.resolved.list.border;
    StyleRefinement::default()
        .border(px(b.line_width))
        .border_color(rgba_to_hsla(b.color))
        .rounded(px(b.corner_radius))
        .overflow_hidden()
}

/// Declarative `Table` (`src/table/table.rs:111` → `:114`); rows are inner, Tier U.
#[must_use]
pub fn table(n: Native<'_>) -> StyleRefinement {
    with_text(StyleRefinement::default(), &n.resolved.list.item_font, n)
}

/// `Progress` (`src/progress/progress.rs:128-129` → `:130`); the fill copies
/// the caller's radii (`:92-94, 137, 147`), so height and radius are exact.
#[must_use]
pub fn progress(n: Native<'_>) -> StyleRefinement {
    let p = &n.resolved.progress_bar;
    StyleRefinement::default()
        .h(px(p.track_height))
        .rounded(px(p.border.corner_radius))
        .min_w(px(p.min_width))
}

/// For `GroupBox::content_style` (`src/group_box.rs:107`), applied to the
/// content surface (`:172-177`), which 0.7.3 nests in a column that also
/// holds the optional footer (`:167-168`).
#[must_use]
pub fn group_box_content(n: Native<'_>) -> StyleRefinement {
    let c = &n.resolved.card;
    with_padding(StyleRefinement::default(), &c.border.padding)
        .rounded(px(c.border.corner_radius))
        .border(px(c.border.line_width))
        .border_color(rgba_to_hsla(c.border.color))
}

/// For `AccordionItem::title_style` (`src/accordion.rs:220`, applied `:301-307`).
#[must_use]
pub fn accordion_title(n: Native<'_>) -> StyleRefinement {
    StyleRefinement::default().h(px(n.resolved.expander.header_height))
}

/// The metrics `Checkbox` and `Radio` share; platform-facts §2.5 defines radio
/// metrics as the checkbox's with a circular indicator, so reading the
/// checkbox's fields for a radio is what the platform states rather than one
/// widget's value standing in for another's. Only the text colour separates
/// the two.
fn checkbox_metrics(n: Native<'_>) -> StyleRefinement {
    StyleRefinement::default().gap(px(n.resolved.checkbox.label_gap))
}

/// `Checkbox` (`src/checkbox.rs:278-290` → `:291`); the indicator is inner,
/// Tier U. The text size replaces upstream's `input_text_size` (`:283-286`:
/// XSmall `text_xs`, Small and Medium `text_sm`, Large `text_base`).
///
/// No colour, although upstream does label the row with `foreground` (`:282`)
/// before this refinement: a `Checkbox::label` is wrapped in a div that sets
/// `foreground` itself (`:339-344`), so a carried colour would never reach it,
/// and the disabled hook applies `muted_foreground` and only *then* the
/// refinement (`:260-264`), so a carried colour would displace the disabled
/// colour of custom children. [`radio`], whose label child sets no colour of
/// its own, carries it.
///
/// The platform's line height lands on the row, over upstream's
/// `relative(1.)` (`:281`); the label column sets its own `relative(1.25)`
/// (`:334`), which stands, and on which upstream centres the indicator
/// (`:298-299`).
#[must_use]
pub fn checkbox(n: Native<'_>) -> StyleRefinement {
    with_text(checkbox_metrics(n), &n.resolved.checkbox.font, n)
}

/// `Radio` (`src/radio.rs:218-230` → `:231`): [`checkbox`]'s metrics, and the
/// colour [`checkbox`] cannot take. The text size replaces upstream's
/// `input_text_size` (`:227-230`), the same ladder as [`checkbox`]'s.
///
/// The colour is carried for the same reason as [`list_item`]: upstream labels
/// the row with `foreground` (`:220`) before applying this refinement, and its
/// disabled muting is on the label child (`:261-262`), which wins over
/// whatever the row carries. The line height, as for [`checkbox`], lands on
/// the row (over `:222`), and the label column keeps its own `relative(1.25)`
/// (`:255`).
#[must_use]
pub fn radio(n: Native<'_>) -> StyleRefinement {
    with_coloured_text(checkbox_metrics(n), &n.resolved.checkbox.font, n)
}

/// The metrics `Select` and `Combobox` share; only the text colour separates
/// the two.
///
/// Height by the control-height rule (module doc): the stated height at a
/// text-scaling factor of 1 or less replaces upstream's own `h_8` for
/// `Size::Medium` (`input_size`, `sizing.rs:256-257`, `:281-284`), which the
/// trigger sets before the refinement (`select.rs:551` → `:553`,
/// `combobox.rs:996` → `:998`).
///
/// The stated padding sides reach the trigger: upstream pads it
/// (`input_size`, `select.rs:551`, `combobox.rs:996`) before the refinement
/// (`select.rs:553`, `combobox.rs:998`), so the refinement wins, and no inner
/// element pads again. The right side is the exception where the theme
/// states `combo_box.arrow_area_width`: the platform then measures its right
/// side to a separate arrow column (WinUI's 38px column, Breeze's 20px one;
/// docs/platform-facts.md §2.24), and gpui's trigger has none — its caret
/// sits inside the padded row (`select.rs:599-600` in the row at
/// `:564-606`, `combobox.rs:655-656` in the row at `:1009-1028`). A right
/// side measured to a column the trigger does not have is not applied, and
/// upstream's own right padding (`input_px`, `select.rs:551`,
/// `combobox.rs:996`) stands. Without an arrow column the stated right side
/// is measured to the text, and is applied like the others.
fn combo_box_metrics(n: Native<'_>) -> StyleRefinement {
    let c = &n.resolved.combo_box;
    let r = with_height_rule(StyleRefinement::default(), Some(c.min_height), n);
    let padding = match c.arrow_area_width {
        Some(_) => ResolvedPadding {
            right: None,
            ..c.border.padding
        },
        None => c.border.padding,
    };
    with_padding(r, &padding)
        .min_w(px(c.min_width))
        .rounded(px(c.border.corner_radius))
}

/// `Select` (`src/select.rs:542-552` → `:553`); the arrow is inner, Tier U.
/// Height by the control-height rule (module doc), through `h`, in place of
/// upstream's `h_8` at a text-scaling factor of 1 or less.
///
/// The stated padding sides reach the trigger: upstream pads it
/// (`input_size`, `select.rs:551`) before the refinement (`:553`), so the
/// refinement wins, and no inner element pads again. Where the theme states
/// `combo_box.arrow_area_width`, the right side is not applied and
/// upstream's own stands: the platform measures it to an arrow column, and
/// the caret sits inside the padded trigger (`select.rs:599-600`, in the row
/// at `:564-606`), which has none (see the shared metrics, and
/// docs/platform-facts.md §2.24).
///
/// The colour is carried for the same reason as [`list_item`]: upstream labels
/// the trigger with `foreground` through `input_style`
/// (`src/input/input.rs:105`, applied at `select.rs:546`) before applying this
/// refinement. It is safe here although the same helper's disabled branch
/// returns `muted_foreground` before the refinement too
/// (`src/input/input.rs:99-103`), because `Select` re-mutes its title *child*
/// when disabled (`select.rs:484-486`) and a child wins over the trigger.
#[must_use]
pub fn select(n: Native<'_>) -> StyleRefinement {
    with_coloured_text(combo_box_metrics(n), &n.resolved.combo_box.font, n)
}

/// `Combobox` (`src/combobox.rs:981-997` → `:998`): [`select`]'s metrics, and
/// the colour [`select`] can take but this cannot.
///
/// The metrics are the height and the padding sides, and the right side
/// follows the same rule: beside a stated `combo_box.arrow_area_width` it is
/// upstream's own, because the caret sits inside the padded trigger
/// (`combobox.rs:655-656`, in the row at `:1009-1028`).
///
/// Upstream labels the trigger with the same `input_style` `foreground`
/// (`:991`), but its disabled branch (`src/input/input.rs:99-103`) delivers
/// `muted_foreground` through that same call, before the refinement at `:998`,
/// and the selected-title child (`:585-591`) sets no colour to re-mute with —
/// so a carried colour would beat the disabled colour instead of yielding to
/// it.
#[must_use]
pub fn combobox(n: Native<'_>) -> StyleRefinement {
    with_text(combo_box_metrics(n), &n.resolved.combo_box.font, n)
}

/// `TitleBar` (`src/title_bar.rs:338` → `:346`); the height has no theme field.
///
/// Upstream sets no text colour on the bar (`:331-346` paints the fill and the
/// border, then refines), so the route is open and the carried colour displaces
/// nothing. It is worth carrying: the KDE reader states
/// `window.title_bar_font.color` from `[WM] activeForeground`, which a colour
/// scheme is free to contrast with the window's own text.
#[must_use]
pub fn title_bar(n: Native<'_>) -> StyleRefinement {
    with_coloured_text(
        StyleRefinement::default(),
        &n.resolved.window.title_bar_font,
        n,
    )
}

/// A toolbar row (docs/platform-facts.md §2.13), the application's own or
/// gpui-component's `Toolbar`: its height the content's, at least
/// `toolbar.bar_height` where the platform states one -- KDE's and GNOME's
/// toolbars size to their content and state none (platform-facts.md:1352),
/// and a fixed height would be an invention there, so the refinement sets
/// the height to `auto`, which also replaces the `h_8` gpui-component's
/// `Toolbar` sets at its Small size (toolbar.rs:252-256); an application that
/// wants a fixed height sets one after the refinement --,
/// `toolbar.item_gap` between items where the theme states one, the
/// `toolbar.border` padding sides the platform states,
/// `toolbar.background_color`, and `toolbar.font` size and weight. An
/// unstated side or gap is left to the application. No edge: §2.13 states
/// none; an application that wants a rule draws a Separator.
#[must_use]
pub fn toolbar(n: Native<'_>) -> StyleRefinement {
    let t = &n.resolved.toolbar;
    let r = with_text(StyleRefinement::default(), &t.font, n).h_auto();
    let r = match t.bar_height {
        Some(h) => r.min_h(px(h)),
        None => r,
    };
    let r = match t.item_gap {
        Some(g) => r.gap(px(g)),
        None => r,
    };
    with_padding(r, &t.border.padding).bg(rgba_to_hsla(t.background_color))
}

// --- Size helpers (spec §9.3) -------------------------------------------------

/// `Spinner::with_size` (`src/spinner.rs:53-65` → `Icon`, `src/icon.rs:188`;
/// 0.6.4 merged the two sizing arms 0.6.0 had at `:166` and `:195`).
#[must_use]
pub fn spinner_size(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.spinner.diameter))
}

/// `DataTable::with_size`: `list.row_height` as `Size::Size`, or upstream's own
/// Medium row height where the theme states none. Above a text-scaling factor
/// of 1 the row is at least the scaled `list.item_font`'s line box at
/// `defaults.line_height`, rounded up, and upstream's own top and bottom cell
/// padding (`Size::table_cell_padding`, `src/sizing.rs:89-94`): the
/// control-height rule (module doc), computed here because upstream gives
/// every row this height (`src/table/state.rs:2047`, `:2061`) and clips each
/// cell to it (`:1406-1412`).
///
/// Under `Size::Size` gpui-component sets the row height verbatim
/// (`src/sizing.rs:57-64`) and the cells no text size (`:323-333`), so they
/// inherit — refine the table's container with [`table`] and they take
/// `list.item_font`. The Medium cell padding stands (`:89-94`). At
/// `Size::Medium` the cells are `text_sm` since 0.7.1: 0.875 of the platform
/// font.
#[must_use]
pub fn data_table_size(n: Native<'_>) -> Size {
    let l = &n.resolved.list;
    let stated = l
        .row_height
        .map_or_else(|| Size::Medium.table_row_height(), px);
    Size::Size(table_row_height(stated, &l.item_font, n))
}

/// For `DataTable::header_row_height` (`src/table/data_table.rs:129`):
/// `list.header_height`, grown above a text-scaling factor of 1 as
/// [`data_table_size`] grows a row, or `None` where the theme states none, so
/// the header keeps the body rows' height, which upstream gives an unset
/// header (`src/table/state.rs:775-779`) and [`data_table_size`] sets.
///
/// It grows with `list.item_font`, the font the header's text is drawn in: a
/// `Size::Size` header cell sets no text size (`src/table/state.rs:1681`
/// through `render_cell`, `src/sizing.rs:323-333`), so it inherits the
/// container's, which [`table`] sets; `list.header_font` reaches only the
/// header's text colour (`ThemeColor::table_head_foreground`).
#[must_use]
pub fn list_header_height(n: Native<'_>) -> Option<Pixels> {
    let l = &n.resolved.list;
    l.header_height
        .map(|h| table_row_height(px(h), &l.item_font, n))
}

/// A `DataTable` row's height by the control-height rule (module doc): at a
/// text-scaling factor of 1 or less `stated`, verbatim; above 1 at least the
/// line box of `font`'s scaled size at `defaults.line_height`, rounded up,
/// and upstream's own top and bottom cell padding for `Size::Size`. A `Size`
/// is a number, so the growth the single-line controls get from an automatic
/// height is computed here.
fn table_row_height(stated: Pixels, font: &ResolvedFontSpec, n: Native<'_>) -> Pixels {
    if n.accessibility.text_scale_factor() <= 1.0 {
        return stated;
    }
    let padding = Size::Size(stated).table_cell_padding();
    let line = (f32::from(scaled_text_size(font, n)) * n.resolved.defaults.line_height).ceil();
    let fit = line + f32::from(padding.top) + f32::from(padding.bottom);
    px(f32::from(stated).max(fit))
}

/// `Icon::with_size` for toolbar icons: `toolbar.icon_size`, which inherits
/// `defaults.icon_sizes.toolbar` where a platform states no toolbar-specific size.
#[must_use]
pub fn icon_size_toolbar(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.toolbar.icon_size))
}

/// `Icon::with_size` for small icons (`defaults.icon_sizes.small`).
#[must_use]
pub fn icon_size_small(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.defaults.icon_sizes.small))
}

/// `Icon::with_size` for large icons (`defaults.icon_sizes.large`).
#[must_use]
pub fn icon_size_large(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.defaults.icon_sizes.large))
}

/// `Icon::with_size` for dialog icons (`defaults.icon_sizes.dialog`).
#[must_use]
pub fn icon_size_dialog(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.defaults.icon_sizes.dialog))
}

/// `Icon::with_size` for panel icons (`defaults.icon_sizes.panel`).
#[must_use]
pub fn icon_size_panel(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.defaults.icon_sizes.panel))
}

// --- Scroll containers --------------------------------------------------------

/// The strip a vertical scrollbar needs beside the content it scrolls: the
/// platform's groove width as right padding where its scrollbars are not
/// overlays, and nothing where they are.
///
/// gpui-component overlays its scrollbar on the scroll area whatever the
/// platform does — `Scrollable` "renders the original element as the scroll
/// area and overlays scrollbars" (`src/scroll/scrollable.rs`) — and gpui-base
/// draws the vertical track flush with that area's right edge, at the track
/// width this connector installed for it (gpui-base
/// `src/scrollbar.rs:1408-1432`). Where `scrollbar.overlay_mode` is false —
/// KDE and Windows — [`crate::apply`] also asks for an always-visible bar, so
/// a content element that fills the scroll area runs underneath it. The width
/// is read back from [`crate::base_layer::scrollbar_geometry`], the one that
/// was written onto gpui-base, so the two cannot drift apart.
///
/// Apply it to the element the container scrolls, not around the container:
/// `overflow_y_scrollbar` keeps the caller's own element as the scroll content
/// (`src/scroll/scrollable.rs`, `Scrollable::render`), so padding on it is
/// padding inside the scrolled box and the bar lands beside it.
#[must_use]
pub fn scrollbar_gutter(n: Native<'_>) -> StyleRefinement {
    if n.resolved.scrollbar.overlay_mode {
        return StyleRefinement::default();
    }
    let width = crate::base_layer::scrollbar_geometry(n.resolved).track_width;
    StyleRefinement::default().pr(width)
}

// --- Builder helpers (spec §9.3) ----------------------------------------------

/// For `Dialog::max_w` (`src/dialog/dialog.rs:497`).
#[must_use]
pub fn dialog_max_width(n: Native<'_>) -> Pixels {
    px(n.resolved.dialog.max_width)
}

/// An `Input`'s height rule alone: the line height and height [`input`]
/// sets, and nothing else of its refinement.
///
/// A single-line `Input` takes it through `StyledExt::refine_style`, the
/// caller's style, which `Input` refines its root with last
/// (`src/input/input.rs:803`). Above a text-scaling factor of 1 the rule's
/// height is automatic, so the field then grows around its own drawn text
/// and padding -- upstream's, unless the caller also gives it the
/// platform's.
#[must_use]
pub fn input_height(n: Native<'_>) -> StyleRefinement {
    with_height_rule(
        with_line_height(StyleRefinement::default(), n),
        Some(n.resolved.input.min_height),
        n,
    )
}

/// The fill of an `Input` root (`src/input/input.rs:707-718`, painted at
/// `:795`, refined at `:803`), for a field built with `disabled`: the
/// platform's `input.background_color`, or disabled its
/// `input.disabled_background` and its text in `input.disabled_text_color`.
/// Apply it after [`input`].
///
/// Colour, not geometry, and carried for the reason the builders that carry a
/// text colour carry it (module doc): no `ThemeColor` field maps to it. Upstream fills an enabled
/// field with `Theme::input_background`, which is the window's `background`
/// in a light theme and `input` mixed with transparent in a dark one
/// (`theme/mod.rs:485-491`), and a disabled one with `input` mixed with
/// transparent at half opacity (`input/input.rs:98-101`, `:714-718`); the
/// `input` token is the field's border (`contract.rs`'s `input` row), so
/// neither fill can be the platform's. The fill is painted before the
/// caller's refinement, so the refinement's wins -- in either state, which is
/// why the caller passes the state the field is built with: a field refined
/// with the enabled fill would lose its disabled look. Disabled, the field is
/// also faded by `input.disabled_opacity`: a platform dims by its disabled
/// colours or by that opacity, and its data makes the other an identity
/// (docs/platform-facts.md §2.1.6), so a platform that states no disabled
/// fill (`input.disabled_background` is `None`) gets the enabled fill, faded.
///
/// The disabled text colour replaces [`input`]'s `input.font.color` on the
/// root, whose colour gpui-base paints the field's text in; it feeds
/// gpui-base's dim rather than replacing it, since the editor halves the
/// alpha of a disabled field's text whatever its colour (gpui-base
/// `input/base/element.rs:2541-2542`), and the field's
/// `input.disabled_opacity` fades it with the rest of the field.
#[must_use]
pub fn input_fill(n: Native<'_>, disabled: bool) -> StyleRefinement {
    let i = &n.resolved.input;
    if disabled {
        StyleRefinement::default()
            .bg(rgba_to_hsla(
                i.disabled_background.unwrap_or(i.background_color),
            ))
            .text_color(rgba_to_hsla(i.disabled_text_color))
            .opacity(unit(i.disabled_opacity))
    } else {
        StyleRefinement::default().bg(rgba_to_hsla(i.background_color))
    }
}

/// An opacity for gpui: clamped to 0–1, and 1 (no fade) where it is not a
/// finite number.
fn unit(opacity: f32) -> f32 {
    if opacity.is_finite() {
        opacity.clamp(0., 1.)
    } else {
        1.
    }
}

/// The fill and label colour of a disabled `Button`: the platform's
/// `button.disabled_background`, or `button.background_color` where it states
/// none, and `button.disabled_text_color`, the whole button faded by
/// `button.disabled_opacity` (docs/platform-facts.md §2.1.6: the data makes
/// the mechanism the platform does not use an identity). Apply it after
/// [`button`], to a Button built disabled, only.
///
/// Colour, not geometry, carried because no `ThemeColor` field reaches it:
/// upstream paints a disabled Default button with `input_background()` at
/// half opacity and its label with `muted_foreground` at half opacity
/// (`src/button/button.rs:1333-1379`, the label at `:1344`), literals of its
/// own. Its disabled
/// style replays the caller's refinement over its own (`:808-814`), so the
/// refinement's colours win there -- and at rest too (`:744`), which is why
/// an enabled Button must not take it.
#[must_use]
pub fn button_disabled(n: Native<'_>) -> StyleRefinement {
    let b = &n.resolved.button;
    StyleRefinement::default()
        .bg(rgba_to_hsla(
            b.disabled_background.unwrap_or(b.background_color),
        ))
        .text_color(rgba_to_hsla(b.disabled_text_color))
        .opacity(unit(b.disabled_opacity))
}

/// The fill and label colour of a toggle `Button` that is on
/// (`Button::selected(true)`): `button.checked_background` and
/// `button.checked_text_color`, or, where the theme states none, the pressed
/// pair, `button.active_background` and `button.active_text_color`
/// (docs/platform-facts.md §2.3: Breeze, WinUI, libadwaita and Material state
/// a checked pair, AppKit none). Where neither fill is stated the refinement
/// sets none, and upstream's selected fill stands.
///
/// Colour, not geometry, carried because no `ThemeColor` field reaches it:
/// upstream fills a selected Default button with its `button_active` token
/// and letters it in `button_foreground` (`src/button/button.rs`,
/// `ButtonVariant::selected`), and replays the caller's refinement over that
/// state, so the refinement's colours win. Apply it after [`button`], to a
/// selected Button only.
#[must_use]
pub fn button_checked(n: Native<'_>) -> StyleRefinement {
    let b = &n.resolved.button;
    let r = StyleRefinement::default().text_color(rgba_to_hsla(
        b.checked_text_color.unwrap_or(b.active_text_color),
    ));
    match b.checked_background.or(b.active_background) {
        Some(fill) => r.bg(rgba_to_hsla(fill)),
        None => r,
    }
}

/// `Link` (`src/link.rs:70-90`): `link.underline_enabled`, and the link's
/// text size and weight.
///
/// Upstream underlines a link at rest (`text_decoration_1`, `link.rs:77-78`)
/// before the caller's refinement (`:90`). Where the platform draws its links
/// without an underline (`link.underline_enabled` false), the refinement sets
/// the underline's thickness to zero, so none is painted; where it underlines
/// them, upstream's underline stands. The underline's colour, and the one
/// upstream adds on hover and press (`:79-88`), are upstream's: the model
/// states neither. The text colour is `ThemeColor::link`, the
/// `link.font.color` row.
#[must_use]
pub fn link(n: Native<'_>) -> StyleRefinement {
    let l = &n.resolved.link;
    let r = with_text(StyleRefinement::default(), &l.font, n);
    if l.underline_enabled {
        r
    } else {
        r.text_decoration_0()
    }
}

// --- Layout accessors (spec §9.5) ---------------------------------------------
// The input is `Theme::layout` on the preset path and `SystemTheme.layout` on
// the system path. There is no receiver to map these into: gpui-component's
// `Theme::spacing_tokens()` returns `SpacingTokens::default()` with no field
// behind it (`theme/mod.rs:588-590`), so its one reader, a `Dialog`'s viewport
// margin (`dialog/dialog.rs:616`), gets that default whatever a theme says.
// `None` means the platform specifies nothing (platform-facts §2.20).

/// Space between adjacent widgets.
#[must_use]
pub fn widget_gap(layout: &LayoutTheme) -> Option<Pixels> {
    layout.widget_gap.map(px)
}

/// Padding inside containers.
#[must_use]
pub fn container_margin(layout: &LayoutTheme) -> Option<Pixels> {
    layout.container_margin.map(px)
}

/// Padding inside the main window.
#[must_use]
pub fn window_margin(layout: &LayoutTheme) -> Option<Pixels> {
    layout.window_margin.map(px)
}

/// Space between major content sections.
#[must_use]
pub fn section_gap(layout: &LayoutTheme) -> Option<Pixels> {
    layout.section_gap.map(px)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::colors::rgba_to_hsla;
    use crate::{ColorMode, ResolvedTheme, Theme};
    use gpui::{AbsoluteLength, DefiniteLength, Length};
    use native_theme::AccessibilityPreferences;

    fn resolved(preset: &str, mode: ColorMode) -> ResolvedTheme {
        Theme::preset(preset)
            .expect("preset")
            .into_variant(mode)
            .expect("variant")
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .expect("resolves")
    }
    fn scaled(factor: f32) -> AccessibilityPreferences {
        let mut prefs = AccessibilityPreferences::default();
        prefs.text_scaling_factor = factor;
        prefs
    }
    fn len(v: f32) -> Option<Length> {
        Some(px(v).into())
    }
    fn def(v: f32) -> Option<DefiniteLength> {
        Some(px(v).into())
    }
    fn abs(v: f32) -> Option<AbsoluteLength> {
        Some(px(v).into())
    }
    /// A stated side as the refinement carries it; an unstated one is unset.
    fn side(v: Option<f32>) -> Option<DefiniteLength> {
        v.map(|v| px(v).into())
    }
    /// Asserts the refinement carries exactly the stated sides, and returns
    /// how many sides were stated: comparing `None` with `None` checks nothing,
    /// so a caller has to see that some side was compared as `Some`.
    #[must_use]
    fn assert_padding(out: &StyleRefinement, p: &ResolvedPadding, what: &str) -> usize {
        assert_eq!(out.padding.top, side(p.top), "{what}: top");
        assert_eq!(out.padding.right, side(p.right), "{what}: right");
        assert_eq!(out.padding.bottom, side(p.bottom), "{what}: bottom");
        assert_eq!(out.padding.left, side(p.left), "{what}: left");
        [p.top, p.right, p.bottom, p.left]
            .iter()
            .filter(|v| v.is_some())
            .count()
    }
    /// The stated values a test's assertions compared, per builder and kind,
    /// over its cases: padding sides through [`assert_padding`], optional
    /// heights through [`assert_height_rule`].
    #[derive(Default)]
    struct Stated(std::collections::BTreeMap<(String, &'static str), usize>);
    impl Stated {
        fn padding(&mut self, out: &StyleRefinement, p: &ResolvedPadding, what: &str) {
            *self.0.entry((what.to_owned(), "padding side")).or_default() +=
                assert_padding(out, p, what);
        }
        fn height(
            &mut self,
            out: &StyleRefinement,
            r: &ResolvedTheme,
            stated: Option<f32>,
            s: f32,
            what: &str,
        ) {
            assert_height_rule(out, r, stated, s, what);
            *self.0.entry((what.to_owned(), "height")).or_default() +=
                usize::from(stated.is_some());
        }
        /// Fails for a builder whose cases stated none of a kind: its
        /// assertions of that kind compared `None` with `None` and tested
        /// nothing.
        fn assert_each_compared(&self) {
            for ((what, kind), n) in &self.0 {
                assert!(
                    *n > 0,
                    "{what}: no case states a {kind}, so the builder's {kind} \
                     was compared with nothing; add a case that states one"
                );
            }
        }
    }
    /// The control-height rule: the platform's line height; the stated
    /// height at s <= 1, above 1 a stated minimum and an automatic height.
    fn assert_height_rule(
        out: &StyleRefinement,
        r: &ResolvedTheme,
        stated: Option<f32>,
        s: f32,
        what: &str,
    ) {
        assert_eq!(
            out.text.line_height,
            Some(relative(r.defaults.line_height)),
            "{what}: the platform's line height"
        );
        let Some(stated) = stated else {
            assert_eq!(out.size.height, None, "{what}: no stated height");
            assert_eq!(out.min_size.height, None, "{what}: no stated minimum");
            return;
        };
        if s <= 1.0 {
            assert_eq!(out.size.height, len(stated), "{what}: height");
            assert_eq!(out.min_size.height, None, "{what}: no minimum");
        } else {
            assert_eq!(
                out.min_size.height,
                len(stated),
                "{what}: min height above 1"
            );
            assert_eq!(
                out.size.height,
                Some(Length::Auto),
                "{what}: automatic height above 1"
            );
        }
    }

    /// The catppuccin pair states no padding, `row_height` or `item_gap`;
    /// windows-11 and kde-breeze state padding sides, and windows-11 both row
    /// heights, so each builder is compared with stated values as well as
    /// with unstated ones.
    const CASES: &[(&str, ColorMode)] = &[
        ("catppuccin-mocha", ColorMode::Dark),
        ("catppuccin-latte", ColorMode::Light),
        ("windows-11", ColorMode::Light),
        ("kde-breeze", ColorMode::Dark),
    ];
    const FACTORS: &[f32] = &[1.0, 1.5, 0.8];

    /// Runs `check` for every case and every factor with a `Native` view.
    fn for_each_case(mut check: impl FnMut(&ResolvedTheme, f32, Native<'_>)) {
        for (preset, mode) in CASES {
            let r = resolved(preset, *mode);
            for &s in FACTORS {
                let prefs = scaled(s);
                check(
                    &r,
                    s,
                    Native {
                        resolved: &r,
                        accessibility: &prefs,
                    },
                );
            }
        }
    }

    /// A text size, weight and the platform's line height, which every builder
    /// that sets a text size carries.
    fn assert_text(out: &StyleRefinement, r: &ResolvedTheme, font: &ResolvedFontSpec, s: f32) {
        assert_eq!(out.text.font_size, abs(font.size * s));
        assert_eq!(out.text.line_height, Some(relative(r.defaults.line_height)));
        assert_eq!(
            out.text.font_weight,
            Some(FontWeight(f32::from(font.weight)))
        );
    }

    #[test]
    fn button_refinement_matches_theme_values() {
        let mut stated = Stated::default();
        for_each_case(|r, s, n| {
            let b = &r.button;
            let out = button(n);
            assert_height_rule(&out, r, Some(b.min_height), s, "button");
            assert_eq!(out.min_size.width, len(b.min_width));
            stated.padding(&out, &b.border.padding, "button");
            assert_eq!(
                out.corner_radii.top_left,
                abs(b.border.corner_radius.max(0.0))
            );
            assert_eq!(out.border_widths.top, abs(b.border.line_width));
            assert_eq!(out.border_color, Some(rgba_to_hsla(b.border.color)));
            assert_eq!(
                out.text.font_size, None,
                "button label size is inner (Tier U), not set here"
            );
            // The weight is not inner: nothing on the label's path sets one,
            // so this one cascades. The showcase's ten Button panels called it
            // "hardcoded" for as long as the connector declined to carry it.
            assert_eq!(
                out.text.font_weight,
                Some(FontWeight(f32::from(b.font.weight))),
                "the platform states a button font weight and nothing upstream \
                 overrides it, so the builder carries it"
            );
        });
        stated.assert_each_compared();
    }

    #[test]
    fn button_label_carries_the_button_font() {
        for_each_case(|r, s, n| {
            let out = button_label(n);
            assert_text(&out, r, &r.button.font, s);
            assert_eq!(out.text.line_height, Some(relative(r.defaults.line_height)));
            assert_eq!(
                out.text.color, None,
                "the label takes the button's state colours"
            );
        });
    }

    #[test]
    fn button_label_follows_the_button_font_not_the_body_font() {
        let mut r = resolved("kde-breeze", ColorMode::Light);
        r.button.font.size = r.defaults.font.size + 3.0;
        let out = button_label(Native::unscaled(&r));
        assert_eq!(out.text.font_size, abs(r.button.font.size));
    }

    #[test]
    fn toggle_is_the_button_labels_text() {
        for_each_case(|_, _, n| {
            let out = toggle(n);
            assert_eq!(out, button_label(n));
            assert_eq!(out.size.height, None);
            assert_eq!(out.padding, Default::default());
            assert_eq!(out.corner_radii, Default::default());
            assert_eq!(out.border_widths, Default::default());
        });
    }

    /// The control-height rule for a table row: `stated` at s <= 1; above 1
    /// at least the scaled text's line box, rounded up, and upstream's own
    /// `Size::Size` cell padding top and bottom.
    fn grown_row(stated: Pixels, r: &ResolvedTheme, font: &ResolvedFontSpec, s: f32) -> Pixels {
        if s <= 1.0 {
            return stated;
        }
        let cell = Size::Size(stated).table_cell_padding();
        let line = (font.size * s * r.defaults.line_height).ceil();
        px(f32::from(stated).max(line + f32::from(cell.top) + f32::from(cell.bottom)))
    }

    #[test]
    fn data_table_size_falls_back_to_upstream_medium() {
        let mut stated = 0;
        let mut unstated = 0;
        let mut grown = 0;
        for_each_case(|r, s, n| {
            let base = match r.list.row_height {
                Some(h) => {
                    stated += 1;
                    px(h)
                }
                None => {
                    unstated += 1;
                    Size::Medium.table_row_height()
                }
            };
            let expected = grown_row(base, r, &r.list.item_font, s);
            if expected != base {
                grown += 1;
            }
            assert!(matches!(data_table_size(n), Size::Size(h) if h == expected));
        });
        assert!(
            stated > 0 && unstated > 0 && grown > 0,
            "both branches, and a row grown with its text, must be exercised"
        );
    }

    #[test]
    fn list_header_height_is_the_stated_height_or_none() {
        let adwaita = resolved("adwaita", ColorMode::Light);
        assert_eq!(adwaita.list.header_height, Some(30.0), "precondition");
        assert_eq!(
            list_header_height(Native::unscaled(&adwaita)),
            Some(px(30.0))
        );
        // KDE states none: the DataTable keeps its header at the row height.
        let breeze = resolved("kde-breeze", ColorMode::Light);
        assert_eq!(breeze.list.header_height, None, "precondition");
        assert_eq!(list_header_height(Native::unscaled(&breeze)), None);
        // The stated height at a text scale of 1, grown with the font the
        // header draws in above it, and nothing where the theme states none.
        let mut grown = false;
        for factor in [1.0, 1.5] {
            let prefs = scaled(factor);
            for (preset, stated) in [
                ("adwaita", Some(30.0)),
                ("material-3", Some(56.0)),
                ("windows-11", None),
                ("catppuccin-mocha", None),
            ] {
                for mode in [ColorMode::Light, ColorMode::Dark] {
                    let r = resolved(preset, mode);
                    let n = Native {
                        resolved: &r,
                        accessibility: &prefs,
                    };
                    let expected = stated.map(|h| grown_row(px(h), &r, &r.list.item_font, factor));
                    grown |= expected != stated.map(px);
                    assert_eq!(list_header_height(n), expected, "{preset} {mode:?}");
                }
            }
        }
        assert!(grown, "no case grows a header with its text");
    }

    /// The header text is drawn in the table container's font, which
    /// [`table`] gives `list.item_font`; `list.header_font` reaches only the
    /// header's colour (`colors.rs`, `list_header_fg`). A header font smaller
    /// than the item font must not leave the stated header too short for it.
    #[test]
    fn a_header_grows_with_the_font_it_draws_in() {
        let mut r = resolved("adwaita", ColorMode::Light);
        let stated = r
            .list
            .header_height
            .expect("adwaita states a header height");
        r.list.item_font.size = stated;
        r.list.header_font.size = 1.0;
        let factor = 2.0;
        let prefs = scaled(factor);
        let n = Native {
            resolved: &r,
            accessibility: &prefs,
        };
        let expected = grown_row(px(stated), &r, &r.list.item_font, factor);
        assert_ne!(
            expected,
            px(stated),
            "precondition: the item font needs growth"
        );
        assert_eq!(list_header_height(n), Some(expected));
    }

    #[test]
    fn dialog_entrance_maps_each_style_to_a_fade() {
        use gpui_component::dialog::DialogEntrance;
        use native_theme::theme::DialogEntranceStyle;
        let mut r = resolved("adwaita", ColorMode::Light);
        for (style, expected) in [
            (
                Some(DialogEntranceStyle::ScaleFade),
                Some(DialogEntrance::Fade),
            ),
            (
                Some(DialogEntranceStyle::FadeRise),
                Some(DialogEntrance::Fade),
            ),
            (None, None),
        ] {
            r.dialog.entrance = style;
            assert_eq!(dialog_entrance(Native::unscaled(&r)), expected, "{style:?}");
        }
        // A preset without the field keeps upstream's entrance.
        let mocha = resolved("catppuccin-mocha", ColorMode::Dark);
        assert_eq!(mocha.dialog.entrance, None, "precondition");
        assert_eq!(dialog_entrance(Native::unscaled(&mocha)), None);
        let breeze = resolved("kde-breeze", ColorMode::Light);
        assert_eq!(
            breeze.dialog.entrance,
            Some(DialogEntranceStyle::FadeRise),
            "precondition"
        );
        assert_eq!(
            dialog_entrance(Native::unscaled(&breeze)),
            Some(DialogEntrance::Fade)
        );
    }

    #[test]
    fn input_refinement_matches_theme_values() {
        let mut stated = Stated::default();
        for_each_case(|r, s, n| {
            let i = &r.input;
            let out = input(n);
            assert_height_rule(&out, r, Some(i.min_height), s, "input");
            stated.padding(&out, &i.border.padding, "input");
            assert_eq!(
                out.corner_radii.top_left,
                abs(i.border.corner_radius.max(0.0))
            );
            assert_eq!(out.border_widths.top, abs(i.border.line_width));
            assert_text(&out, r, &i.font, s);
        });
        stated.assert_each_compared();
    }

    /// `input` and `text_area` letter the field in `input.font.color`: the
    /// Input root sets no text colour (`input/input.rs:707-803`) and gpui-base
    /// paints the field's text in the window's (gpui-base
    /// `input/base/element.rs:2539-2542`), which is `foreground`, fed from
    /// `defaults.text_color`. macos-monterey and macos-tahoe state another, in
    /// both modes.
    #[test]
    fn a_field_draws_its_text_in_the_platforms_input_colour() {
        let mut differs = Vec::new();
        for info in Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let r = resolved(info.key, mode);
                let n = Native::unscaled(&r);
                let at = format!("{}/{mode:?}", info.key);
                let ink = Some(rgba_to_hsla(r.input.font.color));
                assert_eq!(input(n).text.color, ink, "{at}: input text");
                assert_eq!(text_area(n).text.color, ink, "{at}: text area text");
                if r.input.font.color != r.defaults.text_color {
                    differs.push(at);
                }
            }
        }
        assert!(
            !differs.is_empty(),
            "no preset states an input text colour other than the window's \
             any more, so this test proves nothing"
        );
    }

    /// `input_fill` is `input.background_color`, or for a disabled field
    /// `input.disabled_background` (the enabled fill where none is stated)
    /// and `input.disabled_text_color`, faded by `input.disabled_opacity`; it
    /// carries nothing else.
    #[test]
    fn input_fill_is_the_platforms_in_either_state() {
        let mut stated_disabled = 0usize;
        for info in Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let r = resolved(info.key, mode);
                let n = Native::unscaled(&r);
                let at = format!("{}/{mode:?}", info.key);
                let enabled = input_fill(n, false);
                assert_eq!(
                    enabled.background,
                    Some(rgba_to_hsla(r.input.background_color).into()),
                    "{at}: enabled fill"
                );
                assert_eq!(enabled.opacity, None, "{at}: enabled, no fade");
                assert_eq!(enabled.text.color, None, "{at}: enabled, input's text");
                let disabled = input_fill(n, true);
                assert_eq!(
                    disabled.background,
                    Some(
                        rgba_to_hsla(
                            r.input
                                .disabled_background
                                .unwrap_or(r.input.background_color)
                        )
                        .into()
                    ),
                    "{at}: disabled fill"
                );
                assert_eq!(
                    disabled.opacity,
                    Some(r.input.disabled_opacity),
                    "{at}: disabled fade"
                );
                assert_eq!(
                    disabled.text.color,
                    Some(rgba_to_hsla(r.input.disabled_text_color)),
                    "{at}: disabled text"
                );
                stated_disabled += usize::from(r.input.disabled_background.is_some());
                let bare = StyleRefinement {
                    background: None,
                    opacity: None,
                    text: Default::default(),
                    ..disabled
                };
                assert_eq!(
                    bare,
                    StyleRefinement::default(),
                    "{at}: only the fill, the text colour and the fade"
                );
            }
        }
        assert!(stated_disabled > 0, "no preset states a disabled fill");
    }

    /// `button_disabled` is `button.disabled_background` (or
    /// `button.background_color` where none is stated) and
    /// `button.disabled_text_color`, and carries nothing else.
    #[test]
    fn button_disabled_is_the_platforms_disabled_pair() {
        let mut stated_fill = 0usize;
        for info in Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let r = resolved(info.key, mode);
                let at = format!("{}/{mode:?}", info.key);
                let b = &r.button;
                let out = button_disabled(Native::unscaled(&r));
                let fill = b.disabled_background.unwrap_or(b.background_color);
                assert_eq!(
                    out.background,
                    Some(rgba_to_hsla(fill).into()),
                    "{at}: disabled fill"
                );
                assert_eq!(
                    out.text.color,
                    Some(rgba_to_hsla(b.disabled_text_color)),
                    "{at}: disabled label"
                );
                assert_eq!(out.opacity, Some(b.disabled_opacity), "{at}: disabled fade");
                stated_fill += usize::from(b.disabled_background.is_some());
                let bare = StyleRefinement {
                    background: None,
                    text: Default::default(),
                    opacity: None,
                    ..out
                };
                assert_eq!(
                    bare,
                    StyleRefinement::default(),
                    "{at}: only the pair and the fade"
                );
            }
        }
        assert!(stated_fill > 0, "no preset states a disabled button fill");
    }

    /// `button_checked` is `button.checked_background` and
    /// `.checked_text_color`, the pressed pair where the theme states none
    /// (docs/platform-facts.md §2.3), and nothing else.
    #[test]
    fn button_checked_is_the_platforms_checked_pair() {
        let mut stated = 0usize;
        for info in Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let r = resolved(info.key, mode);
                let at = format!("{}/{mode:?}", info.key);
                let b = &r.button;
                let out = button_checked(Native::unscaled(&r));
                assert_eq!(
                    out.background,
                    b.checked_background
                        .or(b.active_background)
                        .map(|fill| rgba_to_hsla(fill).into()),
                    "{at}: checked fill"
                );
                assert_eq!(
                    out.text.color,
                    Some(rgba_to_hsla(
                        b.checked_text_color.unwrap_or(b.active_text_color)
                    )),
                    "{at}: checked label"
                );
                stated += usize::from(b.checked_background.is_some());
                let bare = StyleRefinement {
                    background: None,
                    text: Default::default(),
                    ..out
                };
                assert_eq!(bare, StyleRefinement::default(), "{at}: only the pair");
            }
        }
        assert!(stated > 0, "no preset states a checked button fill");
    }

    /// `link` carries the link's text size and weight, and removes upstream's
    /// resting underline exactly where `link.underline_enabled` is false
    /// (material-3 states it false, the others true).
    #[test]
    fn a_link_is_underlined_exactly_where_the_platform_underlines_it() {
        let mut seen = [false, false];
        for info in Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let r = resolved(info.key, mode);
                let n = Native::unscaled(&r);
                let at = format!("{}/{mode:?}", info.key);
                let out = link(n);
                assert_text(&out, &r, &r.link.font, 1.0);
                let thickness = out.text.underline.as_ref().map(|u| u.thickness);
                if r.link.underline_enabled {
                    assert_eq!(thickness, None, "{at}: upstream's underline stands");
                } else {
                    assert_eq!(thickness, Some(px(0.)), "{at}: no underline");
                }
                seen[usize::from(r.link.underline_enabled)] = true;
            }
        }
        assert_eq!(seen, [true, true], "both kinds of platform are compared");
    }

    /// `input_height` is the height rule [`input`] applies, and nothing else
    /// of its refinement.
    #[test]
    fn input_height_is_the_height_rule_alone() {
        for_each_case(|r, s, n| {
            let out = input_height(n);
            assert_height_rule(&out, r, Some(r.input.min_height), s, "input_height");
            let full = input(n);
            assert_eq!(out.size.height, full.size.height);
            assert_eq!(out.min_size.height, full.min_size.height);
            assert_eq!(out.text.line_height, full.text.line_height);
            assert_eq!(out.padding, StyleRefinement::default().padding);
            assert_eq!(out.text.font_size, None);
            assert_eq!(out.border_widths, StyleRefinement::default().border_widths);
            assert_eq!(out.corner_radii, StyleRefinement::default().corner_radii);
        });
    }

    #[test]
    fn menu_and_list_items_match_theme_values() {
        let mut stated = Stated::default();
        for_each_case(|r, s, n| {
            let m = &r.menu;
            let out = menu_item(n);
            stated.height(&out, r, m.row_height, s, "menu_item");
            stated.padding(&out, &m.border.padding, "menu_item");
            assert_eq!(out.gap.width, def(m.icon_text_gap));
            assert_eq!(out.gap.height, None, "gap_x sets the column gap only");
            assert_text(&out, r, &m.font, s);

            let l = &r.list;
            let out = list_item(n);
            stated.height(&out, r, l.row_height, s, "list_item");
            stated.padding(&out, &l.border.padding, "list_item");
            assert_text(&out, r, &l.item_font, s);
        });
        stated.assert_each_compared();
    }

    #[test]
    fn tooltip_popover_status_bar_match_theme_values() {
        let mut stated = Stated::default();
        for_each_case(|r, s, n| {
            let t = &r.tooltip;
            let out = tooltip(n);
            // The width is the content element's, not the bubble's: a bubble
            // that states it clamps itself and not its text.
            assert_eq!(out.max_size.width, None);
            let upstream = TOOLTIP_UPSTREAM_PX_REM * r.defaults.font.size * s;
            assert_eq!(
                tooltip_content(n).max_size.width,
                len((t.max_width
                    - t.border.padding.left.unwrap_or(upstream)
                    - t.border.padding.right.unwrap_or(upstream)
                    - 2.0 * t.border.line_width)
                    .max(0.0))
            );
            stated.padding(&out, &t.border.padding, "tooltip");
            assert_eq!(
                out.corner_radii.top_left,
                abs(t.border.corner_radius.max(0.0))
            );
            assert_text(&out, r, &t.font, s);
            assert_eq!(
                out.background,
                Some(rgba_to_hsla(t.background_color).into()),
                "the tooltip's own fill, not the popover's"
            );
            assert_eq!(out.border_color, Some(rgba_to_hsla(t.border.color)));
            assert_eq!(out.border_widths.top, abs(t.border.line_width));
            let shadow = out.box_shadow.clone().unwrap_or_default();
            assert_eq!(
                shadow.len(),
                if t.border.shadow_enabled {
                    TOOLTIP_SHADOW.len()
                } else {
                    0
                },
                "tooltip.border.shadow_enabled"
            );
            assert!(
                shadow
                    .iter()
                    .all(|l| l.color == rgba_to_hsla(r.defaults.shadow_color)),
                "the tooltip's shadow is not defaults.shadow_color"
            );

            let p = &r.popover;
            let out = popover(n);
            stated.padding(&out, &p.border.padding, "popover");
            assert_eq!(
                out.corner_radii.top_left,
                abs(p.border.corner_radius.max(0.0))
            );

            let sb = &r.status_bar;
            let out = status_bar(n);
            stated.padding(&out, &sb.border.padding, "status_bar");
            assert_text(&out, r, &sb.font, s);
        });
        stated.assert_each_compared();
    }

    /// The gutter is the width the base layer installed, and only where the
    /// platform's scrollbars are not overlays. Over every preset in both modes,
    /// because the `CASES` names are all on one side of that line; both sides
    /// have to occur, or the builder's condition is never exercised.
    #[test]
    fn the_scrollbar_gutter_is_the_installed_groove_where_bars_are_not_overlays() {
        let (mut overlaid, mut embedded) = (0usize, 0usize);
        for info in Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let r = resolved(info.key, mode);
                let prefs = scaled(1.0);
                let n = Native {
                    resolved: &r,
                    accessibility: &prefs,
                };
                let at = format!("{}/{mode:?}", info.key);
                let out = scrollbar_gutter(n);
                if r.scrollbar.overlay_mode {
                    overlaid += 1;
                    assert_eq!(out.padding.right, None, "{at}: an overlay bar took width");
                } else {
                    embedded += 1;
                    assert_eq!(
                        out.padding.right,
                        Some(crate::base_layer::scrollbar_geometry(&r).track_width.into()),
                        "{at}: the gutter is not the track width the base layer installed"
                    );
                }
                // Nothing else: the gutter is one edge, not a layout.
                assert_eq!(out.padding.left, None, "{at}");
                assert_eq!(out.padding.top, None, "{at}");
                assert_eq!(out.padding.bottom, None, "{at}");
            }
        }
        assert!(
            overlaid > 0 && embedded > 0,
            "every preset is on the same side of overlay_mode ({overlaid} overlaid, \
             {embedded} embedded), so one arm of the builder is never reached"
        );
    }

    /// A builder carries the platform's text colour only where the colour
    /// reaches the text *and* upstream's state colours still win afterwards:
    /// upstream either sets a token of its own on the very element the
    /// refinement lands on and re-applies its state colours after it, or sets
    /// no colour on that element at all. Seven are checked here, over every
    /// preset in both modes, not just the `CASES` names; [`input`] and
    /// [`text_area`] in `a_field_draws_its_text_in_the_platforms_input_colour`.
    ///
    /// The `assert_ne!`s below are the reason the first three exist: each of
    /// those native colours differs from the token upstream would otherwise
    /// paint, so leaving the colour out left the widget labelled with a colour
    /// the platform did not state.
    ///
    /// The other four -- `list_item`, `radio`, `select`, `title_bar` -- get no
    /// such `assert_ne!`, and deliberately: their native font colour equals
    /// `defaults.text_color` in 32 of the 34 combinations today (ios-13's
    /// select is the exception, its `combo_box.font.color` the accent), because
    /// a preset that states no `font.color` for a widget inherits the window's
    /// (docs/inheritance-rules.toml:115-136), and `foreground` is fed from
    /// `defaults.text_color`. The colour is carried anyway because nothing
    /// stands in its way -- upstream labels the first three with `foreground`
    /// one refinement earlier and mutes the disabled state on a child
    /// afterwards, and it labels the title bar with nothing at all
    /// (`title_bar.rs:331-346`) -- so the day a source states
    /// `list.item_font.color`, or `window.title_bar_font.color` as the KDE
    /// reader does from `[WM] activeForeground`, the element is painted with it
    /// instead of silently keeping the window's.
    ///
    /// `checkbox` and `combobox` do not carry it although upstream paints them
    /// from `foreground` too: the route is blocked at the other end, which
    /// `checkbox_and_combobox_carry_no_colour_because_upstream_leaves_no_route`
    /// spells out.
    #[test]
    fn text_colour_is_the_platforms_wherever_upstream_would_override_it() {
        let mut tooltip_differs = 0usize;
        let mut dialog_body_differs = 0usize;
        for info in Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let r = resolved(info.key, mode);
                let prefs = scaled(1.0);
                let n = Native {
                    resolved: &r,
                    accessibility: &prefs,
                };
                let at = format!(
                    "{}/{}",
                    info.key,
                    if mode == ColorMode::Dark {
                        "dark"
                    } else {
                        "light"
                    }
                );

                assert_eq!(
                    status_bar(n).text.color,
                    Some(rgba_to_hsla(r.status_bar.font.color)),
                    "{at}: status bar text"
                );
                assert_eq!(
                    tooltip(n).text.color,
                    Some(rgba_to_hsla(r.tooltip.font.color)),
                    "{at}: tooltip text"
                );
                assert_eq!(
                    dialog_description(n).text.color,
                    Some(rgba_to_hsla(r.dialog.body_font.color)),
                    "{at}: dialog description text"
                );

                // The same mechanism, upstream's `foreground` one refinement
                // earlier and its disabled colour on a child afterwards:
                // `list/list_item.rs:205` -> `:209`, `radio.rs:220` -> `:231`
                // (child muted at `:261-262`) and `select.rs:546` -> `:553`
                // (through `input_style`, `input/input.rs:105`; the title child
                // re-mutes at `:484-486`).
                assert_eq!(
                    list_item(n).text.color,
                    Some(rgba_to_hsla(r.list.item_font.color)),
                    "{at}: list item text"
                );
                assert_eq!(
                    radio(n).text.color,
                    Some(rgba_to_hsla(r.checkbox.font.color)),
                    "{at}: radio label text (platform-facts §2.5: the checkbox's)"
                );
                assert_eq!(
                    select(n).text.color,
                    Some(rgba_to_hsla(r.combo_box.font.color)),
                    "{at}: select text"
                );
                // Upstream sets no colour on the bar at all
                // (`title_bar.rs:331-346`), so the route is open and nothing
                // is displaced -- and the KDE reader does state the field,
                // from `[WM] activeForeground` (`native-theme/src/kde/colors.rs`).
                assert_eq!(
                    title_bar(n).text.color,
                    Some(rgba_to_hsla(r.window.title_bar_font.color)),
                    "{at}: title bar text"
                );

                // The two the route is blocked for; the guard below says why.
                assert_eq!(
                    checkbox(n).text.color,
                    None,
                    "{at}: checkbox label text must stay upstream's"
                );
                assert_eq!(
                    combobox(n).text.color,
                    None,
                    "{at}: combobox text must stay upstream's"
                );

                // What upstream paints without the refinement: muted_foreground
                // on the status bar (`status_bar.rs:95`) and on a dialog's
                // description (`dialog/description.rs:50`), popover_foreground
                // on a tooltip (`tooltip.rs:116`).
                assert_ne!(
                    r.status_bar.font.color, r.defaults.muted_color,
                    "{at}: status bar colour no longer differs from upstream's \
                     muted_foreground, so this builder's colour proves nothing"
                );
                if r.dialog.body_font.color != r.defaults.muted_color {
                    dialog_body_differs += 1;
                }
                if r.tooltip.font.color != r.popover.font.color {
                    tooltip_differs += 1;
                }
            }
        }
        assert!(
            tooltip_differs > 0,
            "no preset states a tooltip colour of its own any more, so the \
             tooltip builder's colour proves nothing"
        );
        // Material's dialog body is `on-surface-variant`, its muted colour
        // too (docs/platform-facts.md §2.22, §2.1.3); the others differ.
        assert!(
            dialog_body_differs > 0,
            "no preset's dialog body colour differs from upstream's \
             muted_foreground any more, so the dialog builder's colour proves \
             nothing"
        );
    }

    /// The combinations whose preset states a combo box text colour of its own,
    /// which gpui-component 0.7.3 gives no route for (Tier U): an iOS select's
    /// text is systemBlue (WebKit html.css@31adf9d1:1099).
    const STATED_COMBO_TEXT: [&str; 2] = ["ios-13/light", "ios-13/dark"];

    /// `geometry::checkbox` and `geometry::combobox` carry no colour although
    /// upstream paints both from `foreground`: the route is blocked past the
    /// element the refinement lands on. A `Checkbox::label` sits in a wrapper
    /// that sets `foreground` itself (`checkbox.rs:339-344`), so a carried
    /// colour never reaches it; and both widgets apply their disabled colour
    /// *before* the refinement (`checkbox.rs:260-264`, and
    /// `input_style(disabled, ..)` at `input/input.rs:99-103` feeding
    /// `combobox.rs:991` -> `:998`, whose selected-title branch `:585-591` sets
    /// no colour to re-mute with), so a carried colour would displace it.
    ///
    /// The run therefore also guards the preconditions: every preset states
    /// for both widgets exactly the colour upstream already paints there
    /// (`defaults.text_color` -> `foreground`), so the two builders lose
    /// nothing today, except the combinations in [`STATED_COMBO_TEXT`].
    #[test]
    fn checkbox_and_combobox_carry_no_colour_because_upstream_leaves_no_route() {
        for info in Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let r = resolved(info.key, mode);
                let at = format!(
                    "{}/{}",
                    info.key,
                    if mode == ColorMode::Dark {
                        "dark"
                    } else {
                        "light"
                    }
                );
                let prefs = scaled(1.0);
                let n = Native {
                    resolved: &r,
                    accessibility: &prefs,
                };
                assert_eq!(
                    checkbox(n).text.color,
                    None,
                    "{at}: geometry::checkbox set a text colour; upstream's \
                     label wrapper paints `foreground` itself \
                     (checkbox.rs:339-344) so it cannot arrive, and the \
                     disabled hook (checkbox.rs:260-264) would take it instead \
                     of `muted_foreground`"
                );
                assert_eq!(
                    combobox(n).text.color,
                    None,
                    "{at}: geometry::combobox set a text colour; upstream \
                     applies the disabled `muted_foreground` before this \
                     refinement (input/input.rs:99-103 -> combobox.rs:991 -> \
                     :998), so it would displace the disabled colour"
                );
                assert_eq!(
                    r.checkbox.font.color, r.defaults.text_color,
                    "{at}: this preset states a checkbox label text colour \
                     that gpui-component 0.7.3 gives no route for \
                     (checkbox.rs:339-344) -- record it as a Tier U candidate; \
                     do NOT carry it in the builder, it would displace the \
                     disabled colour (checkbox.rs:260-264)"
                );
                if STATED_COMBO_TEXT.contains(&at.as_str()) {
                    assert_ne!(
                        r.combo_box.font.color, r.defaults.text_color,
                        "{at}: declared in STATED_COMBO_TEXT but no longer \
                         states a combo box text colour of its own -- remove \
                         it from the list"
                    );
                } else {
                    assert_eq!(
                        r.combo_box.font.color, r.defaults.text_color,
                        "{at}: this preset states a combo box text colour that \
                         gpui-component 0.7.3 gives no route for \
                         (combobox.rs:991 -> :998) -- record it as a Tier U \
                         candidate; do NOT carry it in the builder, it would \
                         displace the disabled colour (input/input.rs:99-103)"
                    );
                }
            }
        }
    }

    #[test]
    fn dialog_family_matches_theme_values() {
        let mut stated = Stated::default();
        for_each_case(|r, s, n| {
            let d = &r.dialog;
            let out = dialog(n);
            stated.padding(&out, &d.border.padding, "dialog");
            assert_eq!(out.min_size.height, len(d.min_height));
            assert_eq!(out.max_size.height, len(d.max_height));
            assert_eq!(
                out.corner_radii.top_left,
                abs(d.border.corner_radius.max(0.0))
            );
            assert_eq!(dialog_footer(n).gap.width, def(d.button_gap));
            assert_eq!(dialog_footer(n).gap.height, def(d.button_gap));
            assert_text(&dialog_title(n), r, &d.title_font, s);
            assert_text(&dialog_description(n), r, &d.body_font, s);
            assert_eq!(dialog_max_width(n), px(d.max_width));
            assert_text(&table(n), r, &r.list.item_font, s);
            assert_text(&title_bar(n), r, &r.window.title_bar_font, s);
        });
        stated.assert_each_compared();
    }

    /// The dialog's radius is its own field, not `radius_lg`: adwaita is where
    /// the two differ (18 against 15), so it is the discriminating input.
    #[test]
    fn dialog_radius_is_the_dialogs_own() {
        let r = resolved("adwaita", ColorMode::Light);
        let prefs = scaled(1.0);
        let n = Native {
            resolved: &r,
            accessibility: &prefs,
        };
        assert_ne!(
            r.dialog.border.corner_radius, r.defaults.border.corner_radius_lg,
            "adwaita no longer discriminates; pick another preset"
        );
        assert_eq!(
            dialog(n).corner_radii.top_left,
            abs(r.dialog.border.corner_radius)
        );
    }

    /// A button nested in an input group keeps the platform's button radius;
    /// it sets nothing else, because the group sizes it to fit the field.
    #[test]
    fn input_group_button_carries_only_the_button_radius() {
        for_each_case(|r, _s, n| {
            let out = input_group_button(n);
            assert_eq!(
                out.corner_radii.top_left,
                abs(r.button.border.corner_radius.max(0.0))
            );
            assert_eq!(out.size.height, None);
            assert_eq!(out.min_size.width, None);
            assert_eq!(out.padding.left, None);
        });
    }

    #[test]
    fn progress_group_box_accordion_match_theme_values() {
        let mut stated = Stated::default();
        for_each_case(|r, _s, n| {
            let p = &r.progress_bar;
            let out = progress(n);
            assert_eq!(out.size.height, len(p.track_height));
            assert_eq!(
                out.corner_radii.top_left,
                abs(p.border.corner_radius.max(0.0))
            );
            assert_eq!(out.min_size.width, len(p.min_width));

            let c = &r.card;
            let out = group_box_content(n);
            stated.padding(&out, &c.border.padding, "group_box_content");
            assert_eq!(
                out.corner_radii.top_left,
                abs(c.border.corner_radius.max(0.0))
            );
            assert_eq!(out.border_widths.top, abs(c.border.line_width));
            assert_eq!(out.border_color, Some(rgba_to_hsla(c.border.color)));

            assert_eq!(
                accordion_title(n).size.height,
                len(r.expander.header_height)
            );
        });
        stated.assert_each_compared();
    }

    #[test]
    fn checkbox_radio_select_match_theme_values() {
        let mut stated = Stated::default();
        for_each_case(|r, s, n| {
            let c = &r.checkbox;
            let out = checkbox(n);
            assert_eq!(out.gap.width, def(c.label_gap));
            assert_text(&out, r, &c.font, s);
            // platform-facts §2.5: radio metrics are the checkbox's.
            assert_eq!(radio(n).gap.width, def(c.label_gap));
            assert_text(&radio(n), r, &c.font, s);

            let cb = &r.combo_box;
            // Beside a stated arrow column the right side is upstream's (D2,
            // `select_and_combobox_leave_the_right_side_beside_an_arrow_column`).
            let p = match cb.arrow_area_width {
                Some(_) => ResolvedPadding {
                    right: None,
                    ..cb.border.padding
                },
                None => cb.border.padding,
            };
            let out = select(n);
            assert_height_rule(&out, r, Some(cb.min_height), s, "select");
            stated.padding(&out, &p, "select");
            assert_eq!(out.min_size.width, len(cb.min_width));
            assert_eq!(
                out.corner_radii.top_left,
                abs(cb.border.corner_radius.max(0.0))
            );
            assert_text(&out, r, &cb.font, s);
            assert_eq!(combobox(n).min_size.width, len(cb.min_width));
            assert_height_rule(&combobox(n), r, Some(cb.min_height), s, "combobox");
            stated.padding(&combobox(n), &p, "combobox");
        });
        stated.assert_each_compared();
    }

    /// D2: where `combo_box.arrow_area_width` is stated, the right side is
    /// measured to an arrow column gpui's triggers do not have, so neither
    /// builder sets it and upstream's own right padding stands; the other
    /// sides are the stated ones. Without the column, the right side is
    /// stated to the text and set.
    #[test]
    fn select_and_combobox_leave_the_right_side_beside_an_arrow_column() {
        for preset in ["kde-breeze", "windows-11"] {
            let mut r = resolved(preset, ColorMode::Light);
            assert!(
                r.combo_box.arrow_area_width.is_some(),
                "{preset} no longer states an arrow column; pick another preset"
            );
            let p = r.combo_box.border.padding;
            assert!(
                p.right.is_some(),
                "{preset} no longer states a right side; pick another preset"
            );
            for (what, out) in [
                ("select", select(Native::unscaled(&r))),
                ("combobox", combobox(Native::unscaled(&r))),
            ] {
                assert_eq!(out.padding.right, None, "{preset} {what}: right");
                assert_eq!(out.padding.top, side(p.top), "{preset} {what}: top");
                assert_eq!(
                    out.padding.bottom,
                    side(p.bottom),
                    "{preset} {what}: bottom"
                );
                assert_eq!(out.padding.left, side(p.left), "{preset} {what}: left");
            }
            r.combo_box.arrow_area_width = None;
            for (what, out) in [
                ("select", select(Native::unscaled(&r))),
                ("combobox", combobox(Native::unscaled(&r))),
            ] {
                let what = format!("{preset} {what} without the column");
                assert!(
                    assert_padding(&out, &p, &what) > 0,
                    "{what}: no side stated"
                );
            }
        }
    }

    #[test]
    fn size_helpers_use_theme_values() {
        for_each_case(|r, _s, n| {
            assert_eq!(spinner_size(n), Size::Size(px(r.spinner.diameter)));
            let is = &r.defaults.icon_sizes;
            assert_eq!(icon_size_toolbar(n), Size::Size(px(r.toolbar.icon_size)));
            assert_eq!(icon_size_small(n), Size::Size(px(is.small)));
            assert_eq!(icon_size_large(n), Size::Size(px(is.large)));
            assert_eq!(icon_size_dialog(n), Size::Size(px(is.dialog)));
            assert_eq!(icon_size_panel(n), Size::Size(px(is.panel)));
        });
    }

    /// Over every preset in both modes: the row is the model's `toolbar`, with
    /// the bar height as a floor rather than a fixed height, and only where it
    /// is stated (KDE's toolbar sizes to its content, platform-facts §2.13),
    /// and no edge.
    #[test]
    fn toolbar_carries_the_models_toolbar() {
        let mut stated = 0usize;
        for info in Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let r = resolved(info.key, mode);
                let prefs = scaled(1.0);
                let n = Native {
                    resolved: &r,
                    accessibility: &prefs,
                };
                let at = format!("{}/{mode:?}", info.key);
                let t = &r.toolbar;
                let out = toolbar(n);
                assert_eq!(
                    out.min_size.height,
                    t.bar_height.map(|h| px(h).into()),
                    "{at}: bar height"
                );
                assert_eq!(
                    out.size.height,
                    Some(gpui::Length::Auto),
                    "{at}: the toolbar's height is its content's, the bar height a floor"
                );
                assert_eq!(out.gap.width, t.item_gap.and_then(def), "{at}: item gap");
                stated += assert_padding(&out, &t.border.padding, &at);
                assert_eq!(
                    out.background,
                    Some(rgba_to_hsla(t.background_color).into()),
                    "{at}: background"
                );
                assert_text(&out, &r, &t.font, 1.0);
                assert_eq!(out.text.color, None, "{at}: no text colour");
                assert_eq!(
                    out.border_widths,
                    StyleRefinement::default().border_widths,
                    "{at}: §2.13 states no edge"
                );
                assert_eq!(out.border_color, None, "{at}: §2.13 states no edge");
            }
        }
        assert!(
            stated > 0,
            "no preset states a toolbar padding side, so the builder's padding \
             was compared with nothing"
        );
    }

    /// `toolbar.icon_size` inherits `defaults.icon_sizes.toolbar`, so over the
    /// bundled presets the two agree; the second half states a toolbar-specific one
    /// and checks the helper follows the toolbar, not the default.
    #[test]
    fn icon_size_toolbar_reads_the_toolbar() {
        for info in Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let mut r = resolved(info.key, mode);
                let at = format!("{}/{mode:?}", info.key);
                assert_eq!(
                    icon_size_toolbar(Native::unscaled(&r)),
                    Size::Size(px(r.toolbar.icon_size)),
                    "{at}"
                );
                r.toolbar.icon_size = r.defaults.icon_sizes.toolbar + 8.0;
                assert_eq!(
                    icon_size_toolbar(Native::unscaled(&r)),
                    Size::Size(px(r.toolbar.icon_size)),
                    "{at}: a toolbar icon size of its own"
                );
            }
        }
    }

    #[test]
    fn layout_accessors_pass_through_option() {
        let layout = Theme::preset("kde-breeze").expect("preset").layout;
        assert_eq!(widget_gap(&layout), layout.widget_gap.map(px));
        assert_eq!(container_margin(&layout), layout.container_margin.map(px));
        assert_eq!(window_margin(&layout), layout.window_margin.map(px));
        assert_eq!(section_gap(&layout), layout.section_gap.map(px));
        assert!(
            widget_gap(&layout).is_some(),
            "static presets define all four keys (§1.3)"
        );
        assert_eq!(widget_gap(&LayoutTheme::default()), None);
    }

    /// A theme whose padding sides are stated on some sides and not on
    /// others: stated sides of zero and of a number, and unstated ones.
    fn partly_stated() -> ResolvedTheme {
        let mut r = resolved("catppuccin-mocha", ColorMode::Dark);
        let p = ResolvedPadding {
            top: Some(0.0),
            right: None,
            bottom: None,
            left: Some(7.0),
        };
        for b in [
            &mut r.button.border,
            &mut r.input.border,
            &mut r.text_area.border,
            &mut r.menu.border,
            &mut r.list.border,
            &mut r.tooltip.border,
            &mut r.popover.border,
            &mut r.status_bar.border,
            &mut r.dialog.border,
            &mut r.card.border,
            &mut r.combo_box.border,
            &mut r.toolbar.border,
        ] {
            b.padding = p;
        }
        r
    }

    /// Every builder that pads sets the sides the theme states and leaves the
    /// others unset, so the receiving widget keeps its own padding there.
    #[test]
    fn each_padding_builder_leaves_unstated_sides_unset() {
        let r = partly_stated();
        let n = Native::unscaled(&r);
        type Builder = fn(Native<'_>) -> StyleRefinement;
        let builders: [(&str, Builder); 11] = [
            ("button", button),
            ("input", input),
            ("menu_item", menu_item),
            ("list_item", list_item),
            ("tooltip", tooltip),
            ("popover", popover),
            ("status_bar", status_bar),
            ("dialog", dialog),
            ("group_box_content", group_box_content),
            ("select", select),
            ("toolbar", toolbar),
        ];
        for (what, build) in builders {
            let out = build(n);
            assert_eq!(out.padding.top, def(0.0), "{what}: a stated zero is set");
            assert_eq!(out.padding.left, def(7.0), "{what}: a stated side is set");
            assert_eq!(out.padding.right, None, "{what}: an unstated side is set");
            assert_eq!(out.padding.bottom, None, "{what}: an unstated side is set");
        }
        let out = combobox(n);
        assert_eq!(out.padding.right, None, "combobox: an unstated side is set");
        assert_eq!(out.padding.left, def(7.0), "combobox");
    }

    /// The multi-line field's root pads by the rest of each stated side past
    /// the editor padding upstream gives a Textarea (`Size::Medium`: 10
    /// across, 8 down), nothing below it, and leaves an unstated side unset.
    #[test]
    fn the_text_area_pads_the_root_by_the_rest_of_each_side() {
        let mut r = partly_stated();
        r.text_area.border.padding = ResolvedPadding {
            top: Some(5.0),
            right: None,
            bottom: Some(9.0),
            left: Some(12.0),
        };
        let out = text_area(Native::unscaled(&r));
        assert_eq!(out.padding.top, def(0.0), "5 down, below the editor's 8");
        assert_eq!(out.padding.bottom, def(1.0), "9 down: 9 − 8");
        assert_eq!(out.padding.left, def(2.0), "12 across: 12 − 10");
        assert_eq!(out.padding.right, None, "an unstated side is upstream's");
    }

    /// The multi-line field's edge is `text_area.border.color`, over the
    /// `input` token upstream draws it in (`input/input.rs:798` → `:803`).
    #[test]
    fn the_text_area_draws_its_own_edge_colour() {
        let mut r = resolved("kde-breeze", ColorMode::Light);
        r.text_area.border.color = r.defaults.accent_color;
        assert_ne!(
            r.text_area.border.color, r.input.border.color,
            "precondition"
        );
        assert_eq!(
            text_area(Native::unscaled(&r)).border_color,
            Some(rgba_to_hsla(r.text_area.border.color))
        );
    }

    /// Without a stated bar height the toolbar is its content's height, with
    /// no floor.
    #[test]
    fn toolbar_leaves_min_height_unset_without_a_bar_height() {
        let mut r = resolved("catppuccin-mocha", ColorMode::Dark);
        r.toolbar.bar_height = None;
        r.toolbar.item_gap = None;
        let out = toolbar(Native::unscaled(&r));
        assert_eq!(out.min_size.height, None);
        assert_eq!(out.size.height, Some(gpui::Length::Auto));
        assert_eq!(out.gap.width, None, "an unstated gap is the row's own");
        r.toolbar.bar_height = Some(40.0);
        assert_eq!(toolbar(Native::unscaled(&r)).min_size.height, len(40.0));
    }

    #[test]
    fn menu_and_list_items_leave_height_unset_without_a_row_height() {
        let mut r = resolved("catppuccin-mocha", ColorMode::Dark);
        r.menu.row_height = None;
        r.list.row_height = None;
        for out in [
            menu_item(Native::unscaled(&r)),
            list_item(Native::unscaled(&r)),
        ] {
            assert_eq!(out.size.height, None);
            assert_eq!(out.min_size.height, None);
            assert_eq!(out.text.line_height, Some(relative(r.defaults.line_height)));
        }
        r.menu.row_height = Some(22.0);
        r.list.row_height = Some(24.0);
        assert_eq!(menu_item(Native::unscaled(&r)).size.height, len(22.0));
        assert_eq!(list_item(Native::unscaled(&r)).size.height, len(24.0));
    }

    /// An unstated tooltip side is upstream's `px_2`, 0.5 rem at the rem this
    /// connector installs (`defaults.font.size` × the text-scaling factor);
    /// a stated side is the platform's.
    #[test]
    fn tooltip_content_uses_upstreams_rem_padding_for_an_unstated_side() {
        let mut r = resolved("catppuccin-mocha", ColorMode::Dark);
        r.tooltip.border.padding = ResolvedPadding {
            top: None,
            right: None,
            bottom: None,
            left: Some(3.0),
        };
        for s in [1.0, 2.0] {
            let prefs = scaled(s);
            let n = Native {
                resolved: &r,
                accessibility: &prefs,
            };
            let rem = r.defaults.font.size * s;
            assert_eq!(
                tooltip_content(n).max_size.width,
                len(r.tooltip.max_width - 3.0 - 0.5 * rem - 2.0 * r.tooltip.border.line_width),
                "at s = {s}"
            );
        }
    }

    /// A factor that overflows the rem leaves the unstated sides at upstream's
    /// 0.5 rem of the unscaled font size, so the text keeps its width.
    #[test]
    fn tooltip_content_takes_the_unscaled_rem_where_the_factor_overflows_it() {
        let mut r = resolved("catppuccin-mocha", ColorMode::Dark);
        r.tooltip.border.padding = ResolvedPadding {
            top: None,
            right: None,
            bottom: None,
            left: None,
        };
        assert!((r.defaults.font.size * f32::MAX).is_infinite());
        let prefs = scaled(f32::MAX);
        let n = Native {
            resolved: &r,
            accessibility: &prefs,
        };
        let rem = r.defaults.font.size;
        let expected = r.tooltip.max_width - 2.0 * 0.5 * rem - 2.0 * r.tooltip.border.line_width;
        assert!(expected > 0.0);
        assert_eq!(tooltip_content(n).max_size.width, len(expected));
    }

    /// At s <= 1 the stated height is exact; above 1 it is a floor and the
    /// height is laid out. The boundary is exactly 1.
    #[test]
    fn the_height_rule_switches_above_a_text_scale_of_one() {
        let r = resolved("kde-breeze", ColorMode::Light);
        for (s, laid_out) in [(0.5, false), (1.0, false), (1.01, true), (2.0, true)] {
            let prefs = scaled(s);
            let n = Native {
                resolved: &r,
                accessibility: &prefs,
            };
            let out = button(n);
            assert_eq!(
                out.size.height == Some(Length::Auto),
                laid_out,
                "at s = {s}: {:?}",
                out.size.height
            );
        }
    }

    #[test]
    fn unscaled_view_scales_nothing() {
        let r = resolved("catppuccin-mocha", ColorMode::Dark);
        let n = Native::unscaled(&r);
        assert_eq!(input(n).text.font_size, abs(r.input.font.size));
    }
}
