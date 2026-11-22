//! Per-widget geometry for gpui-component widgets.
//!
//! Every builder is a pure function of a [`Native`] view and returns a
//! [`StyleRefinement`] the application applies with
//! `gpui_component::StyledExt::refine_style`; the four layout accessors,
//! [`widget_gap`], [`container_margin`], [`window_margin`] and
//! [`section_gap`], take a `LayoutTheme` instead and return a length:
//!
//! ```no_run
//! use gpui_kit::component::{StyledExt, button::Button};
//! use native_theme_gpui::{ActiveNativeTheme, geometry};
//! # fn f(cx: &gpui_kit::App) {
//! let button = Button::new("save").label("Save");
//! let button = match cx.native() { Some(n) => button.refine_style(&geometry::button(n)), None => button };
//! # }
//! ```
//!
//! Each widget applies the caller's refinement after its own geometry at the
//! cited upstream line, so the values here win. Text sizes carry the
//! accessibility text-scaling factor; widths, paddings, radii and icon sizes
//! do not, as in the platform toolkits. Every builder that sets a text size also sets the
//! platform's `defaults.line_height` and the font's family, weight and style.
//! Every value is a `ResolvedTheme` field or the one
//! derivation, a font size times the text-scaling factor
//! (`scaled_text_size`), with two conversions the
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
//! Geometry, with one exception that is not geometry: twelve builders also
//! carry the platform's text colour. A builder carries it only where the
//! carried colour actually reaches the text and upstream's state colours
//! (disabled, muted) still win — either because upstream labels the very
//! element the refinement lands on with a token of its own one refinement
//! earlier and applies its state colours after the refinement or on a child,
//! or because upstream sets no colour on that element at all, so there is
//! nothing to displace. Five of the twelve displace a visibly different colour
//! today — [`status_bar`] and [`dialog_description`] displace
//! `muted_foreground`, [`tooltip`] displaces `popover_foreground`, and
//! [`input`] and [`text_area`] the window's `foreground`, which the field's
//! text inherits and the macOS presets' `input.font.color` differs from —
//! four displace `foreground`: [`list_item`], [`radio`], [`select`] and
//! [`accordion_title`], and [`title_bar`], [`dialog_title`] and [`toolbar`]
//! displace nothing (`title_bar.rs:332-349` paints the fill and the border,
//! then refines; `dialog/title.rs:39-46` and `toolbar.rs:243-257` set no
//! colour). Those seven change no pixel over the bundled presets but one --
//! ios-13's [`select`], whose `combo_box.font.color` is the accent -- because
//! a widget that states no `font.color` inherits the window's
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
//! either never arrive or displace the disabled one (`checkbox.rs:342-347` and
//! `:263-267`, `input/input.rs:120-124` through `combobox.rs:1016`).
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
//! Upstream citations in this module are verified against gpui-component 0.9.1,
//! gpui-base 0.9.1 and gpui-pre 0.3.12, but where one names its version.

use gpui::{BoxShadow, FontWeight, Hsla, Pixels, StyleRefinement, Styled, px, relative};
use gpui_component::Size;
use gpui_component::dialog::DialogEntrance;
use native_theme::theme::{DialogEntranceStyle, LayoutTheme, ResolvedFontSpec, ResolvedPadding};

use crate::Native;
use crate::colors::rgba_to_hsla;

/// `font.size × s` (spec §9.4).
fn scaled_text_size(font: &ResolvedFontSpec, n: Native<'_>) -> f32 {
    crate::scaled_text_size(font.size, n.accessibility)
}

/// A CSS weight (100–900) as GPUI's `FontWeight`.
pub(crate) fn css_weight(weight: u16) -> FontWeight {
    FontWeight(f32::from(weight))
}

/// A font as the builders set it, read from a 0.7.2 resolved theme by
/// [`font_of`] and from the 0.8.0 model by the connector's `next` module:
/// the family as the theme states it, the size already scaled by the
/// text-scaling factor, the CSS weight, the slant and the colour.
pub(crate) struct Font<'a> {
    pub(crate) family: &'a std::sync::Arc<str>,
    pub(crate) size: f32,
    pub(crate) weight: u16,
    pub(crate) style: gpui::FontStyle,
    pub(crate) color: Hsla,
}

/// The [`Font`] of a 0.7.2 font spec, its size scaled by `n`'s factor.
fn font_of<'a>(font: &'a ResolvedFontSpec, n: Native<'_>) -> Font<'a> {
    Font {
        family: &font.family,
        size: scaled_text_size(font, n),
        weight: font.weight,
        style: crate::font_style(font.style),
        color: rgba_to_hsla(font.color),
    }
}

/// A 0.7.2 padding's four sides, top, right, bottom, left.
fn sides_of(p: &ResolvedPadding) -> [Option<f32>; 4] {
    [p.top, p.right, p.bottom, p.left]
}

/// Family, text size, weight and style from a font spec, at the platform's
/// `defaults.line_height`.
///
/// The family goes through [`crate::font_family`], the path the installed
/// theme's own `font_family` takes, so a widget whose font is the window's
/// gets the very family the window cascades; a family of its own (the KDE
/// reader states one for `menu.font`, `toolbar.font` and
/// `window.title_bar_font`) arrives in its place.
///
/// The line height goes wherever a text size does: gpui's own default is the
/// golden ratio (gpui-pre `src/style.rs:494`), which upstream leaves in place
/// on most text it sets (`src/tooltip.rs:114-137`, `src/status_bar.rs:86-96`)
/// and replaces with `relative(1.25)` on a dialog title before the caller's
/// refinement (`src/dialog/title.rs:44` → `:45`), so wrapped text would
/// otherwise be spaced by a metric of gpui's or upstream's own.
fn with_text(r: StyleRefinement, font: &ResolvedFontSpec, n: Native<'_>) -> StyleRefinement {
    text(r, &font_of(font, n), n.resolved.defaults.line_height)
}

/// [`with_text`] of a [`Font`] at the line height `line_height`.
pub(crate) fn text(r: StyleRefinement, font: &Font<'_>, line_height: f32) -> StyleRefinement {
    let mut r = line_height_of(r.text_size(px(font.size)), line_height)
        .font_family(crate::font_family(font.family))
        .font_weight(css_weight(font.weight));
    r.text_style().font_style = Some(font.style);
    r
}

/// The platform's `defaults.line_height` alone: [`with_text`]'s, for the
/// builders that set a control's line height and no text size, [`button`],
/// [`input_height`] and [`editor`].
fn with_line_height(r: StyleRefinement, n: Native<'_>) -> StyleRefinement {
    line_height_of(r, n.resolved.defaults.line_height)
}

/// [`with_line_height`] of the line height `line_height`.
pub(crate) fn line_height_of(r: StyleRefinement, line_height: f32) -> StyleRefinement {
    r.line_height(relative(line_height))
}

/// Family, text size, weight, style and colour from a font spec.
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
    coloured_text(r, &font_of(font, n), n.resolved.defaults.line_height)
}

/// [`with_coloured_text`] of a [`Font`] at the line height `line_height`.
pub(crate) fn coloured_text(
    r: StyleRefinement,
    font: &Font<'_>,
    line_height: f32,
) -> StyleRefinement {
    text(r, font, line_height).text_color(font.color)
}

/// Each side the theme states, and no other: an unstated side leaves the
/// receiving widget's own padding in place.
fn with_padding(r: StyleRefinement, p: &ResolvedPadding) -> StyleRefinement {
    padding(r, sides_of(p))
}

/// [`with_padding`] of the four sides top, right, bottom, left.
pub(crate) fn padding(
    r: StyleRefinement,
    [top, right, bottom, left]: [Option<f32>; 4],
) -> StyleRefinement {
    let r = match top {
        Some(v) => r.pt(px(v)),
        None => r,
    };
    let r = match right {
        Some(v) => r.pr(px(v)),
        None => r,
    };
    let r = match bottom {
        Some(v) => r.pb(px(v)),
        None => r,
    };
    match left {
        Some(v) => r.pl(px(v)),
        None => r,
    }
}

/// The control-height rule (module doc, rationale §5 point 3).
///
/// The platform's `defaults.line_height` is the control's line height, which
/// [`with_text`] sets, or [`with_line_height`] where no text size is: each
/// receiving widget takes the refinement after its own line height
/// (`button/button.rs:763` → `:764`, `input/input.rs:788` → `:808`) or sets
/// none (`list/list_item.rs:198-209`, `select.rs:569-580`,
/// `combobox.rs:999-1016`), so the platform's wins. `stated` is the control's
/// height at a text-scaling factor of 1 or less; above 1 it is its minimum
/// and the height is automatic, so layout grows the control around its drawn
/// text and padding. Without a stated height the control keeps the toolkit's
/// own height.
fn with_height_rule(r: StyleRefinement, stated: Option<f32>, n: Native<'_>) -> StyleRefinement {
    height_rule(r, stated, n.accessibility.text_scale_factor())
}

/// [`with_height_rule`] at the text-scaling factor `text_scale`.
pub(crate) fn height_rule(
    r: StyleRefinement,
    stated: Option<f32>,
    text_scale: f32,
) -> StyleRefinement {
    let Some(stated) = stated else {
        return r;
    };
    if text_scale <= 1.0 {
        r.h(px(stated))
    } else {
        r.min_h(px(stated)).h_auto()
    }
}

/// `Button` (gpui-component `src/button/button.rs:694-731` → refined at `:764`).
/// The label's text size is set on an inner element (`:774-782`): upstream work.
///
/// Height by the control-height rule (module doc), through `h`.
#[must_use]
pub fn button(n: Native<'_>) -> StyleRefinement {
    let b = &n.resolved.button;
    button_from(&ButtonFrame {
        line_height: n.resolved.defaults.line_height,
        text_scale: n.accessibility.text_scale_factor(),
        min_width: b.min_width,
        min_height: Some(b.min_height),
        padding: sides_of(&b.border.padding),
        radius: b.border.corner_radius,
        border_width: b.border.line_width,
        border_color: rgba_to_hsla(b.border.color),
        weight: b.font.weight,
    })
}

/// The theme's values [`button`] is made of, read from either model.
pub(crate) struct ButtonFrame {
    pub(crate) line_height: f32,
    pub(crate) text_scale: f32,
    pub(crate) min_width: f32,
    /// The stated height, `None` where the theme states none.
    pub(crate) min_height: Option<f32>,
    pub(crate) padding: [Option<f32>; 4],
    pub(crate) radius: f32,
    pub(crate) border_width: f32,
    pub(crate) border_color: Hsla,
    pub(crate) weight: u16,
}

/// [`button`] of `b`.
pub(crate) fn button_from(b: &ButtonFrame) -> StyleRefinement {
    let r = height_rule(
        line_height_of(StyleRefinement::default(), b.line_height),
        b.min_height,
        b.text_scale,
    );
    padding(r.min_w(px(b.min_width)), b.padding)
        .rounded(px(b.radius))
        .border(px(b.border_width))
        .border_color(b.border_color)
        // The weight, and deliberately not the size. This refinement lands on
        // the button's outer element (`button/button.rs:764`), and GPUI
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
        .font_weight(css_weight(b.weight))
}

/// The text of a `Button`'s label, passed as the button's child: `button.font`'s
/// family, size, weight and style, and the platform's line height.
///
/// gpui-component sets the label's size from the `Size` enum on the content
/// element (`button_text_size`, `src/sizing.rs:337-343`, applied at
/// `src/button/button.rs:782`) after the button's own refinement (`:764`), so
/// a size given to [`button`] would be shadowed; that size is `text_sm` at
/// `Size::Medium`, 0.875 of the platform font. The content
/// element's children are the one public route under it (`:805`): a label
/// passed as a child, built like upstream's own label element (`:796-804`)
/// and refined with this. The name upstream derives from `.label()`
/// (`:768-771`) is then given explicitly:
///
/// ```no_run
/// use gpui_kit::{ParentElement, Styled, component::{StyledExt, button::Button}, div};
/// use native_theme_gpui::geometry;
/// # fn f(n: native_theme_gpui::Native<'_>) {
/// Button::new("save")
///     .refine_style(&geometry::button(n))
///     .child(
///         div().min_w_0().whitespace_nowrap().text_ellipsis()
///             .refine_style(&geometry::button_label(n))
///             .child("Save"),
///     )
///     .accessibility_label("Save");
/// # }
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

/// `Toggle` (`src/button/toggle.rs:182-187` → `:227`): [`button_label`]'s
/// text — `button.font`'s family, size, weight and style and the platform's
/// line height.
///
/// gpui-component sets `text_sm` on a `Size::Medium` Toggle's root before
/// the caller's refinement (0.875 of the platform font), so this
/// size wins. Only the text: a `Toggle` is `Ghost` by default (`:15-16`), a
/// flat button, and the model states no flat-button frame, so upstream's own
/// stands. The platform's segmented control is the segmented `TabBar`, not a
/// `ToggleGroup`.
#[must_use]
pub fn toggle(n: Native<'_>) -> StyleRefinement {
    button_label(n)
}

/// `Input` root (`src/input/input.rs:793-803` → `:808`).
///
/// Height by the control-height rule (module doc), through `h`. The rule is
/// for a single-line field: a multi-line `Input` sets its own height before
/// this refinement (`input/input.rs:795-798`), which the rule's `h` would
/// replace, so a caller that wants a multi-line height sets it after this
/// builder, fully qualified as `Styled::h(input, height)`. Method syntax
/// reaches `Input::h` instead (`input/input.rs:313-317`, "Multi-line only"),
/// which shadows `Styled::h` and only records the height that upstream
/// applies at `:797`, before the caller's refinement at `:808`, so the
/// rule's `h` would still replace it. The padding is a single-line field's too:
/// upstream pads only a single-line root (`input/input.rs:789-791`), so a
/// caller that refines a multi-line `Input` or a `Textarea` uses
/// [`text_area`] instead, or clears this refinement's padding as well.
///
/// The stated padding sides reach the root: upstream pads a single-line
/// field (`input_px`/`input_py`, `input/input.rs:790`) before the refinement
/// at `:808`, so the refinement wins, and the field's text element has no
/// padding of its own to double it. One exception: an `Input` with a suffix
/// takes its right padding from upstream *after* the refinement
/// (`input/input.rs:825`, `this.pr(self.size.input_px())`), so there the
/// platform's right side does not arrive.
///
/// When refining an `InputGroup` or `NumberInput` frame, clear the padding
/// sides: the inner Input already pads (`input/group.rs:272-293` and
/// `input/number_input.rs:173-180` each render an `Input`, which pads itself
/// at `input/input.rs:789-791`).
///
/// The text is `input.font.color`. The root sets no text colour of its own
/// (`input/input.rs:715-808`; `input_style`'s foreground is dropped at
/// `:715`), and gpui-base paints the field's text in the colour the root
/// inherits (gpui-base `input/base/element.rs:3182-3185`), so without the
/// refinement it is the window's `foreground`. gpui-base halves that
/// colour's alpha in a disabled field (`:3184-3185`): [`input_fill`] for a
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

/// A multi-line `Input` or a `Textarea` root (`src/input/input.rs:793-803`
/// → `:808`).
///
/// [`input`]'s refinement for the multi-line field: `text_area.border`'s
/// padding, corner radius and line width in place of the input's (the
/// frame's inherit the input's, docs/platform-facts.md §2.29), the input's
/// font at `defaults.line_height`, its colour included as in [`input`], and no
/// height -- the rows the field's
/// state holds size it (gpui-base `input/base/state.rs`,
/// `InputBaseState::auto_grow`).
///
/// **Padding.** Upstream pads a multi-line field's editor, inside the root,
/// by its `Size`'s `input_px` and `input_py`, from render, where no caller
/// reaches it (`input/input.rs:597-611`), and pads no multi-line root
/// (`:789-791`). A `Textarea` renders its `Input` at its own `Size`
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
            .border(px(a.border.line_width)),
        &n.resolved.input.font,
        n,
    )
}

/// The code `Editor`'s line height: the platform's `defaults.line_height`,
/// over the `relative(1.5)` upstream sets on the editor's `Input` first
/// (`src/input/editor.rs:157`); the `Editor` refines the caller's style last
/// (`:174`), so this one wins.
///
/// The font is upstream's choice and already the platform's: the editor is
/// lettered in `Theme::mono_font_family` at `mono_font_size` (`:155-156`),
/// which [`to_theme`](crate::to_theme) fills from `defaults.mono_font`,
/// scaled. Its fill and its gutter's are the highlight theme's, which
/// `to_theme` gives `input.background_color`, and the gutter, where that
/// fill is translucent, the fill over `window.background_color`.
#[must_use]
pub fn editor(n: Native<'_>) -> StyleRefinement {
    with_line_height(StyleRefinement::default(), n)
}

/// A menu row the application draws with its own elements
/// (`src/menu/menu_item.rs:117-119` → `:125`).
///
/// No gpui-component widget takes this style: upstream's `MenuItemElement` is
/// crate-private in a private module (`src/menu/menu_item.rs:10-11`,
/// `src/menu/mod.rs:6`) and `PopupMenu` builds its own rows, so the receiver
/// the v0.5.8 documentation named does not exist.
///
/// Height by the control-height rule (module doc), through `h`, where
/// `menu.row_height` is stated, else at least `menu.min_row_height`.
#[must_use]
pub fn menu_item(n: Native<'_>) -> StyleRefinement {
    let m = &n.resolved.menu;
    let r = row_height_rule(m.row_height, m.min_row_height, n);
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
    list_item_from(
        l.row_height,
        sides_of(&l.border.padding),
        &font_of(&l.item_font, n),
        n.resolved.defaults.line_height,
        n.accessibility.text_scale_factor(),
    )
}

/// [`list_item`] of the stated row height `row_height`, the padding's sides
/// `sides` (top, right, bottom, left), the row's font `font`, the line height
/// `line_height` and the text-scaling factor `text_scale`.
pub(crate) fn list_item_from(
    row_height: Option<f32>,
    sides: [Option<f32>; 4],
    font: &Font<'_>,
    line_height: f32,
    text_scale: f32,
) -> StyleRefinement {
    let r = height_rule(StyleRefinement::default(), row_height, text_scale);
    coloured_text(padding(r, sides), font, line_height)
}

/// Application-built `Tooltip` (`src/tooltip.rs:125-130` → `:131`).
///
/// No width: `tooltip.max_width` is the bubble's outer width, which the
/// application gives the bubble or the text. For a tooltip without a key
/// binding either is the same bubble: the box the text sits in has no
/// minimum width and clips (gpui-component 0.10.0 `src/tooltip.rs:134-144`),
/// so `Tooltip::new(text)` given `max_w(tooltip.max_width)` wraps its text
/// inside the bubble, and the conversion [`tooltip_content`] makes is not
/// needed (`tests/seams.rs`).
/// With a key binding beside the text the two differ: [`tooltip_content`]
/// holds the text to the inner width and the bubble grows past
/// `tooltip.max_width` by the key and the gap before it, while a `max_w` on
/// the bubble keeps it within `tooltip.max_width` and wraps the text beside
/// the key. A tooltip with a key binding therefore keeps to the platform's
/// width only as `Tooltip::new(text)` given `max_w(tooltip.max_width)`.
///
/// The colours are carried because upstream paints a tooltip as a popover:
/// its text `popover_foreground` (`:120`) and its fill `tokens.popover`
/// (`:119`, `:121`), which this connector fills from `popover.font.color` and
/// `popover.background_color`, and its border `border` (`:123`), from
/// `defaults.border.color` -- all before this refinement. 18 of the 34
/// bundled preset/mode combinations state a different text colour for a
/// tooltip than for a popover, and more a different fill: kde-breeze's
/// tooltip is `#f7f7f7` on an `#eff0f1` popover, adwaita's dark on a light
/// one.
///
/// The edge is `tooltip.border.line_width` wide, over upstream's `border_1`
/// (`:122`). The drop shadow is upstream's `shadow_md` (`:124`) where
/// `tooltip.border.shadow_enabled` holds -- its two layers
/// (`TOOLTIP_SHADOW`), the model stating no shadow geometry -- in
/// `defaults.shadow_color`, and none where it does not.
#[must_use]
pub fn tooltip(n: Native<'_>) -> StyleRefinement {
    let t = &n.resolved.tooltip;
    let frame = tooltip_frame(&TooltipFrame {
        padding: sides_of(&t.border.padding),
        radius: t.border.corner_radius,
        background: rgba_to_hsla(t.background_color),
        border_width: t.border.line_width,
        border_color: rgba_to_hsla(t.border.color),
    });
    with_tooltip_shadow(
        with_coloured_text(frame, &t.font, n),
        t.border
            .shadow_enabled
            .then(|| rgba_to_hsla(n.resolved.defaults.shadow_color)),
    )
}

/// The theme's values [`tooltip`]'s bubble is made of, read from either
/// model.
pub(crate) struct TooltipFrame {
    pub(crate) padding: [Option<f32>; 4],
    pub(crate) radius: f32,
    pub(crate) background: Hsla,
    pub(crate) border_width: f32,
    pub(crate) border_color: Hsla,
}

/// [`tooltip`]'s bubble of `t`, before its text and shadow.
pub(crate) fn tooltip_frame(t: &TooltipFrame) -> StyleRefinement {
    padding(StyleRefinement::default(), t.padding)
        .rounded(px(t.radius))
        .bg(t.background)
        .border(px(t.border_width))
        .border_color(t.border_color)
}

/// `r` with [`tooltip`]'s shadow in the colour `ink`, where the theme gives
/// the tooltip a shadow, and with no shadow where `ink` is `None`.
pub(crate) fn with_tooltip_shadow(mut r: StyleRefinement, ink: Option<Hsla>) -> StyleRefinement {
    let shadow = match ink {
        Some(ink) => TOOLTIP_SHADOW
            .iter()
            .map(|&(y, blur, spread)| {
                BoxShadow::new(px(0.), px(y), ink)
                    .blur_radius(px(blur))
                    .spread_radius(px(spread))
            })
            .collect(),
        None => Vec::new(),
    };
    r.box_shadow = Some(shadow);
    r
}

/// The layers of gpui's `shadow_md`, which upstream gives a tooltip
/// (`src/tooltip.rs:124`), as (y offset, blur radius, spread radius) in
/// pixels (gpui-pre-macros 0.3.12 `src/styles.rs:445-448`).
const TOOLTIP_SHADOW: [(f32, f32, f32); 2] = [(4., 6., -1.), (2., 4., -2.)];

/// The element an application passes to `Tooltip::element`
/// (`src/tooltip.rs:133-136`): `tooltip.max_width` less the bubble's own
/// horizontal paddings and its edge, `tooltip.border.line_width` on each
/// side ([`tooltip`]), which is the inner box the text wraps in.
///
/// Given to the content, the width is what the text wraps at, and the bubble
/// grows to exactly `tooltip.max_width` around it. For a tooltip without a
/// key binding it is not needed: gpui-component 0.10.0 gives the box the
/// content sits in a zero minimum width and clips it (gpui-component 0.10.0
/// `src/tooltip.rs:134-144`), so the same width on the bubble wraps the text
/// too, into the same bubble (`tests/seams.rs`). Up to 0.9.1 that box was
/// a bare `div()`, a flex item with an automatic minimum size, and gpui
/// measures text under `AvailableSpace::MinContent` without wrapping it -- a
/// wrap width is taken only from a *definite* available width (gpui-pre
/// `src/elements/text.rs:656-663`) -- so the bubble stopped at the platform's
/// width and the text carried on past it. With a key binding the width here
/// counts the text alone: the key and the gap before it are laid beside it,
/// and the bubble grows past `tooltip.max_width` by them. [`tooltip`] on
/// `Tooltip::new(text)` given `max_w(tooltip.max_width)` keeps to the width,
/// the text wrapping beside the key (`tests/seams.rs`).
///
/// A side the platform does not state is upstream's own `px_2`
/// (`src/tooltip.rs:128`), which is 0.5 rem (gpui-pre-macros
/// `src/styles.rs:949-950`), converted at the font size this connector
/// installs as the rem: `defaults.font.size` scaled by
/// [`crate::scaled_text_size`] (`theme.font_size` in [`crate::to_theme`],
/// which gpui-component's `WindowState` root plugin sets as the window's rem
/// size, `src/root.rs:725`).
///
/// Never negative: a platform that states a width narrower than its own
/// paddings leaves nothing for the text rather than a width gpui would reject.
///
/// Verified against a real `Tooltip` in `tests/seams.rs`.
#[must_use]
pub fn tooltip_content(n: Native<'_>) -> StyleRefinement {
    let t = &n.resolved.tooltip;
    tooltip_content_from(
        t.max_width,
        [t.border.padding.left, t.border.padding.right],
        t.border.line_width,
        crate::scaled_text_size(n.resolved.defaults.font.size, n.accessibility),
    )
}

/// [`tooltip_content`] of a tooltip `max_width` wide with the given left and
/// right paddings and an edge `border_width` wide, under the rem `rem`, the
/// scaled `defaults.font.size`.
pub(crate) fn tooltip_content_from(
    max_width: f32,
    [left, right]: [Option<f32>; 2],
    border_width: f32,
    rem: f32,
) -> StyleRefinement {
    let upstream_side = TOOLTIP_UPSTREAM_PX_REM * rem;
    let left = left.unwrap_or(upstream_side);
    let right = right.unwrap_or(upstream_side);
    let inner = max_width - left - right - 2.0 * border_width;
    StyleRefinement::default().max_w(px(inner.max(0.0)))
}

/// Upstream's horizontal tooltip padding, `px_2` (`src/tooltip.rs:128`), in
/// rem: gpui's spacing suffix `2` is `rems(0.5)` (gpui-pre-macros
/// `src/styles.rs:949-950`).
const TOOLTIP_UPSTREAM_PX_REM: f32 = 0.5;

/// `Popover` (`src/popover.rs:308` → `:360`). An unstated side keeps
/// upstream's `p_3` (`:308`).
///
/// The edge is `popover.border.color` at `popover.border.line_width`, the
/// popover's as the theme states it. Upstream draws no border on a popover:
/// its edge is a 1px ring of `foreground` at low alpha in the shadow
/// (`styled.rs:36-38`, `:186-194`), outside the surface (`:63-67`), and it
/// stays, so the border lies inside it. With `Popover::arrow(true)` the
/// border opens at the arrow's base: upstream paints the background over
/// 1 px on either side of the surface edge there (`popover.rs:384-387`,
/// `arrow_join_bounds` `:473-487`), and strokes the arrow's two slopes in
/// the ring's colour, not `popover.border.color` (`:388-395`).
#[must_use]
pub fn popover(n: Native<'_>) -> StyleRefinement {
    let p = &n.resolved.popover;
    with_padding(StyleRefinement::default(), &p.border.padding)
        .rounded(px(p.border.corner_radius))
        .border(px(p.border.line_width))
        .border_color(rgba_to_hsla(p.border.color))
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

/// `Dialog` (`src/dialog/dialog.rs:692-702, 779-783` → `:787`); width through
/// [`dialog_max_width`] and `Dialog::max_w`. The radius is the dialog's own
/// (`dialog.border.corner_radius`), which upstream would otherwise take from
/// `radius_lg` (`:782`); the two differ on Adwaita. The fill and the edge are
/// the dialog's own too, `dialog.background_color` and `dialog.border.color`,
/// over the window's `background` and `border` tokens upstream paints the
/// dialog with before the refinement (`:779`, `:781`).
///
/// `max_h` does not reach upstream's `Dialog`: it clamps the dialog to what
/// is left of the viewport *after* applying this style -- in
/// 0.9.1 the view's height less twice the `lg` spacing token and the layer's
/// offset (`:667-689`), set by `max_h` at `:792`, after the refinement at
/// `:787`.
/// `min_h` and the paddings still arrive, because `min_h_24()` runs before the
/// refinement. Kept here for an application-drawn dialog and for the day
/// upstream takes a `max_h` prop
/// (`docs/archive/todo_v0.5.9_gpui-kit-0.6.4-spec.md`).
///
/// `min_w` arrives too: upstream sets the dialog's width after the
/// refinement (`w` at `:790`), a property apart, so the dialog keeps
/// upstream's width and is at least `dialog.min_width` wide. It holds in a
/// window narrower than `dialog.min_width` plus twice the `lg` spacing too:
/// upstream's shrink-to-fit width (`:662-687`) stops at `dialog.min_width`,
/// and gpui-base's positioner keeps the dialog's left edge inside the
/// window (gpui-base `src/positioner.rs:314-316`).
///
/// Upstream reads the four padding sides back out of this style
/// (`dialog/dialog.rs:694-706`), starting from 16px on each side, so
/// an unstated side keeps that 16px. It also reuses them as gaps: the gap
/// between the dialog's sections is `max(top, 8px)` (`:786`), and a
/// `DialogContent`'s gap is the bottom padding (`:817`). A platform that
/// states 32 top and 24 bottom, as GNOME does (docs/platform-facts.md §2.22),
/// therefore also spaces the sections by 32 and the content by 24.
#[must_use]
pub fn dialog(n: Native<'_>) -> StyleRefinement {
    let d = &n.resolved.dialog;
    with_padding(StyleRefinement::default(), &d.border.padding)
        .min_w(px(d.min_width))
        .min_h(px(d.min_height))
        .max_h(px(d.max_height))
        .rounded(px(d.border.corner_radius))
        .bg(rgba_to_hsla(d.background_color))
        .border_color(rgba_to_hsla(d.border.color))
}

/// For `Dialog::entrance` and `AlertDialog::entrance`
/// (`src/dialog/dialog.rs:549`, `src/dialog/alert_dialog.rs:284`): the
/// entrance closest to `dialog.entrance` that gpui-component draws, or `None`
/// where the theme states none, so the dialog keeps upstream's default,
/// `DialogEntrance::SlideDown`.
///
/// Both styles are approximations, and both become `DialogEntrance::Fade`, a
/// fade in place as the dialog opens and out as it closes, over the motion tokens'
/// `duration_normal` both ways (`src/dialog/dialog.rs:558-568`, `:570-580`):
///
/// - `ScaleFade` (Windows, GNOME, Material) loses its scale: gpui-component
///   has no scaling entrance (`src/dialog/entrance.rs:14-27`).
/// - `FadeRise` (KDE) loses its rise: gpui-component has no rising entrance,
///   and its `FadeSlide` settles from just above the final position, the
///   opposite direction.
///
/// Reduced motion skips every entrance and every exit upstream (`:558-562`).
#[must_use]
pub fn dialog_entrance(n: Native<'_>) -> Option<DialogEntrance> {
    n.resolved.dialog.entrance.map(|style| match style {
        DialogEntranceStyle::ScaleFade | DialogEntranceStyle::FadeRise => DialogEntrance::Fade,
    })
}

/// `InputGroupButton` (`src/input/group.rs:602-605` → `:608`): the platform's
/// button radius, and nothing else.
///
/// gpui-component scales a control's radius with its size — an in-group
/// button is `XSmall` and gets `radius / 2`, `Button` does the same for its
/// small and large roundings (`src/button/button.rs:661-663`) — where
/// native-theme records one radius per widget, whatever its size. [`button`]
/// already restores it for a `Button`; this does the same for the button
/// nested in a field, whose height, width and padding the group sets so that
/// it fits, and which [`button`]'s own height would push out of the field.
#[must_use]
pub fn input_group_button(n: Native<'_>) -> StyleRefinement {
    input_group_button_from(n.resolved.button.border.corner_radius)
}

/// [`input_group_button`] of the button radius `radius`.
pub(crate) fn input_group_button_from(radius: f32) -> StyleRefinement {
    StyleRefinement::default().rounded(px(radius))
}

/// `DialogFooter` (`src/dialog/footer.rs:52` → `:56`).
#[must_use]
pub fn dialog_footer(n: Native<'_>) -> StyleRefinement {
    StyleRefinement::default().gap(px(n.resolved.dialog.button_gap))
}

/// `DialogTitle` (`src/dialog/title.rs:42-43` → `:45`): the platform's line
/// height in place of upstream's `relative(1.25)` (`:44`).
///
/// The colour is carried because upstream sets none on the title
/// (`:39-46`), so it takes the dialog's text colour and the platform's
/// `dialog.title_font.color` displaces nothing. Today every preset's title
/// colour is the window's text colour, so this changes no pixel; it is what
/// honours a source that states a title colour of its own.
#[must_use]
pub fn dialog_title(n: Native<'_>) -> StyleRefinement {
    with_coloured_text(StyleRefinement::default(), &n.resolved.dialog.title_font, n)
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

/// The frame and the fill of a list view: `list.border`'s line width, colour
/// and radius, a clip to that radius, and `list.background_color`.
///
/// Neither `List` (`src/list/list.rs`, `RenderOnce for List<D>`) nor `Tree`
/// (`src/tree.rs`, `RenderOnce for Tree`) paints a border or a fill of its
/// own: each refines a plain `div()` and leaves both to the application, where
/// `DataTable` draws one from `Theme::radius` and `Theme::border` when it is
/// `bordered` (`src/table/data_table.rs:248-252`). Apply this to a `List` or a
/// `Tree` — both are `Styled` and the refinement lands on that outer `div` —
/// or to the box an application draws around one, and the three agree. There
/// is no tree theme in the model: a tree is a list view, and reads
/// `resolved.list`.
///
/// The fill is this builder's because nothing upstream paints one: a
/// `ListItem` fills only a hovered and a selected row
/// (`src/list/list_item.rs:225-233`, `:257-264`), and `ThemeColor::list`,
/// which the connector fills with the same colour, has no reader outside the
/// theme schema's fallbacks (`src/theme/schema.rs:985-986`, `:1020`).
///
/// The clip keeps the rows inside the frame; how it meets the rounded corners
/// depends on the core. GPUI Fast clips the children of a rounded box that
/// clips both axes to its curved padding edge (gpui-fast 0.1.15
/// `src/fast/rounded_clip.rs:68-104`), so a row's square fill is cut at the
/// curve. gpui-pre clips them to the frame's rectangle (gpui-pre 0.3.12
/// `src/window.rs:2453-2461`, `ContentMask`: "only rectangular content masks
/// are supported"), so a square fill at a corner shows past the curve there.
/// The application rounds a fill that meets the frame by its inner radius,
/// `corner_radius` less `line_width`: [`list_item`] sets no radius.
#[must_use]
pub fn list(n: Native<'_>) -> StyleRefinement {
    let l = &n.resolved.list;
    let b = &l.border;
    list_from(
        b.line_width,
        rgba_to_hsla(b.color),
        b.corner_radius,
        rgba_to_hsla(l.background_color),
    )
}

/// [`list`] of an edge `border_width` wide in `border_color`, corners of
/// `radius` and the fill `background`.
pub(crate) fn list_from(
    border_width: f32,
    border_color: Hsla,
    radius: f32,
    background: Hsla,
) -> StyleRefinement {
    StyleRefinement::default()
        .border(px(border_width))
        .border_color(border_color)
        .rounded(px(radius))
        .bg(background)
        .overflow_hidden()
}

/// Declarative `Table` (`src/table/table.rs:111` → `:114`); the rows are inner
/// elements, out of the caller's reach, which only upstream can change.
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
/// content surface (`:172-177`), which 0.9.1 nests in a column that also
/// holds the optional footer (`:167-168`).
///
/// The fill is `card.background_color` on every variant: the refinement
/// comes last (`:177`), over the Fill variant's `group_box` token (`:145`),
/// and on a Normal or Outline box, which paints none of its own (`:144`,
/// `:146`).
#[must_use]
pub fn group_box_content(n: Native<'_>) -> StyleRefinement {
    let c = &n.resolved.card;
    with_padding(StyleRefinement::default(), &c.border.padding)
        .rounded(px(c.border.corner_radius))
        .border(px(c.border.line_width))
        .border_color(rgba_to_hsla(c.border.color))
        .bg(rgba_to_hsla(c.background_color))
}

/// For `AccordionItem::title_style` (`src/accordion.rs:220`, applied `:301-307`).
///
/// The title's colour is carried because upstream sets `foreground` on an
/// open item's title row one line before the refinement (`:306` → `:307`)
/// and none on a closed one's, and draws no disabled colour of its own:
/// `expander.font.color` displaces `foreground` there and nothing here. Today
/// every preset's expander text is the window's text colour, so this changes
/// no pixel; it is what honours a source that states a colour of its own.
#[must_use]
pub fn accordion_title(n: Native<'_>) -> StyleRefinement {
    let e = &n.resolved.expander;
    StyleRefinement::default()
        .h(px(e.header_height))
        .text_color(rgba_to_hsla(e.font.color))
}

/// The edge of an `Accordion` and of each of its `AccordionItem`s:
/// `expander.border.color`, which each takes through `Styled`.
///
/// A bordered `Accordion` (upstream's default, `src/accordion.rs:37`) draws
/// one rounded card around its items, and each item but the last a line
/// under it, both in the `border` token and before the caller's refinement
/// (`:108-113` → `:114` on the Accordion, `:380-382` → `:384` on the item),
/// so the platform's colour wins on both. Only the colour: where no edge is
/// drawn -- an unbordered Accordion, the last item -- it colours nothing.
/// Today every preset's expander edge is the window's border colour, so this
/// changes no pixel; it is what honours a source that states one of its own.
#[must_use]
pub fn accordion(n: Native<'_>) -> StyleRefinement {
    StyleRefinement::default().border_color(rgba_to_hsla(n.resolved.expander.border.color))
}

/// For `AccordionItem::hover` (`src/accordion.rs:230-233`): the title row's
/// fill under the pointer, `expander.hover_background`, which the item lays
/// over the row there (`:322-324`), or nothing where the theme states none,
/// so the row stays as it is at rest. gpui-component draws no hover of its
/// own (`:227-229`), and none on a disabled item (`:322`). Pass it as
/// `item.hover(|_| geometry::accordion_hover(n))`.
#[must_use]
pub fn accordion_hover(n: Native<'_>) -> StyleRefinement {
    match n.resolved.expander.hover_background {
        Some(fill) => StyleRefinement::default().bg(rgba_to_hsla(fill)),
        None => StyleRefinement::default(),
    }
}

/// The metrics `Checkbox` and `Radio` share; platform-facts §2.5 defines radio
/// metrics as the checkbox's with a circular indicator, so reading the
/// checkbox's fields for a radio is what the platform states rather than one
/// widget's value standing in for another's. Only the text colour separates
/// the two.
fn checkbox_metrics(n: Native<'_>) -> StyleRefinement {
    label_gap(n.resolved.checkbox.label_gap)
}

/// [`checkbox_metrics`] of the gap `gap` between the indicator and the label.
pub(crate) fn label_gap(gap: f32) -> StyleRefinement {
    StyleRefinement::default().gap(px(gap))
}

/// `Checkbox` (`src/checkbox.rs:281-293` → `:294`); the indicator is an inner
/// element, out of the caller's reach: upstream work (ROADMAP.md, *Upstream
/// receivers for the gpui connector*). The text size replaces upstream's
/// `input_text_size` (`:286-289`: XSmall `text_xs`, Small and Medium
/// `text_sm`, Large `text_base`).
///
/// No colour, although upstream does label the row with `foreground` (`:285`)
/// before this refinement: a `Checkbox::label` is wrapped in a div that sets
/// `foreground` itself (`:342-347`), so a carried colour would never reach it,
/// and the disabled hook applies `muted_foreground` and only *then* the
/// refinement (`:263-267`), so a carried colour would displace the disabled
/// colour of custom children. [`radio`], whose label child sets no colour of
/// its own, carries it.
///
/// The platform's line height lands on the row, over upstream's
/// `relative(1.)` (`:284`); the label column sets its own `relative(1.25)`
/// (`:337`), which stands, and on which upstream centres the indicator
/// (`:301-302`).
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
/// trigger sets before the refinement (`select.rs:578` → `:580`,
/// `combobox.rs:1014` → `:1016`).
///
/// The stated padding sides reach the trigger: upstream pads it
/// (`input_size`, `select.rs:578`, `combobox.rs:1014`) before the refinement
/// (`select.rs:580`, `combobox.rs:1016`), so the refinement wins, and no inner
/// element pads again. The right side is the exception where the theme
/// states `combo_box.arrow_area_width`: the platform then measures its right
/// side to a separate arrow column (WinUI's 38px column, Breeze's 20px one;
/// docs/platform-facts.md §2.24), and gpui's trigger has none — its caret
/// sits inside the padded row (`select.rs:626-627` in the row at
/// `:591-633`, `combobox.rs:673-674` in the row at `:1027-1046`). A right
/// side measured to a column the trigger does not have is not applied, and
/// upstream's own right padding (`input_px`, `select.rs:578`,
/// `combobox.rs:1014`) stands. Without an arrow column the stated right side
/// is measured to the text, and is applied like the others.
///
/// The edge is `combo_box.border.line_width` wide, over upstream's `border_1`
/// (`select.rs:569`, `combobox.rs:1005`): a platform whose trigger has no line
/// at rest (adwaita's 0, `docs/platform-facts.md` §2.24) draws none.
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
        .border(px(c.border.line_width))
}

/// `Select` (`src/select.rs:569-579` → `:580`); the arrow is an inner element,
/// out of the caller's reach: upstream work (ROADMAP.md, *Upstream receivers
/// for the gpui connector*).
/// Height by the control-height rule (module doc), through `h`, in place of
/// upstream's `h_8` at a text-scaling factor of 1 or less.
///
/// The stated padding sides reach the trigger: upstream pads it
/// (`input_size`, `select.rs:578`) before the refinement (`:580`), so the
/// refinement wins, and no inner element pads again. Where the theme states
/// `combo_box.arrow_area_width`, the right side is not applied and
/// upstream's own stands: the platform measures it to an arrow column, and
/// the caret sits inside the padded trigger (`select.rs:626-627`, in the row
/// at `:591-633`), which has none (see the shared metrics, and
/// docs/platform-facts.md §2.24).
///
/// The colour is carried for the same reason as [`list_item`]: upstream labels
/// the trigger with `foreground` through `input_style`
/// (`src/input/input.rs:126`, applied at `select.rs:573`) before applying this
/// refinement. It is safe here although the same helper's disabled branch
/// returns `muted_foreground` before the refinement too
/// (`src/input/input.rs:120-124`), because `Select` re-mutes its title *child*
/// when disabled (`select.rs:511-513`) and a child wins over the trigger.
#[must_use]
pub fn select(n: Native<'_>) -> StyleRefinement {
    with_coloured_text(combo_box_metrics(n), &n.resolved.combo_box.font, n)
}

/// `Combobox` (`src/combobox.rs:999-1015` → `:1016`): [`select`]'s metrics, and
/// the colour [`select`] can take but this cannot.
///
/// The metrics are the height and the padding sides, and the right side
/// follows the same rule: beside a stated `combo_box.arrow_area_width` it is
/// upstream's own, because the caret sits inside the padded trigger
/// (`combobox.rs:673-674`, in the row at `:1027-1046`).
///
/// Upstream labels the trigger with the same `input_style` `foreground`
/// (`:1009`), but its disabled branch (`src/input/input.rs:120-124`) delivers
/// `muted_foreground` through that same call, before the refinement at `:1016`,
/// and the selected-title child (`:603-609`) sets no colour to re-mute with —
/// so a carried colour would beat the disabled colour instead of yielding to
/// it. A disabled trigger takes the platform's disabled text colour from
/// [`combo_box_fill`] instead.
#[must_use]
pub fn combobox(n: Native<'_>) -> StyleRefinement {
    with_text(combo_box_metrics(n), &n.resolved.combo_box.font, n)
}

/// `TitleBar` (`src/title_bar.rs:339` → `:349`); the height has no theme field.
///
/// Upstream sets no text colour on the bar (`:332-349` paints the fill and the
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

/// A `TitleBar` of a window that is not the active one: the fill
/// `window.inactive_title_bar_background` and the text
/// `window.inactive_title_bar_text_color`. Apply it after [`title_bar`] while
/// gpui's `Window::is_window_active` is false (gpui-pre `src/window.rs:3204`);
/// gpui redraws the window when that changes (`:2181-2195`).
///
/// Colour, carried because neither `TitleBar` nor `ThemeColor` knows of a
/// window's activation: upstream paints every bar the same, a gradient from
/// `title_bar` mixed with `background` down to `title_bar`
/// (`src/title_bar.rs:22-37`, painted at `:345-348` and refined at `:349`).
/// The refinement's fill is that gradient drawn from the inactive fill in
/// place of `title_bar` -- `background` is `window.background_color`, as the
/// connector installs it -- so an inactive bar has the active one's shape,
/// and a theme whose inactive pair is the active one draws the same bar in
/// both states; its text colour replaces [`title_bar`]'s. A separate
/// builder, not a parameter of [`title_bar`], as [`button_disabled`] and
/// [`button_checked`] are for a Button's states: [`title_bar`]'s signature
/// stays, and an active bar takes nothing more.
#[must_use]
pub fn title_bar_inactive(n: Native<'_>) -> StyleRefinement {
    let w = &n.resolved.window;
    StyleRefinement::default()
        .bg(title_bar_gradient(
            rgba_to_hsla(w.inactive_title_bar_background),
            rgba_to_hsla(w.background_color),
        ))
        .text_color(rgba_to_hsla(w.inactive_title_bar_text_color))
}

/// The weight of the bar's own colour in the colour its gradient starts
/// from, the window background's being the rest: upstream's
/// `default_title_bar_background` (`src/title_bar.rs:25-30`), which is
/// private.
const TITLE_BAR_GRADIENT_OWN: f32 = 0.55;

/// Upstream's title-bar fill (`src/title_bar.rs:22-37`) from `bar` and
/// `background`: a top-to-bottom gradient from `bar` mixed with
/// `background` in gamma-encoded RGB, [`TITLE_BAR_GRADIENT_OWN`] to the
/// rest, down to `bar` itself.
fn title_bar_gradient(bar: gpui::Hsla, background: gpui::Hsla) -> gpui::Background {
    let (b, w) = (bar.to_rgb(), background.to_rgb());
    let own = TITLE_BAR_GRADIENT_OWN;
    let rest = 1. - own;
    let start = gpui::Hsla::from(gpui::Rgba {
        r: b.r * own + w.r * rest,
        g: b.g * own + w.g * rest,
        b: b.b * own + w.b * rest,
        a: b.a * own + w.a * rest,
    });
    gpui::linear_gradient(
        180.,
        gpui::linear_color_stop(start, 0.),
        gpui::linear_color_stop(bar, 1.),
    )
}

/// A toolbar row (docs/platform-facts.md §2.13), the application's own or
/// gpui-component's `Toolbar`: its height the content's, at least
/// `toolbar.bar_height` where the platform states one -- KDE's and GNOME's
/// toolbars size to their content and state none (platform-facts.md:1390),
/// and a fixed height would be an invention there, so the refinement sets
/// the height to `auto`, and replaces the `min_h_8` gpui-component's
/// `Toolbar` sets at its Small size (toolbar.rs:253-255) with
/// `toolbar.bar_height` where the theme states one and with `auto`, the
/// content's height, where it states none; an application that wants a
/// fixed height or a floor sets one after the refinement --,
/// `toolbar.item_gap` between items where the theme states one, the
/// `toolbar.border` padding sides the platform states,
/// `toolbar.background_color`, and `toolbar.font`. An unstated side or gap
/// is left to the application.
///
/// The edge is the bar's bottom rule, `toolbar.border.color` at
/// `toolbar.border.line_width`, inside the bar, between it and the content
/// under it, as the theme states it: a width of 0 draws none. Neither
/// gpui-component's `Toolbar` nor gpui-base's draws an edge of its own
/// (`toolbar.rs:243-257`). Bottom only, not a frame, for parity:
/// docs/platform-facts.md §2.13 has no border row, and the other two
/// connectors draw the toolbar's border as one line under the bar -- egui's
/// toolbar role strokes it as the top panel's separator
/// (native-theme-egui `src/style/roles.rs:128-141`), and the iced connector's
/// `styles::toolbar_rule` is a `toolbar.border.line_width` rule under the
/// bar (native-theme-iced `src/styles.rs`, `toolbar_rule`).
///
/// The text colour displaces nothing: neither gpui-component's `Toolbar`
/// nor gpui-base's sets a colour on the row (`toolbar.rs:243-257`). Today
/// every preset's toolbar text is the window's text colour, so this changes
/// no pixel; it is what honours a source that states one of its own.
#[must_use]
pub fn toolbar(n: Native<'_>) -> StyleRefinement {
    let t = &n.resolved.toolbar;
    let r = with_coloured_text(StyleRefinement::default(), &t.font, n).h_auto();
    let r = match t.bar_height {
        Some(h) => r.min_h(px(h)),
        None => r.min_h_auto(),
    };
    let r = match t.item_gap {
        Some(g) => r.gap(px(g)),
        None => r,
    };
    with_padding(r, &t.border.padding)
        .bg(rgba_to_hsla(t.background_color))
        .border_b(px(t.border.line_width))
        .border_color(rgba_to_hsla(t.border.color))
}

// --- Size helpers (spec §9.3) -------------------------------------------------

/// `Spinner::with_size` (`src/spinner.rs:53-65` → `Icon`, `src/icon.rs:226`;
/// 0.6.4 merged the two sizing arms 0.6.0 had at `:166` and `:195`).
#[must_use]
pub fn spinner_size(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.spinner.diameter))
}

/// `DataTable::with_size`: `list.row_height` as `Size::Size`, or upstream's own
/// Medium row height where the theme states none. Above a text-scaling factor
/// of 1 the row is at least the scaled `list.item_font`'s line box at
/// `defaults.line_height`, rounded up, upstream's own top and bottom cell
/// padding (`Size::table_cell_padding`, `src/sizing.rs:89-94`) and the row's
/// 1 px bottom border (`border_b_1`, `src/table/state.rs:2375`): the
/// control-height rule (module doc), computed here because upstream gives
/// every row this height (`src/table/state.rs:2344`, `:2373`), its border
/// inside it, and clips each cell to it (`:1567-1573`).
///
/// Under `Size::Size` gpui-component sets the row height verbatim
/// (`src/sizing.rs:57-64`) and the cells no text size (`:323-333`), so they
/// inherit — refine the table's container with [`table`] and they take
/// `list.item_font`. The Medium cell padding stands (`:89-94`). At
/// `Size::Medium` the cells are `text_sm`: 0.875 of the platform
/// font.
#[must_use]
pub fn data_table_size(n: Native<'_>) -> Size {
    let l = &n.resolved.list;
    data_table_size_from(
        l.row_height,
        scaled_text_size(&l.item_font, n),
        n.resolved.defaults.line_height,
        n.accessibility.text_scale_factor(),
    )
}

/// [`data_table_size`] of the stated row height `row_height`, the scaled
/// size `font_size` of `list.item_font`, the line height `line_height` and
/// the text-scaling factor `text_scale`.
pub(crate) fn data_table_size_from(
    row_height: Option<f32>,
    font_size: f32,
    line_height: f32,
    text_scale: f32,
) -> Size {
    let stated = row_height.map_or_else(|| Size::Medium.table_row_height(), px);
    Size::Size(row_height_fit(stated, font_size, line_height, text_scale))
}

/// For `DataTable::header_row_height` (`src/table/data_table.rs:151`):
/// `list.header_height`, grown above a text-scaling factor of 1 as
/// [`data_table_size`] grows a row, or `None` where the theme states none, so
/// the header keeps the body rows' height, which upstream gives an unset
/// header (`src/table/state.rs:840-844`) and [`data_table_size`] sets.
///
/// It grows with `list.item_font`, the font the header's text is drawn in: a
/// `Size::Size` header cell sets no text size (`src/table/state.rs:1940`
/// through `render_cell`, `src/sizing.rs:323-333`), so it inherits the
/// container's, which [`table`] sets; `list.header_font` reaches only the
/// header's text colour (`ThemeColor::table_head_foreground`).
#[must_use]
pub fn list_header_height(n: Native<'_>) -> Option<Pixels> {
    let l = &n.resolved.list;
    list_header_height_from(
        l.header_height,
        scaled_text_size(&l.item_font, n),
        n.resolved.defaults.line_height,
        n.accessibility.text_scale_factor(),
    )
}

/// [`list_header_height`] of the stated header height `header_height`, with
/// [`data_table_size_from`]'s other inputs.
pub(crate) fn list_header_height_from(
    header_height: Option<f32>,
    font_size: f32,
    line_height: f32,
    text_scale: f32,
) -> Option<Pixels> {
    header_height.map(|h| row_height_fit(px(h), font_size, line_height, text_scale))
}

/// The bottom border upstream draws under a `DataTable` row and under its
/// header row, inside the row's height: `border_b_1()` (`src/table/state.rs:2375`,
/// the header's `:2187-2188`), 1 px (gpui-pre-macros 0.3.12 `src/styles.rs:1336-1340`).
const TABLE_ROW_BORDER: f32 = 1.0;

/// A `DataTable` row's height by the control-height rule (module doc): at a
/// text-scaling factor of 1 or less `stated`, verbatim; above 1 at least the
/// line box of the scaled font size `font_size` at the line height
/// `line_height` (`defaults.line_height`), rounded up, upstream's own top and
/// bottom cell padding for `Size::Size` and the row's bottom border
/// ([`TABLE_ROW_BORDER`]). A `Size` is a number, so the growth the
/// single-line controls get from an automatic height is computed here.
fn row_height_fit(stated: Pixels, font_size: f32, line_height: f32, text_scale: f32) -> Pixels {
    if text_scale <= 1.0 {
        return stated;
    }
    let padding = Size::Size(stated).table_cell_padding();
    let line = (font_size * line_height).ceil();
    let fit = line + f32::from(padding.top) + f32::from(padding.bottom) + TABLE_ROW_BORDER;
    px(f32::from(stated).max(fit))
}

/// [`row_height_fit`] of a 0.7.2 font spec under `n`, as the tests state it.
#[cfg(test)]
fn table_row_height(stated: Pixels, font: &ResolvedFontSpec, n: Native<'_>) -> Pixels {
    row_height_fit(
        stated,
        scaled_text_size(font, n),
        n.resolved.defaults.line_height,
        n.accessibility.text_scale_factor(),
    )
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
    icon_size(n.resolved.defaults.icon_sizes.small)
}

/// `Icon::with_size` for large icons (`defaults.icon_sizes.large`).
#[must_use]
pub fn icon_size_large(n: Native<'_>) -> Size {
    icon_size(n.resolved.defaults.icon_sizes.large)
}

/// `Icon::with_size` for dialog icons (`defaults.icon_sizes.dialog`).
#[must_use]
pub fn icon_size_dialog(n: Native<'_>) -> Size {
    icon_size(n.resolved.defaults.icon_sizes.dialog)
}

/// `Icon::with_size` for panel icons (`defaults.icon_sizes.panel`).
#[must_use]
pub fn icon_size_panel(n: Native<'_>) -> Size {
    icon_size(n.resolved.defaults.icon_sizes.panel)
}

/// The `Size` of an icon `size` pixels square.
pub(crate) fn icon_size(size: f32) -> Size {
    Size::Size(px(size))
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
/// `src/scrollbar.rs:1427-1451`). Where `scrollbar.overlay_mode` is false —
/// every bundled preset but adwaita, ios-13, macos-monterey and macos-tahoe:
/// kde-breeze, windows-11, material-3 and the community themes — [`crate::apply`]
/// also asks for an always-visible bar, so a content element that fills the
/// scroll area runs underneath it. Where the theme states no track as well (a
/// transparent `scrollbar.track_color`: kde-breeze, windows-11), that content
/// shows through the bar, a row's hover fill and text or cells that reach the
/// edge among it; the gutter keeps it clear of the bar. The width
/// is read back from [`crate::base_layer::scrollbar_geometry`], the one that
/// was written onto gpui-base, so the two cannot drift apart.
///
/// Apply it to the element the container scrolls, not around the container:
/// `overflow_y_scrollbar` keeps the caller's own element as the scroll content
/// (`src/scroll/scrollable.rs`, `Scrollable::render`), so padding on it is
/// padding inside the scrolled box and the bar lands beside it.
#[must_use]
pub fn scrollbar_gutter(n: Native<'_>) -> StyleRefinement {
    scrollbar_gutter_from(
        n.resolved.scrollbar.overlay_mode,
        crate::base_layer::scrollbar_geometry(n.resolved).track_width,
    )
}

/// [`scrollbar_gutter`] of scrollbars that are overlays where `overlay`, in
/// a track `track_width` wide.
pub(crate) fn scrollbar_gutter_from(overlay: bool, track_width: Pixels) -> StyleRefinement {
    if overlay {
        return StyleRefinement::default();
    }
    StyleRefinement::default().pr(track_width)
}

// --- Builder helpers (spec §9.3) ----------------------------------------------

/// For `Dialog::max_w` (`src/dialog/dialog.rs:519`).
#[must_use]
pub fn dialog_max_width(n: Native<'_>) -> Pixels {
    px(n.resolved.dialog.max_width)
}

/// An `Input`'s height rule alone: the line height and height [`input`]
/// sets, and nothing else of its refinement.
///
/// A single-line `Input` takes it through `StyledExt::refine_style`, the
/// caller's style, which `Input` refines its root with last
/// (`src/input/input.rs:808`). Above a text-scaling factor of 1 the rule's
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

/// The fill of an `Input` root (`src/input/input.rs:715-726`, painted at
/// `:800`, refined at `:808`), for a field built with `disabled`: the
/// platform's `input.background_color`, or disabled its
/// `input.disabled_background` and its text in `input.disabled_text_color`.
/// Apply it after [`input`].
///
/// Colour, not geometry, and carried for the reason the builders that carry a
/// text colour carry it (module doc): no `ThemeColor` field maps to it. Upstream fills an enabled
/// field with `Theme::input_background`, which is the window's `background`
/// in a light theme and `input` mixed with transparent in a dark one
/// (`theme/mod.rs:493-499`), and a disabled one with `input` mixed with
/// transparent at half opacity (`input/input.rs:119-122`, `:722-726`); the
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
/// `input/base/element.rs:3184-3185`), and the field's
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

/// The fill of a `Select` or `Combobox` trigger (`src/select.rs:572`,
/// `src/combobox.rs:1008`, refined at `select.rs:580` and `combobox.rs:1016`),
/// for a trigger built with `disabled`: `combo_box.background_color`, or
/// disabled `combo_box.disabled_background` (the enabled fill where the theme
/// states none) and the text in `combo_box.disabled_text_color`, faded by
/// `combo_box.disabled_opacity`. Apply it after [`select`] or [`combobox`].
///
/// Colour, carried for the reason [`input_fill`] is: upstream fills an
/// enabled trigger with `Theme::input_background` and a disabled one with the
/// `input` token mixed toward transparent (`input_style`,
/// `src/input/input.rs:118-128`), and no `ThemeColor` field maps to the
/// platform's combo box fill. Disabled, the fade replaces upstream's
/// `opacity(0.5)` on the trigger (`select.rs:574`, `combobox.rs:1010`), both
/// set before the refinement: a platform dims by its disabled colours or by
/// that opacity, and its data makes the other an identity
/// (docs/platform-facts.md §2.1.6).
///
/// The disabled text colour reaches a `Combobox`'s selected text: its
/// trigger takes `muted_foreground` from `input_style` before the refinement
/// (`input/input.rs:120-124` → `combobox.rs:1009` → `:1016`), and the title
/// elements set none (`combobox.rs:588-609`). It does not reach a `Select`'s
/// title, which re-mutes itself when disabled (`select.rs:511-512`), nor the
/// placeholder or the caret of either, which are `muted_foreground` on
/// elements of their own (`select.rs:479`, `:624-627`; `combobox.rs:575`,
/// `:670-674`).
#[must_use]
pub fn combo_box_fill(n: Native<'_>, disabled: bool) -> StyleRefinement {
    let c = &n.resolved.combo_box;
    if disabled {
        StyleRefinement::default()
            .bg(rgba_to_hsla(
                c.disabled_background.unwrap_or(c.background_color),
            ))
            .text_color(rgba_to_hsla(c.disabled_text_color))
            .opacity(unit(c.disabled_opacity))
    } else {
        StyleRefinement::default().bg(rgba_to_hsla(c.background_color))
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
/// (`src/button/button.rs:1358-1404`, the label at `:1369`), literals of its
/// own. Its disabled
/// style replays the caller's refinement over its own (`:830-836`), so the
/// refinement's colours win there -- and at rest too (`:764`), which is why
/// an enabled Button must not take it.
#[must_use]
pub fn button_disabled(n: Native<'_>) -> StyleRefinement {
    let b = &n.resolved.button;
    button_disabled_from(
        b.disabled_background.map(rgba_to_hsla),
        rgba_to_hsla(b.background_color),
        rgba_to_hsla(b.disabled_text_color),
        b.disabled_opacity,
    )
}

/// [`button_disabled`] of the disabled fill `fill` (`None` where the theme
/// states none), the resting fill `rest`, the label colour `text` and the
/// opacity `opacity`.
pub(crate) fn button_disabled_from(
    fill: Option<Hsla>,
    rest: Hsla,
    text: Hsla,
    opacity: f32,
) -> StyleRefinement {
    StyleRefinement::default()
        .bg(fill.unwrap_or(rest))
        .text_color(text)
        .opacity(unit(opacity))
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
    button_checked_from(
        b.checked_text_color.map(rgba_to_hsla),
        rgba_to_hsla(b.active_text_color),
        b.checked_background.map(rgba_to_hsla),
        b.active_background.map(rgba_to_hsla),
    )
}

/// [`button_checked`] of the checked label colour and fill, each `None`
/// where the theme states none, and the pressed ones, which stand in for
/// them there.
pub(crate) fn button_checked_from(
    checked_text: Option<Hsla>,
    pressed_text: Hsla,
    checked_fill: Option<Hsla>,
    pressed_fill: Option<Hsla>,
) -> StyleRefinement {
    let r = StyleRefinement::default().text_color(checked_text.unwrap_or(pressed_text));
    match checked_fill.or(pressed_fill) {
        Some(fill) => r.bg(fill),
        None => r,
    }
}

/// The rest state of a toolbar `Button` built on
/// [`variants::toolbar_button`](crate::variants::toolbar_button): where the
/// theme states `toolbar.button_hover_border_color`, a transparent fill, a
/// transparent border `button.border.line_width` wide, as [`button`] sets the
/// width, and each side of `button.border.padding` the theme states less that
/// width, floored at 0; nothing where it states no outline (the variant's
/// colour then stays transparent, as `ghost_button`'s).
///
/// Where the theme states the outline, the variant's colour is the edge
/// colour, which upstream draws as the border in every state and mixes
/// toward transparent into the fill at rest (gpui-component 0.10.0
/// `src/button/button.rs:1044`), with no border width for a custom variant
/// (`:727-732`): this hides both at rest and gives the edge its width. gpui
/// lays the border out, so the padding is reduced by it and the button keeps
/// its size. It assumes the caller pads the button with
/// `button.border.padding`, as [`button`] and the showcase's tool buttons do:
/// on a button padded otherwise it replaces the padding of each stated side
/// with that side less the width. A side the theme leaves unstated is not
/// set, and the border makes the button that much wider there. Upstream
/// replays the refinement over the selected and disabled states too
/// (`:836-850`). Apply it before [`toolbar_button_checked`].
#[must_use]
pub fn toolbar_button_rest(n: Native<'_>) -> StyleRefinement {
    if n.resolved.toolbar.button_hover_border_color.is_none() {
        return StyleRefinement::default();
    }
    let b = &n.resolved.button;
    let w = b.border.line_width;
    let p = &b.border.padding;
    let less = |side: Option<f32>| side.map(|v| (v - w).max(0.0));
    let inside = ResolvedPadding {
        top: less(p.top),
        right: less(p.right),
        bottom: less(p.bottom),
        left: less(p.left),
    };
    with_padding(StyleRefinement::default(), &inside)
        .bg(Hsla::transparent_black())
        .border(px(w))
        .border_color(Hsla::transparent_black())
}

/// The fill, text colour and pen of a toolbar Button that is on
/// (`Button::selected(true)`), over [`variants::toolbar_button`](crate::variants::toolbar_button):
/// `toolbar.button_checked_background`, lettered `button.checked_text_color`
/// (`button.active_text_color` where that is `None`); with no checked fill the
/// pressed one, `toolbar.button_active_background`, else `button.active_background`,
/// as [`button_checked`] falls back, the text the custom variant's one foreground;
/// with none of the three no fill (the variant's selected one, or none after
/// [`toolbar_button_rest`], whose transparent fill upstream replays). The pen is
/// `toolbar.button_checked_border_color` where stated, at [`toolbar_button_rest`]'s
/// width. A selected Button takes no hover or press style (gpui-component 0.10.0
/// `src/button/button.rs:733`, `:758`), so the pen stays under the pointer and
/// pressed. Apply it after [`toolbar_button_rest`], to a selected toolbar Button
/// only: upstream replays the refinement at rest too.
#[must_use]
pub fn toolbar_button_checked(n: Native<'_>) -> StyleRefinement {
    let t = &n.resolved.toolbar;
    let b = &n.resolved.button;
    let r = match t.button_checked_background {
        Some(_) => StyleRefinement::default().text_color(rgba_to_hsla(
            b.checked_text_color.unwrap_or(b.active_text_color),
        )),
        None => StyleRefinement::default(),
    };
    match t
        .button_checked_background
        .or(t.button_active_background)
        .or(b.active_background)
    {
        Some(fill) => checked_pen(r.bg(rgba_to_hsla(fill)), t),
        None => checked_pen(r, t),
    }
}

/// The rest look of a dialog's default Button where the theme gives it one
/// of its own (`button.default_button_background`, KDE): that fill, and a
/// border in `button.default_button_border_color`, else `button.border.color`
/// (docs/platform-facts.md §2.3). `None` where the theme states no fill: the
/// default button is then the primary one, `.primary()`. Apply it after
/// [`button`], which sets the border's width, over
/// [`variants::default_button`](crate::variants::default_button): upstream applies the refinement at rest,
/// under the variant's hover and press styles. An enabled Button only: a
/// disabled one takes [`button_disabled`] instead, since upstream replays a
/// refinement in the disabled state too and [`button_disabled`] sets no
/// border colour, so this frame would stand there.
#[must_use]
pub fn default_button_rest(n: Native<'_>) -> Option<StyleRefinement> {
    let b = &n.resolved.button;
    default_button_rest_from(
        b.default_button_background.map(rgba_to_hsla),
        b.default_button_border_color.map(rgba_to_hsla),
        rgba_to_hsla(b.border.color),
    )
}

/// [`default_button_rest`] of the default button's fill and frame, each
/// `None` where the theme states none, and the ordinary button's frame
/// `border`, which stands in for the default one there.
pub(crate) fn default_button_rest_from(
    fill: Option<Hsla>,
    frame: Option<Hsla>,
    border: Hsla,
) -> Option<StyleRefinement> {
    let fill = fill?;
    Some(
        StyleRefinement::default()
            .bg(fill)
            .border_color(frame.unwrap_or(border)),
    )
}

/// A row's height for [`menu_item`]: `stated`, a fixed height, by the
/// control-height rule ([`with_height_rule`]); where it is `None`, at least
/// `least`, a least height the content grows past (`min_h` is the border
/// box); nothing where both are `None`. A stated `menu.row_height` wins over
/// `menu.min_row_height` (docs/platform-facts.md §2.6).
fn row_height_rule(stated: Option<f32>, least: Option<f32>, n: Native<'_>) -> StyleRefinement {
    match (stated, least) {
        (None, Some(least)) => StyleRefinement::default().min_h(px(least)),
        _ => with_height_rule(StyleRefinement::default(), stated, n),
    }
}

/// `r` with a checked toolbar Button's pen ([`toolbar_button_checked`]): its
/// border colour `toolbar.button_checked_border_color` where the theme states
/// it, drawn at the width [`toolbar_button_rest`] gives the border; `r` as it
/// is where the theme states none.
fn checked_pen(
    r: StyleRefinement,
    t: &native_theme::theme::ResolvedToolbarTheme,
) -> StyleRefinement {
    match t.button_checked_border_color {
        Some(pen) => r.border_color(rgba_to_hsla(pen)),
        None => r,
    }
}

/// The text colour of a selected `ListItem` (`ListItem::selected(true)`):
/// `list.selection_text_color`. Apply it after [`list_item`], to a selected
/// row only.
///
/// Colour, not geometry, carried because no `ThemeColor` field reaches it:
/// upstream marks a selected row by its fill alone, `list_active` (or
/// `accent`), painted after the refinement (`src/list/list_item.rs:257-264`),
/// which the `list_active` row gives `list.selection_background`, and leaves
/// its text the `foreground` it set before the refinement (`:205` → `:209`)
/// -- or [`list_item`]'s `list.item_font.color` -- which a platform that
/// letters its selected rows in a colour of their own (Breeze's white on its
/// accent) does not. A row that is not selectable is muted after the
/// refinement (`:235-237`), so its colour still wins.
///
/// The fill is `list_active` only while gpui-component's
/// `Theme::list.active_highlight` is true (gpui-component 0.9.1
/// `list_item.rs:258-259`), its default (gpui-base 0.9.1 `list_settings.rs:15`),
/// which this connector leaves alone. An application that turns it off gets
/// `accent`, the menu hover fill (`menu.hover_background`), lettered
/// `accent_foreground` (`menu.hover_text_color`), both set by upstream after the
/// refinement (0.9.1 `:260-262`), so this builder's colour does not reach that
/// row. Apply it only while `active_highlight` is on.
#[must_use]
pub fn list_item_selected(n: Native<'_>) -> StyleRefinement {
    list_item_selected_from(rgba_to_hsla(n.resolved.list.selection_text_color))
}

/// [`list_item_selected`] of the selected row's text colour `text`.
pub(crate) fn list_item_selected_from(text: Hsla) -> StyleRefinement {
    StyleRefinement::default().text_color(text)
}

/// `Link` (`src/link.rs:70-90`): `link.underline_enabled`, and the link's
/// font: its family, size, weight and style.
///
/// Upstream underlines a link at rest (`text_decoration_1`, `link.rs:77-78`)
/// before the caller's refinement (`:90`). Where the platform draws its links
/// without an underline (`link.underline_enabled` false), the refinement sets
/// the underline's thickness to zero, so none is painted; where it underlines
/// them, upstream's underline stands. The underline's colour, and the one
/// upstream adds on hover and press (`:79-88`), are upstream's: the model
/// states neither. The text colour is `ThemeColor::link`, the
/// `link.font.color` row. The fill is `link.background_color`: upstream paints
/// none (`link.rs:73-90`), so it displaces nothing, and every preset states a
/// transparent one, as inline links have none (`docs/inheritance-rules.toml:405`).
#[must_use]
pub fn link(n: Native<'_>) -> StyleRefinement {
    let l = &n.resolved.link;
    let r = with_text(StyleRefinement::default(), &l.font, n).bg(rgba_to_hsla(l.background_color));
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
// behind it (gpui-component `theme/mod.rs:596-598`), so its readers get that
// default whatever a theme says: a `Dialog`'s viewport margin (gpui-component
// `dialog/dialog.rs:667`), a `GridLayout`'s gap (gpui-component
// `grid_layout.rs:65`) and, through `Theme::semantic_tokens`, a
// `Questionnaire`'s metrics (gpui-component
// `questionnaire/components.rs:87-88`, `:1219`).
// `None` means the platform specifies nothing (platform-facts §2.20).

/// Space between adjacent widgets.
#[must_use]
pub fn widget_gap(layout: &LayoutTheme) -> Option<Pixels> {
    layout_metric(layout.widget_gap)
}

/// Padding inside containers.
#[must_use]
pub fn container_margin(layout: &LayoutTheme) -> Option<Pixels> {
    layout_metric(layout.container_margin)
}

/// Padding inside the main window.
#[must_use]
pub fn window_margin(layout: &LayoutTheme) -> Option<Pixels> {
    layout_metric(layout.window_margin)
}

/// Space between major content sections.
#[must_use]
pub fn section_gap(layout: &LayoutTheme) -> Option<Pixels> {
    layout_metric(layout.section_gap)
}

/// One of the four layout metrics above, `None` where the theme states none.
pub(crate) fn layout_metric(v: Option<f32>) -> Option<Pixels> {
    v.map(px)
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
    /// windows-11 and kde-breeze state padding sides, windows-11 the list's
    /// row height and adwaita the menu's, so each builder is compared with
    /// stated values as well as with unstated ones.
    const CASES: &[(&str, ColorMode)] = &[
        ("catppuccin-mocha", ColorMode::Dark),
        ("catppuccin-latte", ColorMode::Light),
        ("windows-11", ColorMode::Light),
        ("kde-breeze", ColorMode::Dark),
        ("adwaita", ColorMode::Light),
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

    /// A family, text size, weight, style and the platform's line height,
    /// which every builder that sets a text size carries.
    fn assert_text(out: &StyleRefinement, r: &ResolvedTheme, font: &ResolvedFontSpec, s: f32) {
        assert_eq!(out.text.font_family, Some(crate::font_family(&font.family)));
        assert_eq!(out.text.font_size, abs(font.size * s));
        assert_eq!(out.text.line_height, Some(relative(r.defaults.line_height)));
        assert_eq!(
            out.text.font_weight,
            Some(FontWeight(f32::from(font.weight)))
        );
        assert_eq!(out.text.font_style, Some(crate::font_style(font.style)));
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

    /// The row the theme states, or upstream's Medium one, grown by
    /// [`table_row_height`], whose formula `tests/seams.rs` checks against the
    /// drawn row (`a_data_table_row_holds_its_text_at_every_scale`).
    #[test]
    fn data_table_size_falls_back_to_upstream_medium() {
        let mut stated = 0;
        let mut unstated = 0;
        let mut grown = 0;
        for_each_case(|r, _, n| {
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
            let expected = table_row_height(base, &r.list.item_font, n);
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
                    let expected = stated.map(|h| table_row_height(px(h), &r.list.item_font, n));
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
        let expected = table_row_height(px(stated), &r.list.item_font, n);
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
    /// Input root sets no text colour (`input/input.rs:715-808`) and gpui-base
    /// paints the field's text in the window's (gpui-base
    /// `input/base/element.rs:3182-3185`), which is `foreground`, fed from
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

    /// `accordion_hover` is `expander.hover_background` where the theme
    /// states one, and nothing where it does not: an unset soft hover is the
    /// row at rest.
    #[test]
    fn accordion_hover_is_the_stated_hover_or_nothing() {
        let (mut stated, mut unstated) = (0usize, 0usize);
        for info in Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let r = resolved(info.key, mode);
                let at = format!("{}/{mode:?}", info.key);
                let out = accordion_hover(Native::unscaled(&r));
                match r.expander.hover_background {
                    Some(fill) => {
                        stated += 1;
                        assert_eq!(
                            out,
                            StyleRefinement::default().bg(rgba_to_hsla(fill)),
                            "{at}"
                        );
                    }
                    None => {
                        unstated += 1;
                        assert_eq!(out, StyleRefinement::default(), "{at}");
                    }
                }
            }
        }
        assert!(
            stated > 0 && unstated > 0,
            "both arms must be exercised ({stated} stated, {unstated} unstated)"
        );
    }

    /// What upstream's `TitleBar` paints for `title_bar` and `background`
    /// (`title_bar.rs:22-37`, at `:345-348`), written out again from the
    /// tokens it reads.
    fn upstream_bar(title_bar: gpui::Hsla, background: gpui::Hsla) -> gpui::Background {
        let (t, b) = (title_bar.to_rgb(), background.to_rgb());
        let mixed = gpui::Hsla::from(gpui::Rgba {
            r: t.r * 0.55 + b.r * 0.45,
            g: t.g * 0.55 + b.g * 0.45,
            b: t.b * 0.55 + b.b * 0.45,
            a: t.a * 0.55 + b.a * 0.45,
        });
        gpui::linear_gradient(
            180.,
            gpui::linear_color_stop(mixed, 0.),
            gpui::linear_color_stop(title_bar, 1.),
        )
    }

    /// Where the inactive pair is the active one, the inactive form draws
    /// what upstream draws for the active bar -- its gradient from the
    /// installed `title_bar` and `background` tokens -- under the active
    /// text colour, so the bar does not change on a focus change.
    #[test]
    fn an_inactive_bar_with_the_active_pair_is_the_active_bar() {
        use gpui::Refineable as _;
        for info in Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let mut r = resolved(info.key, mode);
                let at = format!("{}/{mode:?}", info.key);
                r.window.inactive_title_bar_background = r.window.title_bar_background;
                r.window.inactive_title_bar_text_color = r.window.title_bar_font.color;
                let n = Native::unscaled(&r);
                let tc = crate::colors::to_theme_color(&r, mode == ColorMode::Dark, false);
                let active = title_bar(n).bg(upstream_bar(tc.title_bar, tc.background));
                let mut inactive = title_bar(n);
                inactive.refine(&title_bar_inactive(n));
                assert_eq!(inactive, active, "{at}");
            }
        }
    }

    /// `title_bar_inactive` is the window's inactive title-bar pair, the fill
    /// as upstream's gradient, and nothing else; the pair differs from the
    /// active one in some preset, or the builder would change nothing.
    #[test]
    fn title_bar_inactive_is_the_platforms_inactive_pair() {
        let mut differs = 0usize;
        for info in Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let r = resolved(info.key, mode);
                let at = format!("{}/{mode:?}", info.key);
                let w = &r.window;
                assert_eq!(
                    title_bar_inactive(Native::unscaled(&r)),
                    StyleRefinement::default()
                        .bg(upstream_bar(
                            rgba_to_hsla(w.inactive_title_bar_background),
                            rgba_to_hsla(w.background_color)
                        ))
                        .text_color(rgba_to_hsla(w.inactive_title_bar_text_color)),
                    "{at}"
                );
                differs += usize::from(
                    w.inactive_title_bar_background != w.title_bar_background
                        || w.inactive_title_bar_text_color != w.title_bar_font.color,
                );
            }
        }
        assert!(
            differs > 0,
            "no preset's inactive title bar differs from its active one"
        );
    }

    /// `combo_box_fill` is `combo_box.background_color`, or for a disabled
    /// trigger `combo_box.disabled_background` (the enabled fill where none
    /// is stated) and `combo_box.disabled_text_color`, faded by
    /// `combo_box.disabled_opacity`; it carries nothing else.
    #[test]
    fn combo_box_fill_is_the_platforms_in_either_state() {
        let mut stated_disabled = 0usize;
        for info in Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let r = resolved(info.key, mode);
                let n = Native::unscaled(&r);
                let at = format!("{}/{mode:?}", info.key);
                let c = &r.combo_box;
                let enabled = combo_box_fill(n, false);
                assert_eq!(
                    enabled,
                    StyleRefinement::default().bg(rgba_to_hsla(c.background_color)),
                    "{at}: enabled, the fill alone"
                );
                let disabled = combo_box_fill(n, true);
                assert_eq!(
                    disabled.background,
                    Some(rgba_to_hsla(c.disabled_background.unwrap_or(c.background_color)).into()),
                    "{at}: disabled fill"
                );
                assert_eq!(
                    disabled.text.color,
                    Some(rgba_to_hsla(c.disabled_text_color)),
                    "{at}: disabled text"
                );
                assert_eq!(
                    disabled.opacity,
                    Some(c.disabled_opacity),
                    "{at}: disabled fade"
                );
                stated_disabled += usize::from(c.disabled_background.is_some());
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
        assert!(
            stated_disabled > 0,
            "no preset states a disabled combo box fill"
        );
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

    /// `toolbar_button_rest` keeps a tool button's padding box: under
    /// kde-breeze light, which states the toolbar's hover outline, each side
    /// is the stated 6 less `button.border.line_width`, the border that wide
    /// and transparent, the fill transparent; under macos-tahoe, which states
    /// no outline, the refinement is empty.
    #[test]
    fn toolbar_button_rest_keeps_the_padding_box() {
        let r = resolved("kde-breeze", ColorMode::Light);
        let b = &r.button;
        assert!(r.toolbar.button_hover_border_color.is_some());
        let p = &b.border.padding;
        assert_eq!(
            [p.top, p.right, p.bottom, p.left],
            [Some(6.0); 4],
            "kde-breeze's stated button padding"
        );
        let w = b.border.line_width;
        assert!(w > 0.0, "a border width of 0 would prove nothing");
        let out = toolbar_button_rest(Native::unscaled(&r));
        let inside = def(6.0 - w);
        assert_eq!(out.padding.top, inside, "top");
        assert_eq!(out.padding.right, inside, "right");
        assert_eq!(out.padding.bottom, inside, "bottom");
        assert_eq!(out.padding.left, inside, "left");
        assert_eq!(out.border_widths.top, abs(w));
        assert_eq!(out.border_widths.right, abs(w));
        assert_eq!(out.border_widths.bottom, abs(w));
        assert_eq!(out.border_widths.left, abs(w));
        assert_eq!(out.border_color, Some(Hsla::transparent_black()));
        assert_eq!(out.background, Some(Hsla::transparent_black().into()));

        let r = resolved("macos-tahoe", ColorMode::Light);
        assert!(r.toolbar.button_hover_border_color.is_none());
        assert_eq!(
            toolbar_button_rest(Native::unscaled(&r)),
            StyleRefinement::default()
        );
    }

    /// `toolbar_button_checked` fills a checked toolbar button with
    /// `toolbar.button_checked_background` and letters it in
    /// `button.checked_text_color`, or `button.active_text_color` where that
    /// is unset; with no checked fill stated it fills with
    /// `toolbar.button_active_background`, else `button.active_background`,
    /// and sets no text colour; under a theme that states no toolbar fill nor
    /// `button.active_background` it sets nothing.
    #[test]
    fn toolbar_button_checked_letters_in_the_checked_text() {
        use native_theme::color::Rgba;
        let cases = [
            (
                "windows-11",
                ColorMode::Light,
                Rgba::new(0x00, 0x67, 0xc0, 0xff),
                Rgba::new(0xff, 0xff, 0xff, 0xff),
            ),
            (
                "windows-11",
                ColorMode::Dark,
                Rgba::new(0x4c, 0xc2, 0xff, 0xff),
                Rgba::new(0x00, 0x00, 0x00, 0xff),
            ),
            (
                "kde-breeze",
                ColorMode::Light,
                Rgba::new(0x23, 0x26, 0x29, 0x20),
                Rgba::new(0x23, 0x26, 0x29, 0xff),
            ),
        ];
        for (preset, mode, fill, text) in cases {
            let r = resolved(preset, mode);
            let out = toolbar_button_checked(Native::unscaled(&r));
            assert_eq!(
                out.background,
                Some(rgba_to_hsla(fill).into()),
                "{preset} {mode:?}: fill"
            );
            assert_eq!(
                out.text.color,
                Some(rgba_to_hsla(text)),
                "{preset} {mode:?}: text"
            );
        }

        let r = resolved("macos-tahoe", ColorMode::Light);
        assert_eq!(
            toolbar_button_checked(Native::unscaled(&r)),
            StyleRefinement::default(),
            "macos-tahoe states none of the three fills"
        );

        let mut r = resolved("windows-11", ColorMode::Light);
        assert_ne!(
            r.button.checked_text_color,
            Some(r.button.active_text_color),
            "windows-11's checked and pressed texts differ, or the fallback proves nothing"
        );
        r.button.checked_text_color = None;
        assert_eq!(
            toolbar_button_checked(Native::unscaled(&r)).text.color,
            Some(rgba_to_hsla(r.button.active_text_color)),
            "windows-11 with button.checked_text_color cleared"
        );

        let mut r = resolved("windows-11", ColorMode::Light);
        let pressed = Rgba::new(0x00, 0x00, 0x00, 0x06);
        assert_eq!(r.toolbar.button_active_background, Some(pressed));
        assert_ne!(
            r.button.active_background,
            Some(pressed),
            "windows-11's toolbar and button pressed fills differ, or the fallback proves nothing"
        );
        r.toolbar.button_checked_background = None;
        let out = toolbar_button_checked(Native::unscaled(&r));
        assert_eq!(
            out.background,
            Some(rgba_to_hsla(pressed).into()),
            "windows-11 with toolbar.button_checked_background cleared: fill"
        );
        assert_eq!(
            out.text.color, None,
            "windows-11 with toolbar.button_checked_background cleared: text"
        );

        let r = resolved("material-3", ColorMode::Light);
        assert_eq!(
            [
                r.toolbar.button_checked_background,
                r.toolbar.button_active_background
            ],
            [None, None],
            "material-3 states no toolbar fill"
        );
        let pressed = Rgba::new(0xd0, 0xc6, 0xdf, 0xff);
        assert_eq!(r.button.active_background, Some(pressed));
        let out = toolbar_button_checked(Native::unscaled(&r));
        assert_eq!(
            out.background,
            Some(rgba_to_hsla(pressed).into()),
            "material-3 light: fill"
        );
        assert_eq!(out.text.color, None, "material-3 light: text");
    }

    /// A checked toolbar Button takes `toolbar.button_checked_border_color`
    /// as its border colour where the theme states it (kde-breeze light), and
    /// no border colour where it is cleared.
    #[test]
    fn toolbar_button_checked_takes_the_checked_pen() {
        let mut r = resolved("kde-breeze", ColorMode::Light);
        let pen = r.toolbar.button_checked_border_color;
        assert!(pen.is_some(), "kde-breeze states the checked pen");
        assert_eq!(
            toolbar_button_checked(Native::unscaled(&r)).border_color,
            pen.map(rgba_to_hsla),
            "kde-breeze light: the checked pen"
        );
        r.toolbar.button_checked_border_color = None;
        assert_eq!(
            toolbar_button_checked(Native::unscaled(&r)).border_color,
            None,
            "the checked pen cleared: no border colour"
        );
    }

    /// No bundled preset states the toolbar's hover outline without its
    /// checked fill: `toolbar_button_rest`'s transparent fill is replayed over
    /// the selected state, so `toolbar_button_checked` must have a fill to set.
    #[test]
    fn every_preset_with_a_toolbar_outline_states_its_checked_fill() {
        let mut outlined = 0usize;
        for info in Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let r = resolved(info.key, mode);
                if r.toolbar.button_hover_border_color.is_none() {
                    continue;
                }
                outlined += 1;
                assert!(
                    r.toolbar.button_checked_background.is_some(),
                    "{}/{mode:?}: a checked toolbar button would show no fill in gpui \
                     (toolbar_button_rest's transparent fill is replayed over the selected state)",
                    info.key
                );
            }
        }
        assert!(
            outlined > 0,
            "no preset states the outline, so this proves nothing"
        );
    }

    /// `default_button_rest` is the stated fill and frame: under kde-breeze
    /// light #d6ecf8 framed #87c0dd; with `button.default_button_border_color`
    /// cleared, the frame is `button.border.color`; under adwaita light, which
    /// states no fill, it is `None`.
    #[test]
    fn default_button_rest_is_the_stated_fill_and_frame() {
        use native_theme::color::Rgba;
        let mut r = resolved("kde-breeze", ColorMode::Light);
        let out = default_button_rest(Native::unscaled(&r));
        assert!(out.is_some(), "kde-breeze light states the fill");
        let Some(out) = out else {
            return;
        };
        assert_eq!(
            out.background,
            Some(rgba_to_hsla(Rgba::rgb(0xd6, 0xec, 0xf8)).into()),
            "kde-breeze light: fill"
        );
        assert_eq!(
            out.border_color,
            Some(rgba_to_hsla(Rgba::rgb(0x87, 0xc0, 0xdd))),
            "kde-breeze light: frame"
        );

        assert_ne!(
            r.button.default_button_border_color,
            Some(r.button.border.color),
            "kde-breeze's frame is its border colour, so the fallback proves nothing"
        );
        r.button.default_button_border_color = None;
        assert_eq!(
            default_button_rest(Native::unscaled(&r)).and_then(|out| out.border_color),
            Some(rgba_to_hsla(r.button.border.color)),
            "kde-breeze light with button.default_button_border_color cleared"
        );

        let r = resolved("adwaita", ColorMode::Light);
        assert_eq!(r.button.default_button_background, None);
        assert_eq!(
            default_button_rest(Native::unscaled(&r)),
            None,
            "adwaita light"
        );
    }

    /// `list_item_selected` letters a selected row in
    /// `list.selection_text_color` and carries nothing else: upstream changes
    /// only a selected row's fill (`list/list_item.rs:257-264`), so its text
    /// stays the `foreground` set before the refinement (`:205` → `:209`), or
    /// [`list_item`]'s `list.item_font.color`, which the platforms' selected
    /// text differs from.
    #[test]
    fn a_selected_list_row_is_lettered_in_the_platforms_selection_colour() {
        let mut differs = 0usize;
        for info in Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let r = resolved(info.key, mode);
                let at = format!("{}/{mode:?}", info.key);
                let out = list_item_selected(Native::unscaled(&r));
                assert_eq!(
                    out.text.color,
                    Some(rgba_to_hsla(r.list.selection_text_color)),
                    "{at}: selected row text"
                );
                let bare = StyleRefinement {
                    text: Default::default(),
                    ..out
                };
                assert_eq!(bare, StyleRefinement::default(), "{at}: only the text");
                differs += usize::from(r.list.selection_text_color != r.list.item_font.color);
            }
        }
        assert!(
            differs > 0,
            "no preset's selected row text differs from its row text any more, \
             so this test proves nothing"
        );
    }

    /// `link` carries the link's font, and removes upstream's
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
                assert_eq!(
                    out.background,
                    Some(rgba_to_hsla(r.link.background_color).into()),
                    "{at}: fill"
                );
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

    /// `menu_item`'s height where the theme states `menu.min_row_height`:
    /// kde-breeze's 29 as a least height and no fixed one; with
    /// `menu.row_height` 30 stated too, the fixed 30 and no least height (a
    /// stated row height wins); adwaita's stated 32, no least height.
    #[test]
    fn menu_item_takes_the_least_row_height() {
        let mut r = resolved("kde-breeze", ColorMode::Light);
        assert_eq!(
            (r.menu.row_height, r.menu.min_row_height),
            (None, Some(29.)),
            "kde-breeze states a least menu row height of 29 and no fixed one"
        );
        let out = menu_item(Native::unscaled(&r));
        assert_eq!(out.min_size.height, len(29.), "kde-breeze: least height");
        assert_eq!(out.size.height, None, "kde-breeze: no fixed height");
        r.menu.row_height = Some(30.);
        let out = menu_item(Native::unscaled(&r));
        assert_eq!(
            out.size.height,
            len(30.),
            "row_height 30 stated too: height"
        );
        assert_eq!(
            out.min_size.height, None,
            "row_height 30 stated too: no least height"
        );
        let r = resolved("adwaita", ColorMode::Light);
        assert_eq!(
            r.menu.min_row_height, None,
            "adwaita states no least height"
        );
        let out = menu_item(Native::unscaled(&r));
        assert_eq!(out.size.height, len(32.), "adwaita: its stated row_height");
        assert_eq!(out.min_size.height, None, "adwaita: no least height");
    }

    #[test]
    fn menu_and_list_items_match_theme_values() {
        let mut stated = Stated::default();
        for_each_case(|r, s, n| {
            let m = &r.menu;
            let out = menu_item(n);
            match (m.row_height, m.min_row_height) {
                (None, Some(least)) => {
                    assert_eq!(out.min_size.height, len(least), "menu_item: least height");
                    assert_eq!(out.size.height, None, "menu_item: no fixed height");
                }
                _ => stated.height(&out, r, m.row_height, s, "menu_item"),
            }
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
            assert_eq!(out.border_widths.top, abs(p.border.line_width));
            assert_eq!(out.border_color, Some(rgba_to_hsla(p.border.color)));

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
    /// no colour on that element at all. Ten are checked here, over every
    /// preset in both modes, not just the `CASES` names; [`input`] and
    /// [`text_area`] in `a_field_draws_its_text_in_the_platforms_input_colour`.
    ///
    /// The `assert_ne!`s below are the reason the first three exist: each of
    /// those native colours differs from the token upstream would otherwise
    /// paint, so leaving the colour out left the widget labelled with a colour
    /// the platform did not state.
    ///
    /// The other seven -- `list_item`, `radio`, `select`, `title_bar`,
    /// `dialog_title`, `accordion_title`, `toolbar` -- get no
    /// such `assert_ne!`, and deliberately: their native font colour equals
    /// `defaults.text_color` in 32 of the 34 combinations today (ios-13's
    /// select is the exception, its `combo_box.font.color` the accent), because
    /// a preset that states no `font.color` for a widget inherits the window's
    /// (docs/inheritance-rules.toml:121-142), and `foreground` is fed from
    /// `defaults.text_color`. The colour is carried anyway because nothing
    /// stands in its way -- upstream labels the first three with `foreground`
    /// one refinement earlier and mutes the disabled state on a child
    /// afterwards, an open accordion title with `foreground` and no disabled
    /// colour at all, and it labels the title bar, a dialog's title and a
    /// toolbar with nothing at all (`title_bar.rs:332-349`,
    /// `dialog/title.rs:39-46`, `toolbar.rs:243-257`) -- so
    /// the day a source states
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
                // (child muted at `:261-262`) and `select.rs:573` -> `:580`
                // (through `input_style`, `input/input.rs:126`; the title child
                // re-mutes at `select.rs:511-513`).
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
                // (`title_bar.rs:332-349`), so the route is open and nothing
                // is displaced -- and the KDE reader does state the field,
                // from `[WM] activeForeground` (`native-theme/src/kde/colors.rs`).
                assert_eq!(
                    title_bar(n).text.color,
                    Some(rgba_to_hsla(r.window.title_bar_font.color)),
                    "{at}: title bar text"
                );
                assert_eq!(
                    dialog_title(n).text.color,
                    Some(rgba_to_hsla(r.dialog.title_font.color)),
                    "{at}: dialog title text"
                );
                assert_eq!(
                    accordion_title(n).text.color,
                    Some(rgba_to_hsla(r.expander.font.color)),
                    "{at}: accordion title text"
                );
                assert_eq!(
                    toolbar(n).text.color,
                    Some(rgba_to_hsla(r.toolbar.font.color)),
                    "{at}: toolbar text"
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
                // on a tooltip (`tooltip.rs:120`).
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
    /// which gpui-component 0.9.1 gives no route for (Tier U): an iOS select's
    /// text is systemBlue (WebKit html.css@31adf9d1:1099).
    const STATED_COMBO_TEXT: [&str; 2] = ["ios-13/light", "ios-13/dark"];

    /// `geometry::checkbox` and `geometry::combobox` carry no colour although
    /// upstream paints both from `foreground`: the route is blocked past the
    /// element the refinement lands on. A `Checkbox::label` sits in a wrapper
    /// that sets `foreground` itself (`checkbox.rs:342-347`), so a carried
    /// colour never reaches it; and both widgets apply their disabled colour
    /// *before* the refinement (`checkbox.rs:263-267`, and
    /// `input_style(disabled, ..)` at `input/input.rs:120-124` feeding
    /// `combobox.rs:1009` -> `:1016`, whose selected-title branch `:603-609` sets
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
                     (checkbox.rs:342-347) so it cannot arrive, and the \
                     disabled hook (checkbox.rs:263-267) would take it instead \
                     of `muted_foreground`"
                );
                assert_eq!(
                    combobox(n).text.color,
                    None,
                    "{at}: geometry::combobox set a text colour; upstream \
                     applies the disabled `muted_foreground` before this \
                     refinement (input/input.rs:120-124 -> combobox.rs:1009 -> \
                     :1016), so it would displace the disabled colour"
                );
                assert_eq!(
                    r.checkbox.font.color, r.defaults.text_color,
                    "{at}: this preset states a checkbox label text colour \
                     that gpui-component 0.9.1 gives no route for \
                     (checkbox.rs:342-347) -- record it as a Tier U candidate; \
                     do NOT carry it in the builder, it would displace the \
                     disabled colour (checkbox.rs:263-267)"
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
                         gpui-component 0.9.1 gives no route for \
                         (combobox.rs:1009 -> :1016) -- record it as a Tier U \
                         candidate; do NOT carry it in the builder, it would \
                         displace the disabled colour (input/input.rs:120-124)"
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
            assert_eq!(out.min_size.width, len(d.min_width));
            assert_eq!(out.min_size.height, len(d.min_height));
            assert_eq!(out.max_size.height, len(d.max_height));
            assert_eq!(
                out.corner_radii.top_left,
                abs(d.border.corner_radius.max(0.0))
            );
            assert_eq!(
                out.background,
                Some(rgba_to_hsla(d.background_color).into())
            );
            assert_eq!(out.border_color, Some(rgba_to_hsla(d.border.color)));
            assert_eq!(
                dialog_title(n).text.color,
                Some(rgba_to_hsla(d.title_font.color))
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

    /// `list` fills the list view with `list.background_color` in every
    /// preset and mode: nothing upstream paints a list's fill.
    #[test]
    fn list_paints_the_list_background() {
        for info in Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let r = resolved(info.key, mode);
                assert_eq!(
                    list(Native::unscaled(&r)).background,
                    Some(rgba_to_hsla(r.list.background_color).into()),
                    "{}/{mode:?}",
                    info.key
                );
            }
        }
    }

    /// `editor` is the platform's line height and nothing else, in every
    /// preset and mode: the Editor's font is the theme's mono font already.
    #[test]
    fn editor_carries_the_platforms_line_height_alone() {
        for info in Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let r = resolved(info.key, mode);
                let out = editor(Native::unscaled(&r));
                let at = format!("{}/{mode:?}", info.key);
                assert_eq!(
                    out.text.line_height,
                    Some(relative(r.defaults.line_height)),
                    "{at}"
                );
                let mut rest = out.clone();
                rest.text.line_height = None;
                assert_eq!(
                    rest,
                    StyleRefinement::default(),
                    "{at}: more than the line height"
                );
            }
        }
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
                out.background,
                Some(rgba_to_hsla(c.background_color).into())
            );

            assert_eq!(
                accordion_title(n).size.height,
                len(r.expander.header_height)
            );
            assert_eq!(
                accordion_title(n).text.color,
                Some(rgba_to_hsla(r.expander.font.color))
            );
            assert_eq!(
                accordion(n),
                StyleRefinement::default().border_color(rgba_to_hsla(r.expander.border.color))
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
            // The trigger's edge is `combo_box.border.line_width`, over
            // upstream's `border_1` (`select.rs:569`, `combobox.rs:1005`).
            assert_eq!(out.border_widths.top, abs(cb.border.line_width));
            assert_eq!(combobox(n).border_widths.top, abs(cb.border.line_width));
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
    /// the minimum reset to `auto` where it is not, and its bottom edge the
    /// model's `toolbar.border`, on no other side.
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
                    Some(t.bar_height.map_or(gpui::Length::Auto, |h| px(h).into())),
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
                assert_eq!(
                    out.text.color,
                    Some(rgba_to_hsla(t.font.color)),
                    "{at}: text colour"
                );
                let edge = StyleRefinement::default().border_b(px(t.border.line_width));
                assert_eq!(out.border_widths, edge.border_widths, "{at}: bottom edge");
                assert_eq!(
                    out.border_color,
                    Some(rgba_to_hsla(t.border.color)),
                    "{at}: edge colour"
                );
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

    /// Without a stated bar height the minimum is reset to `auto`, so the bar
    /// is its content's height, whatever upstream's `Toolbar` sets.
    #[test]
    fn toolbar_resets_the_min_height_without_a_bar_height() {
        let mut r = resolved("catppuccin-mocha", ColorMode::Dark);
        r.toolbar.bar_height = None;
        r.toolbar.item_gap = None;
        let out = toolbar(Native::unscaled(&r));
        assert_eq!(out.min_size.height, Some(gpui::Length::Auto));
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

    /// The KDE reader states a family and a style for `menu.font`,
    /// `toolbar.font` and `window.title_bar_font` (native-theme
    /// `src/kde/fonts.rs`), whose only receivers are [`menu_item`],
    /// [`toolbar`] and [`title_bar`]: each carries both, the family through
    /// the path the installed theme's own takes.
    #[test]
    fn the_builders_carry_a_stated_family_and_style() {
        let family: std::sync::Arc<str> = std::sync::Arc::from("Noto Serif");
        let mut r = resolved("kde-breeze", ColorMode::Light);
        for font in [
            &mut r.menu.font,
            &mut r.toolbar.font,
            &mut r.window.title_bar_font,
        ] {
            font.family = family.clone();
            font.style = native_theme::theme::FontStyle::Italic;
        }
        let n = Native::unscaled(&r);
        for (what, out) in [
            ("menu_item", menu_item(n)),
            ("toolbar", toolbar(n)),
            ("title_bar", title_bar(n)),
        ] {
            assert_eq!(
                out.text.font_family,
                Some(crate::font_family(&family)),
                "{what}: family"
            );
            assert_eq!(
                out.text.font_style,
                Some(gpui::FontStyle::Italic),
                "{what}: style"
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
