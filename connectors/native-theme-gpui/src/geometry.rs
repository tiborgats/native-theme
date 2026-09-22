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
//! do not (spec §3.4). Every value is a `ResolvedTheme` field or one of the
//! two derivations in spec §9.4 (`scaled_text_size`, [`control_height`]), with
//! two conversions the receiving widget forces: [`tooltip_content`] turns the
//! platform's outer tooltip width into the inner box its text wraps in, and
//! [`scrollbar_gutter`] turns the scrollbar groove width the base layer
//! installs into the strip a non-overlay bar needs beside the content.
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
//! gpui-base 0.6.4 and gpui-pre 0.3.5.

use gpui::{FontWeight, Pixels, StyleRefinement, Styled, px};
use gpui_component::Size;
use native_theme::theme::{LayoutTheme, ResolvedBorderSpec, ResolvedFontSpec};

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

/// Control height (spec §9.4, rationale §5.3):
/// `max(theme_height, ceil(font.size × s × defaults.line_height) + 2 × padding_vertical)`.
///
/// At `s = 1` a platform's declared height already accommodates its text, so
/// this returns the theme's own value; it grows only when scaled text would be
/// clipped by gpui-component's fixed heights.
#[must_use]
pub fn control_height(
    theme_height: f32,
    font: &ResolvedFontSpec,
    border: &ResolvedBorderSpec,
    n: Native<'_>,
) -> Pixels {
    let text =
        (font.size * text_scale_factor(n.accessibility) * n.resolved.defaults.line_height).ceil();
    px(theme_height.max(text + 2.0 * border.padding_vertical))
}

/// `Button` (gpui-component `src/button/button.rs:626-663` → refined at `:690`).
/// The label's text size is set on an inner element (`:698-706`), Tier U.
#[must_use]
pub fn button(n: Native<'_>) -> StyleRefinement {
    let b = &n.resolved.button;
    StyleRefinement::default()
        .h(control_height(b.min_height, &b.font, &b.border, n))
        .min_w(px(b.min_width))
        .px(px(b.border.padding_horizontal))
        .py(px(b.border.padding_vertical))
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

/// `Input` root (`src/input/input.rs:704-714` → `:719`); padding is inner, Tier U.
#[must_use]
pub fn input(n: Native<'_>) -> StyleRefinement {
    let i = &n.resolved.input;
    with_text(
        StyleRefinement::default()
            .h(control_height(i.min_height, &i.font, &i.border, n))
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
#[must_use]
pub fn menu_item(n: Native<'_>) -> StyleRefinement {
    let m = &n.resolved.menu;
    with_text(
        StyleRefinement::default()
            .h(control_height(m.row_height, &m.font, &m.border, n))
            .px(px(m.border.padding_horizontal))
            .py(px(m.border.padding_vertical))
            .gap_x(px(m.icon_text_gap)),
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
#[must_use]
pub fn list_item(n: Native<'_>) -> StyleRefinement {
    let l = &n.resolved.list;
    with_coloured_text(
        StyleRefinement::default()
            .h(control_height(l.row_height, &l.item_font, &l.border, n))
            .px(px(l.border.padding_horizontal))
            .py(px(l.border.padding_vertical)),
        &l.item_font,
        n,
    )
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
        StyleRefinement::default()
            .px(px(t.border.padding_horizontal))
            .py(px(t.border.padding_vertical))
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
/// Never negative: a platform that states a width narrower than its own
/// paddings leaves nothing for the text rather than a width gpui would reject.
///
/// Verified against a real `Tooltip` in `tests/seams.rs`.
#[must_use]
pub fn tooltip_content(n: Native<'_>) -> StyleRefinement {
    let t = &n.resolved.tooltip;
    let inner = t.max_width - 2.0 * (t.border.padding_horizontal + TOOLTIP_BORDER);
    StyleRefinement::default().max_w(px(inner.max(0.0)))
}

/// `Popover` (`src/popover.rs:284` → `:312`).
#[must_use]
pub fn popover(n: Native<'_>) -> StyleRefinement {
    let p = &n.resolved.popover;
    StyleRefinement::default()
        .px(px(p.border.padding_horizontal))
        .py(px(p.border.padding_vertical))
        .rounded(px(p.border.corner_radius.max(0.0)))
}

/// `StatusBar` (`src/status_bar.rs:88-90` → `:96`).
///
/// The colour is carried because upstream labels the bar with
/// `muted_foreground` (`:95`), one line before it applies this refinement, and
/// every preset states a status-bar colour of its own that `ThemeColor` has no
/// field for.
#[must_use]
pub fn status_bar(n: Native<'_>) -> StyleRefinement {
    let s = &n.resolved.status_bar;
    with_coloured_text(
        StyleRefinement::default()
            .px(px(s.border.padding_horizontal))
            .py(px(s.border.padding_vertical)),
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
#[must_use]
pub fn dialog(n: Native<'_>) -> StyleRefinement {
    let d = &n.resolved.dialog;
    StyleRefinement::default()
        .px(px(d.border.padding_horizontal))
        .py(px(d.border.padding_vertical))
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
    StyleRefinement::default()
        .px(px(c.border.padding_horizontal))
        .py(px(c.border.padding_vertical))
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
fn combo_box_metrics(n: Native<'_>) -> StyleRefinement {
    let c = &n.resolved.combo_box;
    StyleRefinement::default()
        .min_h(control_height(c.min_height, &c.font, &c.border, n))
        .min_w(px(c.min_width))
        .rounded(px(c.border.corner_radius.max(0.0)))
}

/// `Select` (`src/select.rs:535-545` → `:546`); the arrow is inner, Tier U.
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

// --- Size helpers (spec §9.3) -------------------------------------------------

/// `Spinner::with_size` (`src/spinner.rs:53-65` → `Icon`, `src/icon.rs:182`;
/// 0.6.4 merged the two sizing arms 0.6.0 had at `:160` and `:189`).
#[must_use]
pub fn spinner_size(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.spinner.diameter))
}

/// `Icon::with_size` for toolbar icons (`defaults.icon_sizes.toolbar`).
#[must_use]
pub fn icon_size_toolbar(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.defaults.icon_sizes.toolbar))
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

/// For `Input::h` (`src/input/input.rs:257`): the same control height [`input`] sets.
#[must_use]
pub fn input_height(n: Native<'_>) -> Pixels {
    let i = &n.resolved.input;
    control_height(i.min_height, &i.font, &i.border, n)
}

// --- Layout accessors (spec §9.5) ---------------------------------------------
// The input is `Theme::layout` on the preset path and `SystemTheme.layout` on
// the system path. No gpui-component widget reads the gpui-base spacing
// tokens, so there is no receiver to map these into; `None` means the
// platform specifies nothing (platform-facts §2.20).

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

    const CASES: &[(&str, ColorMode)] = &[
        ("catppuccin-mocha", ColorMode::Dark),
        ("catppuccin-latte", ColorMode::Light),
    ];
    const FACTORS: &[f32] = &[1.0, 1.5];

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
        for_each_case(|r, _s, n| {
            let b = &r.button;
            let out = button(n);
            assert_eq!(
                out.size.height,
                Some(control_height(b.min_height, &b.font, &b.border, n).into())
            );
            assert_eq!(out.min_size.width, len(b.min_width));
            assert_eq!(out.padding.left, def(b.border.padding_horizontal));
            assert_eq!(out.padding.right, def(b.border.padding_horizontal));
            assert_eq!(out.padding.top, def(b.border.padding_vertical));
            assert_eq!(out.padding.bottom, def(b.border.padding_vertical));
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
            assert_eq!(
                out.size.height,
                Some(control_height(i.min_height, &i.font, &i.border, n).into())
            );
            assert_eq!(
                out.corner_radii.top_left,
                abs(i.border.corner_radius.max(0.0))
            );
            assert_eq!(out.border_widths.top, abs(i.border.line_width));
            assert_text(&out, &i.font, s);
            assert_eq!(
                input_height(n),
                control_height(i.min_height, &i.font, &i.border, n)
            );
        });
    }

    #[test]
    fn menu_and_list_items_match_theme_values() {
        for_each_case(|r, s, n| {
            let m = &r.menu;
            let out = menu_item(n);
            assert_eq!(
                out.size.height,
                Some(control_height(m.row_height, &m.font, &m.border, n).into())
            );
            assert_eq!(out.padding.left, def(m.border.padding_horizontal));
            assert_eq!(out.padding.top, def(m.border.padding_vertical));
            assert_eq!(out.gap.width, def(m.icon_text_gap));
            assert_eq!(out.gap.height, None, "gap_x sets the column gap only");
            assert_text(&out, &m.font, s);

            let l = &r.list;
            let out = list_item(n);
            assert_eq!(
                out.size.height,
                Some(control_height(l.row_height, &l.item_font, &l.border, n).into())
            );
            assert_eq!(out.padding.left, def(l.border.padding_horizontal));
            assert_eq!(out.padding.top, def(l.border.padding_vertical));
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
            assert_eq!(
                tooltip_content(n).max_size.width,
                len(t.max_width - 2.0 * (t.border.padding_horizontal + TOOLTIP_BORDER))
            );
            assert_eq!(out.padding.left, def(t.border.padding_horizontal));
            assert_eq!(out.padding.top, def(t.border.padding_vertical));
            assert_eq!(
                out.corner_radii.top_left,
                abs(t.border.corner_radius.max(0.0))
            );
            assert_text(&out, &t.font, s);

            let p = &r.popover;
            let out = popover(n);
            assert_eq!(out.padding.left, def(p.border.padding_horizontal));
            assert_eq!(out.padding.top, def(p.border.padding_vertical));
            assert_eq!(
                out.corner_radii.top_left,
                abs(p.border.corner_radius.max(0.0))
            );

            let sb = &r.status_bar;
            let out = status_bar(n);
            assert_eq!(out.padding.left, def(sb.border.padding_horizontal));
            assert_eq!(out.padding.top, def(sb.border.padding_vertical));
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
            assert_eq!(out.padding.left, def(d.border.padding_horizontal));
            assert_eq!(out.padding.top, def(d.border.padding_vertical));
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
            assert_eq!(out.padding.left, def(c.border.padding_horizontal));
            assert_eq!(out.padding.top, def(c.border.padding_vertical));
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
            assert_eq!(
                out.min_size.height,
                Some(control_height(cb.min_height, &cb.font, &cb.border, n).into())
            );
            assert_eq!(out.min_size.width, len(cb.min_width));
            assert_eq!(
                out.corner_radii.top_left,
                abs(cb.border.corner_radius.max(0.0))
            );
            assert_text(&out, &cb.font, s);
            assert_eq!(combobox(n).min_size.width, len(cb.min_width));
        });
    }

    #[test]
    fn size_helpers_use_theme_values() {
        for_each_case(|r, _s, n| {
            assert_eq!(spinner_size(n), Size::Size(px(r.spinner.diameter)));
            let is = &r.defaults.icon_sizes;
            assert_eq!(icon_size_toolbar(n), Size::Size(px(is.toolbar)));
            assert_eq!(icon_size_small(n), Size::Size(px(is.small)));
            assert_eq!(icon_size_large(n), Size::Size(px(is.large)));
            assert_eq!(icon_size_dialog(n), Size::Size(px(is.dialog)));
            assert_eq!(icon_size_panel(n), Size::Size(px(is.panel)));
        });
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

    /// §9.4, rationale §2.23: at s = 1 the platform's own height wins.
    #[test]
    fn control_height_returns_theme_height_when_text_fits() {
        for preset in ["kde-breeze", "adwaita"] {
            let r = resolved(preset, ColorMode::Light);
            let prefs = scaled(1.0);
            let n = Native {
                resolved: &r,
                accessibility: &prefs,
            };
            let b = &r.button;
            let text =
                (b.font.size * r.defaults.line_height).ceil() + 2.0 * b.border.padding_vertical;
            assert!(
                text <= b.min_height,
                "{preset}: precondition, text {text} must fit in {}",
                b.min_height
            );
            assert_eq!(
                control_height(b.min_height, &b.font, &b.border, n),
                px(b.min_height)
            );
        }
    }

    /// §9.4: at s = 1.5 scaled text no longer fits and the height grows.
    #[test]
    fn control_height_grows_when_scaled_text_does_not_fit() {
        for preset in ["kde-breeze", "adwaita"] {
            let r = resolved(preset, ColorMode::Light);
            let prefs = scaled(1.5);
            let n = Native {
                resolved: &r,
                accessibility: &prefs,
            };
            let b = &r.button;
            let text = (b.font.size * 1.5 * r.defaults.line_height).ceil()
                + 2.0 * b.border.padding_vertical;
            assert!(
                text > b.min_height,
                "{preset}: precondition, scaled text {text} must exceed {}",
                b.min_height
            );
            assert_eq!(
                control_height(b.min_height, &b.font, &b.border, n),
                px(text)
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
