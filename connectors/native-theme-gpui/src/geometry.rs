//! Per-widget geometry for gpui-component 0.6.4 widgets (spec §9).
//!
//! Every builder is a pure function of a [`Native`] view and returns a
//! [`StyleRefinement`] the application applies with
//! `gpui_component::StyledExt::refine_style`:
//!
//! ```ignore
//! use gpui_component::StyledExt;
//! use native_theme_gpui::{ActiveNativeTheme, geometry};
//!
//! let n = cx.native_theme().and_then(|t| t.native(cx));
//! let button = Button::new("save").label("Save");
//! let button = match n { Some(n) => button.refine_style(&geometry::button(n)), None => button };
//! ```
//!
//! Each widget applies the caller's refinement after its own geometry at the
//! cited upstream line, so the values here win. Text sizes carry the
//! accessibility text-scaling factor; widths, paddings, radii and icon sizes
//! do not (spec §3.4). Every value is a `ResolvedTheme` field or the one
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
//! lays out with the platform's metrics. [`select`] and [`combobox`] take the
//! stated height as a minimum (`min_h`) at every text-scaling factor and
//! leave upstream's own height in place: `h_8` for `Size::Medium`
//! (`sizing.rs:236-237`, `:261-264`), 2 rem, where the rem is the installed
//! `Theme::font_size` (`root.rs:582`), which [`to_theme`](crate::to_theme)
//! scales by the text-scaling factor (`lib.rs:170`), so upstream's height
//! grows with the text. The other four take their stated height (`h`) at a
//! factor of 1 or less; above 1 they take it as a minimum and an automatic
//! height, so layout grows the control around its drawn text and padding.
//! Where the theme states no height (`menu.row_height` and `list.row_height`
//! are optional, and KDE states neither), the builder sets the line height
//! alone and the row keeps the toolkit's own height. The rule is for
//! single-line controls.
//!
//! Geometry, with one exception that is not geometry: seven builders also
//! carry the platform's text colour. A builder carries it only where the
//! carried colour actually reaches the text and upstream's state colours
//! (disabled, muted) still win — either because upstream labels the very
//! element the refinement lands on with a token of its own one refinement
//! earlier and applies its state colours after the refinement or on a child,
//! or because upstream sets no colour on that element at all, so there is
//! nothing to displace. Three of the seven displace a visibly different colour
//! today — [`status_bar`] and [`dialog_description`] displace
//! `muted_foreground`, [`tooltip`] displaces `popover_foreground` — three
//! displace `foreground`: [`list_item`], [`radio`] and [`select`], and
//! [`title_bar`] displaces nothing (`title_bar.rs:328-343` paints the fill and
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
//! either never arrive or displace the disabled one (`checkbox.rs:334-339` and
//! `:252-256`, `input/input.rs:99-103` through `combobox.rs:997`).
//!
//! [`button`], [`input`], [`select`], [`combobox`], [`list_item`] and
//! [`progress`] are verified against real gpui-component widgets in
//! `tests/seams.rs`, which lays each one out headlessly with and without the
//! refinement; the other builders rest on the source citations in their doc
//! comments.
//!
//! Upstream citations in this module are verified against gpui-component 0.6.4,
//! gpui-base 0.6.4 and gpui-pre 0.3.5; the per-side padding and height-rule
//! citations against gpui-component 0.6.6 and gpui-pre 0.3.6.

use gpui::{FontWeight, Pixels, StyleRefinement, Styled, px, relative};
use gpui_component::Size;
use native_theme::theme::{LayoutTheme, ResolvedFontSpec, ResolvedPadding};

use crate::colors::rgba_to_hsla;
use crate::{Native, text_scale_factor};

/// `font.size × s` in pixels (spec §9.4).
fn scaled_text_size(font: &ResolvedFontSpec, n: Native<'_>) -> Pixels {
    px(font.size * text_scale_factor(n.accessibility))
}

/// CSS weight (100–900) as GPUI's `FontWeight`.
fn weight_of(font: &ResolvedFontSpec) -> FontWeight {
    FontWeight(f32::from(font.weight))
}

/// Text size and weight from a font spec.
fn with_text(r: StyleRefinement, font: &ResolvedFontSpec, n: Native<'_>) -> StyleRefinement {
    r.text_size(scaled_text_size(font, n))
        .font_weight(weight_of(font))
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

/// The style property a control's stated height goes through.
#[derive(Clone, Copy)]
enum HeightProp {
    /// `Styled::h`: the button, input, menu row and list row.
    Height,
    /// `Styled::min_h`, at every text-scaling factor: the select and
    /// combobox.
    MinHeight,
}

/// The control-height rule (module doc, rationale §5 point 3).
///
/// The platform's `defaults.line_height` becomes the control's line height:
/// each receiving widget takes the refinement after its own line height
/// (`button/button.rs:689` → `:690`, `input/input.rs:699` → `:719`) or sets
/// none (`list/list_item.rs:182-193`, `select.rs:535-546`,
/// `combobox.rs:980-997`), so the platform's wins. Through `min_h`, `stated`
/// is the control's minimum at every text-scaling factor, and upstream's own
/// height stays: an automatic height would drop upstream's `h_8`, which
/// grows with the scaled rem, and the trigger could come out shorter just
/// above a factor of 1 than at 1. Through `h`, `stated` is the control's
/// height at a factor of 1 or less; above 1 it is its minimum and the height
/// is automatic, so layout grows the control around its drawn text and
/// padding. Without a stated height, the line height alone: the control
/// keeps the toolkit's own height.
fn with_height_rule(
    r: StyleRefinement,
    stated: Option<f32>,
    prop: HeightProp,
    n: Native<'_>,
) -> StyleRefinement {
    let r = r.line_height(relative(n.resolved.defaults.line_height));
    let Some(stated) = stated else {
        return r;
    };
    match prop {
        HeightProp::MinHeight => r.min_h(px(stated)),
        HeightProp::Height if text_scale_factor(n.accessibility) <= 1.0 => r.h(px(stated)),
        HeightProp::Height => r.min_h(px(stated)).h_auto(),
    }
}

/// `Button` (gpui-component `src/button/button.rs:626-663` → refined at `:690`).
/// The label's text size is set on an inner element (`:698-706`), Tier U.
///
/// Height by the control-height rule (module doc), through `h`.
#[must_use]
pub fn button(n: Native<'_>) -> StyleRefinement {
    let b = &n.resolved.button;
    let r = with_height_rule(
        StyleRefinement::default(),
        Some(b.min_height),
        HeightProp::Height,
        n,
    );
    with_padding(r.min_w(px(b.min_width)), &b.border.padding)
        .rounded(px(b.border.corner_radius.max(0.0)))
        .border(px(b.border.line_width))
        .border_color(rgba_to_hsla(b.border.color))
        // The weight, and deliberately not the size. This refinement lands on
        // the button's outer element (`button/button.rs:690`), and GPUI
        // cascades text style to descendants: the label is a child that sets
        // its own size from the `Size` enum (`button_text_size`,
        // `sizing.rs:319-325`, which maps to `text_xs`/`text_sm`/`text_base`)
        // and so would overrule a size from here -- but it sets no weight, and
        // nothing else on that path does either, so the platform's weight
        // arrives. A size set here would be shadowed on the label and still
        // apply to anything else inside, which is worse than not setting it;
        // that remainder is `content_style`, `pub(crate)` upstream (Tier U).
        .font_weight(weight_of(&b.font))
}

/// `Input` root (`src/input/input.rs:704-714` → `:719`).
///
/// Height by the control-height rule (module doc), through `h`. The rule is
/// for a single-line field: a multi-line `Input` sets its own height before
/// this refinement (`input/input.rs:706-709`), which the rule's `h` would
/// replace, so a caller that wants a multi-line height applies its own
/// `Styled::h` after this builder. The padding is a single-line field's too:
/// upstream pads only a single-line root (`input/input.rs:700-702`), so a
/// caller that refines a multi-line `Input` or a `Textarea` clears the
/// refinement's padding as well.
///
/// The stated padding sides reach the root: upstream pads a single-line
/// field (`input_px`/`input_py`, `input/input.rs:701`) before the refinement
/// at `:719`, so the refinement wins, and the field's text element has no
/// padding of its own to double it. One exception: an `Input` with a suffix
/// takes its right padding from upstream *after* the refinement
/// (`input/input.rs:736`, `this.pr(self.size.input_px())`), so there the
/// platform's right side does not arrive.
///
/// When refining an `InputGroup` or `NumberInput` frame, clear the padding
/// sides: the inner Input already pads (`input/group.rs:265-286` and
/// `input/number_input.rs:158-165` each render an `Input`, which pads itself
/// at `input/input.rs:700-702`).
#[must_use]
pub fn input(n: Native<'_>) -> StyleRefinement {
    let i = &n.resolved.input;
    let r = with_height_rule(
        StyleRefinement::default(),
        Some(i.min_height),
        HeightProp::Height,
        n,
    );
    with_text(
        with_padding(r, &i.border.padding)
            .rounded(px(i.border.corner_radius.max(0.0)))
            .border(px(i.border.line_width)),
        &i.font,
        n,
    )
}

/// A menu row the application draws with its own elements
/// (`src/menu/menu_item.rs:103-105` → `:111`).
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
    let r = with_height_rule(
        StyleRefinement::default(),
        m.row_height,
        HeightProp::Height,
        n,
    );
    with_text(
        with_padding(r, &m.border.padding).gap_x(px(m.icon_text_gap)),
        &m.font,
        n,
    )
}

/// `ListItem` (`src/list/list_item.rs:185-187` → `:193`).
///
/// The colour is carried because upstream labels the row with `foreground`
/// (`:189`) one line before it applies this refinement. Today
/// `list.item_font.color` is the window's text colour in every bundled preset,
/// so this changes no pixel; it is what honours a preset that states a row
/// colour of its own.
///
/// Height by the control-height rule (module doc), through `h`, where
/// `list.row_height` is stated.
#[must_use]
pub fn list_item(n: Native<'_>) -> StyleRefinement {
    let l = &n.resolved.list;
    let r = with_height_rule(
        StyleRefinement::default(),
        l.row_height,
        HeightProp::Height,
        n,
    );
    with_coloured_text(with_padding(r, &l.border.padding), &l.item_font, n)
}

/// Application-built `Tooltip` (`src/tooltip.rs:120-125` → `:126`).
///
/// No width: `tooltip.max_width` is the bubble's outer width, and a bubble that
/// states it clamps itself without clamping its text, which then runs out of
/// it. [`tooltip_content`] carries the width to the element the application
/// passes to `Tooltip::element`, where it makes the text wrap; a tooltip built
/// from `Tooltip::new(text)` has no element to put it on and stays as wide as
/// its text.
///
/// The colour is carried because upstream labels a tooltip with
/// `popover_foreground` (`:115`), which this connector fills from
/// `popover.font.color`; 16 of the 32 bundled preset/mode combinations state a
/// different colour for a tooltip than for a popover.
#[must_use]
pub fn tooltip(n: Native<'_>) -> StyleRefinement {
    let t = &n.resolved.tooltip;
    with_coloured_text(
        with_padding(StyleRefinement::default(), &t.border.padding)
            .rounded(px(t.border.corner_radius.max(0.0))),
        &t.font,
        n,
    )
}

/// Upstream draws the bubble with a one-pixel border on every side
/// (`src/tooltip.rs:117`, `border_1()`), which the platform's outer width pays
/// for along with the two paddings.
const TOOLTIP_BORDER: f32 = 1.0;

/// The element an application passes to `Tooltip::element`
/// (`src/tooltip.rs:128-131`): `tooltip.max_width` less the bubble's own
/// horizontal paddings and border, which is the inner box the text wraps in.
///
/// The width has to land here and not on the bubble. The tooltip's content
/// sits in a bare `div()` inside upstream's `h_flex()`, so it is a flex item
/// with an automatic minimum size, and gpui measures text under
/// `AvailableSpace::MinContent` without wrapping it — a wrap width is taken
/// only from a *definite* available width (gpui-pre
/// `src/elements/text.rs:649-656`). The item's minimum is therefore the whole
/// unwrapped line, which a max width on the bubble cannot shrink: the bubble
/// stops at the platform's width and the text carries on past it. Given to the
/// content instead, the same width is what the text wraps at, and the bubble
/// grows to exactly `tooltip.max_width` around it.
///
/// A side the platform does not state is upstream's own `px_2`
/// (`src/tooltip.rs:123`), which is 0.5 rem (gpui-pre-macros
/// `src/styles.rs:949-950`), converted at the font size this connector
/// installs as the rem: `defaults.font.size` × the text-scaling factor
/// (`theme.font_size` in [`crate::to_theme`], which gpui-component's `Root`
/// sets as the window's rem size, `src/root.rs:582`).
///
/// Never negative: a platform that states a width narrower than its own
/// paddings leaves nothing for the text rather than a width gpui would reject.
///
/// Verified against a real `Tooltip` in `tests/seams.rs`.
#[must_use]
pub fn tooltip_content(n: Native<'_>) -> StyleRefinement {
    let t = &n.resolved.tooltip;
    let upstream_side = TOOLTIP_UPSTREAM_PX_REM
        * n.resolved.defaults.font.size
        * text_scale_factor(n.accessibility);
    let left = t.border.padding.left.unwrap_or(upstream_side);
    let right = t.border.padding.right.unwrap_or(upstream_side);
    let inner = t.max_width - left - right - 2.0 * TOOLTIP_BORDER;
    StyleRefinement::default().max_w(px(inner.max(0.0)))
}

/// Upstream's horizontal tooltip padding, `px_2` (`src/tooltip.rs:123`), in
/// rem: gpui's spacing suffix `2` is `rems(0.5)` (gpui-pre-macros
/// `src/styles.rs:949-950`).
const TOOLTIP_UPSTREAM_PX_REM: f32 = 0.5;

/// `Popover` (`src/popover.rs:284` → `:312`). An unstated side keeps
/// upstream's `p_3` (`:284`).
#[must_use]
pub fn popover(n: Native<'_>) -> StyleRefinement {
    let p = &n.resolved.popover;
    with_padding(StyleRefinement::default(), &p.border.padding)
        .rounded(px(p.border.corner_radius.max(0.0)))
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

/// `Dialog` (`src/dialog/dialog.rs:538-548, 616-617` → `:621`); width through
/// [`dialog_max_width`] and `Dialog::max_w`. The radius is the dialog's own
/// (`dialog.border.corner_radius`), which upstream would otherwise take from
/// `radius_lg` (`:616`); the two differ on Adwaita.
///
/// `max_h` no longer reaches upstream's `Dialog`: 0.6.4 clamps it to what is
/// left of the viewport *after* applying this style (`:535`, applied at
/// `:631`, in a block upstream marks "high priority, can't be overridden").
/// `min_h` and the paddings still arrive, because `min_h_24()` runs before the
/// refinement. Kept here for an application-drawn dialog and for the day
/// upstream takes a `max_h` prop (spec v0.5.9 §4, E18).
///
/// Upstream reads the four padding sides back out of this style
/// (`dialog/dialog.rs:540-552`, 0.6.6), starting from 16px on each side, so
/// an unstated side keeps that 16px. It also reuses them as gaps: the gap
/// between the dialog's sections is `max(top, 8px)` (`:620`), and a
/// `DialogContent`'s gap is the bottom padding (`:656`). A platform that
/// states 32 top and 24 bottom, as GNOME does (docs/platform-facts.md §2.22),
/// therefore also spaces the sections by 32 and the content by 24.
#[must_use]
pub fn dialog(n: Native<'_>) -> StyleRefinement {
    let d = &n.resolved.dialog;
    with_padding(StyleRefinement::default(), &d.border.padding)
        .min_h(px(d.min_height))
        .max_h(px(d.max_height))
        .rounded(px(d.border.corner_radius.max(0.0)))
}

/// `InputGroupButton` (`src/input/group.rs:590-593` → `:596`): the platform's
/// button radius, and nothing else.
///
/// gpui-component scales a control's radius with its size — an in-group
/// button is `XSmall` and gets `radius / 2`, `Button` does the same for its
/// small and large roundings (`src/button/button.rs:593-595`) — where
/// native-theme records one radius per widget, whatever its size. [`button`]
/// already restores it for a `Button`; this does the same for the button
/// nested in a field, whose height, width and padding the group sets so that
/// it fits, and which [`button`]'s own height would push out of the field.
#[must_use]
pub fn input_group_button(n: Native<'_>) -> StyleRefinement {
    StyleRefinement::default().rounded(px(n.resolved.button.border.corner_radius.max(0.0)))
}

/// `DialogFooter` (`src/dialog/footer.rs:52` → `:56`).
#[must_use]
pub fn dialog_footer(n: Native<'_>) -> StyleRefinement {
    StyleRefinement::default().gap(px(n.resolved.dialog.button_gap))
}

/// `DialogTitle` (`src/dialog/title.rs:42-43` → `:45`).
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
/// `bordered` (`src/table/data_table.rs:167-171`). Apply this to a `List` or a
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
        .rounded(px(b.corner_radius.max(0.0)))
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
        .rounded(px(p.border.corner_radius.max(0.0)))
        .min_w(px(p.min_width))
}

/// For `GroupBox::content_style` (`src/group_box.rs:105`, applied `:156-161`).
#[must_use]
pub fn group_box_content(n: Native<'_>) -> StyleRefinement {
    let c = &n.resolved.card;
    with_padding(StyleRefinement::default(), &c.border.padding)
        .rounded(px(c.border.corner_radius.max(0.0)))
        .border(px(c.border.line_width))
        .border_color(rgba_to_hsla(c.border.color))
}

/// For `AccordionItem::title_style` (`src/accordion.rs:219`, applied `:300-306`).
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

/// `Checkbox` (`src/checkbox.rs:270-285` → `:286`); the indicator is inner,
/// Tier U.
///
/// No colour, although upstream does label the row with `foreground` (`:274`)
/// before this refinement: a `Checkbox::label` is wrapped in a div that sets
/// `foreground` itself (`:334-339`), so a carried colour would never reach it,
/// and the disabled hook applies `muted_foreground` and only *then* the
/// refinement (`:252-256`), so a carried colour would displace the disabled
/// colour of custom children. [`radio`], whose label child sets no colour of
/// its own, carries it.
#[must_use]
pub fn checkbox(n: Native<'_>) -> StyleRefinement {
    with_text(checkbox_metrics(n), &n.resolved.checkbox.font, n)
}

/// `Radio` (`src/radio.rs:210-225` → `:226`): [`checkbox`]'s metrics, and the
/// colour [`checkbox`] cannot take.
///
/// The colour is carried for the same reason as [`list_item`]: upstream labels
/// the row with `foreground` (`:212`) before applying this refinement, and its
/// disabled muting is on the label child (`:256-257`), which wins over
/// whatever the row carries.
#[must_use]
pub fn radio(n: Native<'_>) -> StyleRefinement {
    with_coloured_text(checkbox_metrics(n), &n.resolved.checkbox.font, n)
}

/// The metrics `Select` and `Combobox` share; only the text colour separates
/// the two.
///
/// Height by the control-height rule (module doc), through `min_h` at every
/// text-scaling factor: upstream gives both triggers their own `h_8` for
/// `Size::Medium` (`input_size`, `sizing.rs:236-237`, `:261-264`), 2 rem at
/// the rem the `Root` installs, `Theme::font_size` (`root.rs:582`), which
/// [`to_theme`](crate::to_theme) scales by the text-scaling factor
/// (`lib.rs:170`). So the
/// trigger is the larger of the stated minimum and upstream's height, which
/// grows with the text.
///
/// The stated padding sides reach the trigger: upstream pads it
/// (`input_size`, `select.rs:544`, `combobox.rs:995`) before the refinement
/// (`select.rs:546`, `combobox.rs:997`), so the refinement wins, and no inner
/// element pads again. The caret sits inside that padded trigger
/// (`select.rs:57-66`, `combobox.rs:1009-1026`), so the right side has no
/// receiver in gpui where the platform measures it to a separate arrow
/// column: WinUI does (spec v0.5.9 unstated-sizes §1.4,
/// docs/platform-facts.md §2.24), so that platform's value is not applied.
fn combo_box_metrics(n: Native<'_>) -> StyleRefinement {
    let c = &n.resolved.combo_box;
    let r = with_height_rule(
        StyleRefinement::default(),
        Some(c.min_height),
        HeightProp::MinHeight,
        n,
    );
    with_padding(r, &c.border.padding)
        .min_w(px(c.min_width))
        .rounded(px(c.border.corner_radius.max(0.0)))
}

/// `Select` (`src/select.rs:535-545` → `:546`); the arrow is inner, Tier U.
/// Height by the control-height rule (module doc), through `min_h` at every
/// text-scaling factor: upstream gives the trigger its own `h_8` for
/// `Size::Medium` (`input_size`, `sizing.rs:236-237`, `:261-264`), 2 rem,
/// which grows with the scaled rem (`root.rs:582`), so the trigger is the
/// larger of the two.
///
/// The stated padding sides reach the trigger: upstream pads it
/// (`input_size`, `select.rs:544`) before the refinement (`:546`), so the
/// refinement wins, and no inner element pads again. The caret sits inside
/// that padded trigger (`select.rs:57-66`), so the right side has no receiver
/// in gpui where the platform measures it to a separate arrow column: WinUI
/// does (spec v0.5.9 unstated-sizes §1.4, docs/platform-facts.md §2.24), so
/// that platform's value is not applied.
///
/// The colour is carried for the same reason as [`list_item`]: upstream labels
/// the trigger with `foreground` through `input_style`
/// (`src/input/input.rs:105`, applied at `select.rs:539`) before applying this
/// refinement. It is safe here although the same helper's disabled branch
/// returns `muted_foreground` before the refinement too
/// (`src/input/input.rs:99-103`), because `Select` re-mutes its title *child*
/// when disabled (`select.rs:477-479`) and a child wins over the trigger.
#[must_use]
pub fn select(n: Native<'_>) -> StyleRefinement {
    with_coloured_text(combo_box_metrics(n), &n.resolved.combo_box.font, n)
}

/// `Combobox` (`src/combobox.rs:980-996` → `:997`): [`select`]'s metrics, and
/// the colour [`select`] can take but this cannot.
///
/// Upstream labels the trigger with the same `input_style` `foreground`
/// (`:990`), but its disabled branch (`src/input/input.rs:99-103`) delivers
/// `muted_foreground` through that same call, before the refinement at `:997`,
/// and the selected-title child (`:584-590`) sets no colour to re-mute with —
/// so a carried colour would beat the disabled colour instead of yielding to
/// it.
#[must_use]
pub fn combobox(n: Native<'_>) -> StyleRefinement {
    with_text(combo_box_metrics(n), &n.resolved.combo_box.font, n)
}

/// `TitleBar` (`src/title_bar.rs:335` → `:343`); the height has no theme field.
///
/// Upstream sets no text colour on the bar (`:328-343` paints the fill and the
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

/// An application-drawn toolbar row (docs/platform-facts.md §2.13):
/// `toolbar.bar_height` as its minimum height where the platform states one
/// -- a toolbar that sizes to its content, as KDE's does, states none, and a
/// fixed height would be an invention there --, `toolbar.item_gap` between
/// items, the `toolbar.border` padding sides the platform states,
/// `toolbar.background_color`, and `toolbar.font` size and weight. The row
/// is the application's own, so an unstated side or height is left to the
/// application. No edge: §2.13 states none; an application that wants a
/// rule draws a Separator.
#[must_use]
pub fn toolbar(n: Native<'_>) -> StyleRefinement {
    let t = &n.resolved.toolbar;
    let r = with_text(StyleRefinement::default(), &t.font, n);
    let r = match t.bar_height {
        Some(h) => r.min_h(px(h)),
        None => r,
    };
    with_padding(r.gap(px(t.item_gap)), &t.border.padding).bg(rgba_to_hsla(t.background_color))
}

// --- Size helpers (spec §9.3) -------------------------------------------------

/// `Spinner::with_size` (`src/spinner.rs:53-65` → `Icon`, `src/icon.rs:182`;
/// 0.6.4 merged the two sizing arms 0.6.0 had at `:160` and `:189`).
#[must_use]
pub fn spinner_size(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.spinner.diameter))
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

/// For `Dialog::max_w` (`src/dialog/dialog.rs:419`).
#[must_use]
pub fn dialog_max_width(n: Native<'_>) -> Pixels {
    px(n.resolved.dialog.max_width)
}

/// An `Input`'s height rule alone: the line height and height [`input`]
/// sets, and nothing else of its refinement.
///
/// A single-line `Input` takes it through `StyledExt::refine_style`, the
/// caller's style, which `Input` refines its root with last
/// (`src/input/input.rs:719`). Above a text-scaling factor of 1 the rule's
/// height is automatic, so the field then grows around its own drawn text
/// and padding -- upstream's, unless the caller also gives it the
/// platform's.
#[must_use]
pub fn input_height(n: Native<'_>) -> StyleRefinement {
    with_height_rule(
        StyleRefinement::default(),
        Some(n.resolved.input.min_height),
        HeightProp::Height,
        n,
    )
}

// --- Layout accessors (spec §9.5) ---------------------------------------------
// The input is `Theme::layout` on the preset path and `SystemTheme.layout` on
// the system path. There is no receiver to map these into: gpui-component's
// `Theme::spacing_tokens()` returns `SpacingTokens::default()` with no field
// behind it (`theme/mod.rs:482-484`), so its one reader, a `Dialog`'s viewport
// margin (`dialog/dialog.rs:528`), gets that default whatever a theme says.
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
        AccessibilityPreferences {
            text_scaling_factor: factor,
            ..AccessibilityPreferences::default()
        }
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
    fn assert_padding(out: &StyleRefinement, p: &ResolvedPadding, what: &str) {
        assert_eq!(out.padding.top, side(p.top), "{what}: top");
        assert_eq!(out.padding.right, side(p.right), "{what}: right");
        assert_eq!(out.padding.bottom, side(p.bottom), "{what}: bottom");
        assert_eq!(out.padding.left, side(p.left), "{what}: left");
    }
    /// The control-height rule: the platform's line height; through `min_h`
    /// the stated minimum at every scale, with no height of its own; through
    /// `h` the stated height at s <= 1, above 1 a stated minimum and an
    /// automatic height.
    fn assert_height_rule(
        out: &StyleRefinement,
        r: &ResolvedTheme,
        stated: Option<f32>,
        s: f32,
        through_min_h: bool,
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
        if through_min_h {
            assert_eq!(out.min_size.height, len(stated), "{what}: min height");
            assert_eq!(out.size.height, None, "{what}: no height of its own");
        } else if s <= 1.0 {
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

    const CASES: &[(&str, ColorMode)] = &[
        ("catppuccin-mocha", ColorMode::Dark),
        ("catppuccin-latte", ColorMode::Light),
    ];
    const FACTORS: &[f32] = &[1.0, 1.5, 0.8];

    /// Runs `check` for both presets and both factors with a `Native` view.
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

    fn assert_text(out: &StyleRefinement, font: &ResolvedFontSpec, s: f32) {
        assert_eq!(out.text.font_size, abs(font.size * s));
        assert_eq!(
            out.text.font_weight,
            Some(FontWeight(f32::from(font.weight)))
        );
    }

    #[test]
    fn button_refinement_matches_theme_values() {
        for_each_case(|r, s, n| {
            let b = &r.button;
            let out = button(n);
            assert_height_rule(&out, r, Some(b.min_height), s, false, "button");
            assert_eq!(out.min_size.width, len(b.min_width));
            assert_padding(&out, &b.border.padding, "button");
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
    }

    #[test]
    fn input_refinement_matches_theme_values() {
        for_each_case(|r, s, n| {
            let i = &r.input;
            let out = input(n);
            assert_height_rule(&out, r, Some(i.min_height), s, false, "input");
            assert_padding(&out, &i.border.padding, "input");
            assert_eq!(
                out.corner_radii.top_left,
                abs(i.border.corner_radius.max(0.0))
            );
            assert_eq!(out.border_widths.top, abs(i.border.line_width));
            assert_text(&out, &i.font, s);
        });
    }

    /// `input_height` is the height rule [`input`] applies, and nothing else
    /// of its refinement.
    #[test]
    fn input_height_is_the_height_rule_alone() {
        for_each_case(|r, s, n| {
            let out = input_height(n);
            assert_height_rule(&out, r, Some(r.input.min_height), s, false, "input_height");
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
        for_each_case(|r, s, n| {
            let m = &r.menu;
            let out = menu_item(n);
            assert_height_rule(&out, r, m.row_height, s, false, "menu_item");
            assert_padding(&out, &m.border.padding, "menu_item");
            assert_eq!(out.gap.width, def(m.icon_text_gap));
            assert_eq!(out.gap.height, None, "gap_x sets the column gap only");
            assert_text(&out, &m.font, s);

            let l = &r.list;
            let out = list_item(n);
            assert_height_rule(&out, r, l.row_height, s, false, "list_item");
            assert_padding(&out, &l.border.padding, "list_item");
            assert_text(&out, &l.item_font, s);
        });
    }

    #[test]
    fn tooltip_popover_status_bar_match_theme_values() {
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
                    - 2.0 * TOOLTIP_BORDER)
                    .max(0.0))
            );
            assert_padding(&out, &t.border.padding, "tooltip");
            assert_eq!(
                out.corner_radii.top_left,
                abs(t.border.corner_radius.max(0.0))
            );
            assert_text(&out, &t.font, s);

            let p = &r.popover;
            let out = popover(n);
            assert_padding(&out, &p.border.padding, "popover");
            assert_eq!(
                out.corner_radii.top_left,
                abs(p.border.corner_radius.max(0.0))
            );

            let sb = &r.status_bar;
            let out = status_bar(n);
            assert_padding(&out, &sb.border.padding, "status_bar");
            assert_text(&out, &sb.font, s);
        });
    }

    /// The gutter is the width the base layer installed, and only where the
    /// platform's scrollbars are not overlays. Over every preset in both modes,
    /// because the two `CASES` names are on one side of that line; both sides
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
    /// no colour on that element at all. Seven do, over every preset in both
    /// modes, not just the two `CASES` names.
    ///
    /// The `assert_ne!`s below are the reason the first three exist: each of
    /// those native colours differs from the token upstream would otherwise
    /// paint, so leaving the colour out left the widget labelled with a colour
    /// the platform did not state.
    ///
    /// The other four -- `list_item`, `radio`, `select`, `title_bar` -- get no
    /// such `assert_ne!`, and deliberately: their native font colour equals
    /// `defaults.text_color` in all 32 combinations today, because a preset
    /// that states no `font.color` for a widget inherits the window's
    /// (docs/inheritance-rules.toml:115-136), and `foreground` is fed from
    /// `defaults.text_color`. The colour is carried anyway because nothing
    /// stands in its way -- upstream labels the first three with `foreground`
    /// one refinement earlier and mutes the disabled state on a child
    /// afterwards, and it labels the title bar with nothing at all
    /// (`title_bar.rs:328-343`) -- so the day a source states
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
                // `list/list_item.rs:189` -> `:193`, `radio.rs:212` -> `:226`
                // (child muted at `:256-257`) and `select.rs:539` -> `:546`
                // (through `input_style`, `input/input.rs:105`; the title child
                // re-mutes at `:477-479`).
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
                // (`title_bar.rs:328-343`), so the route is open and nothing
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
                // on a tooltip (`tooltip.rs:115`).
                assert_ne!(
                    r.status_bar.font.color, r.defaults.muted_color,
                    "{at}: status bar colour no longer differs from upstream's \
                     muted_foreground, so this builder's colour proves nothing"
                );
                assert_ne!(
                    r.dialog.body_font.color, r.defaults.muted_color,
                    "{at}: dialog body colour no longer differs from upstream's \
                     muted_foreground"
                );
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
    }

    /// `geometry::checkbox` and `geometry::combobox` carry no colour although
    /// upstream paints both from `foreground`: the route is blocked past the
    /// element the refinement lands on. A `Checkbox::label` sits in a wrapper
    /// that sets `foreground` itself (`checkbox.rs:334-339`), so a carried
    /// colour never reaches it; and both widgets apply their disabled colour
    /// *before* the refinement (`checkbox.rs:252-256`, and
    /// `input_style(disabled, ..)` at `input/input.rs:99-103` feeding
    /// `combobox.rs:990` -> `:997`, whose selected-title branch `:584-590` sets
    /// no colour to re-mute with), so a carried colour would displace it.
    ///
    /// The run therefore also guards the preconditions: every preset states
    /// for both widgets exactly the colour upstream already paints there
    /// (`defaults.text_color` -> `foreground`), so the two builders lose
    /// nothing today.
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
                     (checkbox.rs:334-339) so it cannot arrive, and the \
                     disabled hook (checkbox.rs:252-256) would take it instead \
                     of `muted_foreground`"
                );
                assert_eq!(
                    combobox(n).text.color,
                    None,
                    "{at}: geometry::combobox set a text colour; upstream \
                     applies the disabled `muted_foreground` before this \
                     refinement (input/input.rs:99-103 -> combobox.rs:990 -> \
                     :997), so it would displace the disabled colour"
                );
                assert_eq!(
                    r.checkbox.font.color, r.defaults.text_color,
                    "{at}: this preset states a checkbox label text colour \
                     that gpui-component 0.6.4 gives no route for \
                     (checkbox.rs:334-339) -- record it as a Tier U candidate; \
                     do NOT carry it in the builder, it would displace the \
                     disabled colour (checkbox.rs:252-256)"
                );
                assert_eq!(
                    r.combo_box.font.color, r.defaults.text_color,
                    "{at}: this preset states a combo box text colour that \
                     gpui-component 0.6.4 gives no route for \
                     (combobox.rs:990 -> :997) -- record it as a Tier U \
                     candidate; do NOT carry it in the builder, it would \
                     displace the disabled colour (input/input.rs:99-103)"
                );
            }
        }
    }

    #[test]
    fn dialog_family_matches_theme_values() {
        for_each_case(|r, s, n| {
            let d = &r.dialog;
            let out = dialog(n);
            assert_padding(&out, &d.border.padding, "dialog");
            assert_eq!(out.min_size.height, len(d.min_height));
            assert_eq!(out.max_size.height, len(d.max_height));
            assert_eq!(
                out.corner_radii.top_left,
                abs(d.border.corner_radius.max(0.0))
            );
            assert_eq!(dialog_footer(n).gap.width, def(d.button_gap));
            assert_eq!(dialog_footer(n).gap.height, def(d.button_gap));
            assert_text(&dialog_title(n), &d.title_font, s);
            assert_text(&dialog_description(n), &d.body_font, s);
            assert_eq!(dialog_max_width(n), px(d.max_width));
            assert_text(&table(n), &r.list.item_font, s);
            assert_text(&title_bar(n), &r.window.title_bar_font, s);
        });
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
            assert_padding(&out, &c.border.padding, "group_box_content");
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
    }

    #[test]
    fn checkbox_radio_select_match_theme_values() {
        for_each_case(|r, s, n| {
            let c = &r.checkbox;
            let out = checkbox(n);
            assert_eq!(out.gap.width, def(c.label_gap));
            assert_text(&out, &c.font, s);
            // platform-facts §2.5: radio metrics are the checkbox's.
            assert_eq!(radio(n).gap.width, def(c.label_gap));
            assert_text(&radio(n), &c.font, s);

            let cb = &r.combo_box;
            let out = select(n);
            assert_height_rule(&out, r, Some(cb.min_height), s, true, "select");
            assert_padding(&out, &cb.border.padding, "select");
            assert_eq!(out.min_size.width, len(cb.min_width));
            assert_eq!(
                out.corner_radii.top_left,
                abs(cb.border.corner_radius.max(0.0))
            );
            assert_text(&out, &cb.font, s);
            assert_eq!(combobox(n).min_size.width, len(cb.min_width));
            assert_height_rule(&combobox(n), r, Some(cb.min_height), s, true, "combobox");
            assert_padding(&combobox(n), &cb.border.padding, "combobox");
        });
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
                assert_eq!(out.size.height, None, "{at}: the height is a floor");
                assert_eq!(out.gap.width, def(t.item_gap), "{at}: item gap");
                assert_padding(&out, &t.border.padding, &at);
                assert_eq!(
                    out.background,
                    Some(rgba_to_hsla(t.background_color).into()),
                    "{at}: background"
                );
                assert_text(&out, &t.font, 1.0);
                assert_eq!(out.text.color, None, "{at}: no text colour");
                assert_eq!(
                    out.border_widths,
                    StyleRefinement::default().border_widths,
                    "{at}: §2.13 states no edge"
                );
                assert_eq!(out.border_color, None, "{at}: §2.13 states no edge");
            }
        }
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

    /// Without a stated bar height the application's row keeps its own.
    #[test]
    fn toolbar_leaves_min_height_unset_without_a_bar_height() {
        let mut r = resolved("catppuccin-mocha", ColorMode::Dark);
        r.toolbar.bar_height = None;
        let out = toolbar(Native::unscaled(&r));
        assert_eq!(out.min_size.height, None);
        assert_eq!(out.size.height, None);
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
                len(r.tooltip.max_width - 3.0 - 0.5 * rem - 2.0 * TOOLTIP_BORDER),
                "at s = {s}"
            );
        }
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
