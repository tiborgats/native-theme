//! Connector parity of the crate root's functions (docs/v0.8.0-plan.md,
//! section 1, rule 2): each function of [`super`] that has a twin at the
//! crate root, or a 0.7.2 field it reads in its place, gives on the 0.8.0
//! model what the twin gives on the 0.7.2 model, on every preset of
//! [`super::parity::PRESETS`], light and dark.
//!
//! The two sides are compared by their `Debug` forms, a value of the 0.8.0
//! side that may be unstated as `Ok` of the 0.7.2 side's value, and a 0.7.2
//! type the 0.8.0 model names anew (`DialogButtonOrder`) by its variant's
//! name. A difference is allowed only as an entry of [`DIFFERENCES`], with
//! its reason; an entry that differs nowhere is stale and fails.

use std::fmt::Debug;

use super::parity::{PRESETS, resolve};
use super::{Mapped, Unmapped};

/// A listed difference: the function, the preset's key, the 0.8.0 side's
/// value it asserts there (its `Debug` form), and why.
struct Difference {
    name: &'static str,
    preset: &'static str,
    new: New,
    reason: &'static str,
}

/// The 0.8.0 side's value a listed difference asserts: as written, or
/// computed from the homes the 0.8.0 preset states (`None` where it leaves
/// one unstated, which fails).
enum New {
    Is(&'static str),
    Computed(fn(&native_theme::next::ResolvedTheme) -> Option<String>),
}

impl New {
    /// The `Debug` form the entry asserts on `new`.
    fn on(&self, new: &native_theme::next::ResolvedTheme) -> Option<String> {
        match self {
            Self::Is(v) => Some((*v).to_owned()),
            Self::Computed(f) => f(new),
        }
    }
}

/// `button.body(NONE)`'s padding inside its border, from its homes: each
/// stated side plus the stated stroke's width, a stroke stated as none
/// adding 0 (spec §10, rule 2); `None` where a side or the stroke is
/// unstated, or the stroke is per side and not reduced (`Stroke::prevailing`).
fn body_padding(new: &native_theme::next::ResolvedTheme) -> Option<iced_core::Padding> {
    use native_theme::next::{PartStated, Stroke, WidgetState};

    let body = new.button.body(WidgetState::NONE);
    if !body.stated.contains(PartStated::STROKE) {
        return None;
    }
    let width = match body.stroke.prevailing() {
        Stroke::None => 0.0,
        Stroke::Uniform { width, .. } => width,
        _ => return None,
    };
    let side = |bit: PartStated, v: f32| body.stated.contains(bit).then_some(v + width);
    let p = &body.padding;
    Some(iced_core::Padding {
        top: side(PartStated::PADDING_TOP, p.top)?,
        right: side(PartStated::PADDING_RIGHT, p.right)?,
        bottom: side(PartStated::PADDING_BOTTOM, p.bottom)?,
        left: side(PartStated::PADDING_LEFT, p.left)?,
    })
}

/// `focus_ring` round a field of corner `radius`, where the 0.8.0 theme
/// states a ring inset at the edge (its offset minus its stroke's width)
/// of one uniform stroke and no inner stroke or corners: the ring of
/// `focus_ring.stroke`'s colour and width at `focus_ring.offset`, shown for
/// keyboard focus only where `focus_ring.trigger` says so; `None` where it
/// states another.
#[cfg(feature = "widgets")]
fn stated_inset_ring(new: &native_theme::next::ResolvedTheme, radius: f32) -> Option<String> {
    use native_theme::next::{FocusKind, FocusTrigger, Stroke};
    let f = &new.focus_ring;
    let Stroke::Uniform { width, color, .. } = f.stroke else {
        return None;
    };
    let plain =
        f.kind == FocusKind::Ring && matches!(f.inner_stroke, Stroke::None) && f.corners.is_none();
    (plain && width > 0.0 && f.offset == -width).then(|| {
        format!(
            "{:?}",
            ok(Some(crate::focus_ring::RingLook {
                color: super::to_color(color),
                width,
                offset: f.offset,
                radius,
                keyboard_only: f.trigger == Some(FocusTrigger::KeyboardOnly),
                inner: None,
                corners: None,
            }))
        )
    })
}

/// [`stated_inset_ring`] round a square field.
#[cfg(feature = "widgets")]
fn stated_inset_ring_square(new: &native_theme::next::ResolvedTheme) -> Option<String> {
    stated_inset_ring(new, 0.0)
}

/// [`stated_inset_ring`] round a field of the default corner radius
/// (`layout.corner_radius`, which `border_corner_radius` gives on both
/// models alike).
#[cfg(feature = "widgets")]
fn stated_inset_ring_rounded(new: &native_theme::next::ResolvedTheme) -> Option<String> {
    stated_inset_ring(new, super::border_corner_radius(new))
}

/// `focus_ring` is compared with the `widgets` feature only, which builds
/// it: no value to assert without it.
#[cfg(not(feature = "widgets"))]
fn stated_inset_ring_square(_: &native_theme::next::ResolvedTheme) -> Option<String> {
    None
}

/// As [`stated_inset_ring_square`].
#[cfg(not(feature = "widgets"))]
fn stated_inset_ring_rounded(_: &native_theme::next::ResolvedTheme) -> Option<String> {
    None
}

/// The reason of adwaita's focus ring, which the 0.8.0 side draws.
const INSET_RING: &str = "adwaita's focus ring is inset at the field's edge (2 wide at offset \
     -2, libadwaita 1.10.0 src/stylesheet/_drawing.scss, focus-ring), which 0.7.2's connector \
     drew as the focused field's border and so not round it (crate::focus_ring::ring_look: \
     None); the 0.8.0 side paints it over the field as the band over its outermost 2 px, the \
     field's styles leaving it to the ring (docs/v0.8.0-spec.md §2, FocusIndicator: the \
     geometry of a Ring)";

/// `button_padding` as [`body_padding`] computes it.
fn stated_button_padding(new: &native_theme::next::ResolvedTheme) -> Option<String> {
    Some(format!("{:?}", ok(body_padding(new)?)))
}

/// `button_content_min_size`: `button.body(NONE).min_size` less
/// [`body_padding`] on each axis, no less than 0.
fn stated_content_min_size(new: &native_theme::next::ResolvedTheme) -> Option<String> {
    let p = body_padding(new)?;
    let [width, height] = new
        .button
        .body(native_theme::next::WidgetState::NONE)
        .min_size;
    Some(format!(
        "{:?}",
        ok([
            width.map(|w| (w - p.left - p.right).max(0.0)),
            height.map(|h| (h - p.top - p.bottom).max(0.0)),
        ])
    ))
}

/// `combo_box_padding` from `combo_box.field(NONE)`'s homes: each stated
/// side plus the stated stroke's width, a stroke stated as none adding 0
/// (spec §10, rule 2), an unstated side iced's own
/// (`iced_widget::button::DEFAULT_PADDING`); `None` where the stroke is
/// unstated or per side, or the right side is stated (an arrow column then
/// widens it, which this does not reckon).
#[cfg(feature = "widgets")]
fn stated_combo_box_padding(new: &native_theme::next::ResolvedTheme) -> Option<String> {
    use native_theme::next::{PartStated, Stroke, WidgetState};

    let field = new.combo_box.field(WidgetState::NONE);
    if !field.stated.contains(PartStated::STROKE)
        || field.stated.contains(PartStated::PADDING_RIGHT)
    {
        return None;
    }
    let width = match field.stroke.prevailing() {
        Stroke::None => 0.0,
        Stroke::Uniform { width, .. } => width,
        _ => return None,
    };
    let own = iced_widget::button::DEFAULT_PADDING;
    let side = |bit: PartStated, v: f32, own: f32| {
        if field.stated.contains(bit) {
            v + width
        } else {
            own
        }
    };
    let p = &field.padding;
    Some(format!(
        "{:?}",
        ok(iced_core::Padding {
            top: side(PartStated::PADDING_TOP, p.top, own.top),
            right: own.right,
            bottom: side(PartStated::PADDING_BOTTOM, p.bottom, own.bottom),
            left: side(PartStated::PADDING_LEFT, p.left, own.left),
        })
    ))
}

/// `combo_box_padding` is compared with the `widgets` feature only, which
/// builds it: no value to assert without it.
#[cfg(not(feature = "widgets"))]
fn stated_combo_box_padding(_: &native_theme::next::ResolvedTheme) -> Option<String> {
    None
}

/// The radio look as the builder reads it, its dot's colour from
/// `radio.dot(CHECKED).fill` (`radio_look`), the circle's width from
/// `radio.indicator(NONE).min_size`, the dot's diameter from
/// `radio.dot(NONE).min_size`; `None` where the dot's diameter or colour is
/// unstated.
#[cfg(feature = "widgets")]
fn stated_radio_dot(new: &native_theme::next::ResolvedTheme) -> Option<String> {
    use native_theme::next::WidgetState;

    let w = &new.radio;
    let [width, _] = w.indicator(WidgetState::NONE).min_size;
    let [diameter, _] = w.dot(WidgetState::NONE).min_size;
    let color = super::fill_of(&w.dot(WidgetState::CHECKED), "radio.dot(CHECKED).fill")
        .ok()
        .flatten()?;
    Some(format!(
        "{:?}",
        ok(crate::RadioLook {
            width,
            label_gap: w.label_gap,
            dot: Some((diameter?, color)),
        })
    ))
}

/// `radio` is compared with the `widgets` feature only, which builds it: no
/// value to assert without it.
#[cfg(not(feature = "widgets"))]
fn stated_radio_dot(_: &native_theme::next::ResolvedTheme) -> Option<String> {
    None
}

/// The focus ring round a field whose radius is `radius`, from the homes the
/// 0.8.0 preset states: `focus_ring.stroke`'s colour and width, its
/// `offset`, keyboard only where its `trigger` says, and the second ring
/// from `focus_ring.inner_stroke`; `None` where either stroke is not one
/// width and colour.
#[cfg(feature = "widgets")]
fn stated_ring(new: &native_theme::next::ResolvedTheme, radius: f32) -> Option<String> {
    use native_theme::next::{FocusTrigger, Stroke};

    let f = &new.focus_ring;
    let (
        Stroke::Uniform { width, color, .. },
        Stroke::Uniform {
            width: inner_width,
            color: inner_color,
            ..
        },
    ) = (&f.stroke, &f.inner_stroke)
    else {
        return None;
    };
    Some(format!(
        "{:?}",
        ok(Some(crate::focus_ring::RingLook {
            color: super::to_color(*color),
            width: *width,
            offset: f.offset,
            radius,
            keyboard_only: f.trigger == Some(FocusTrigger::KeyboardOnly),
            inner: Some((super::to_color(*inner_color), *inner_width)),
            corners: None,
        }))
    ))
}

/// [`stated_ring`] round a square field.
#[cfg(feature = "widgets")]
fn stated_square_ring(new: &native_theme::next::ResolvedTheme) -> Option<String> {
    stated_ring(new, 0.0)
}

/// [`stated_ring`] round a field of the default radius.
#[cfg(feature = "widgets")]
fn stated_rounded_ring(new: &native_theme::next::ResolvedTheme) -> Option<String> {
    stated_ring(new, new.layout.corner_radius)
}

/// The focus ring is compared with the `widgets` feature only, which builds
/// it: no value to assert without it.
#[cfg(not(feature = "widgets"))]
fn stated_square_ring(_: &native_theme::next::ResolvedTheme) -> Option<String> {
    None
}

/// As [`stated_square_ring`] without the `widgets` feature.
#[cfg(not(feature = "widgets"))]
fn stated_rounded_ring(_: &native_theme::next::ResolvedTheme) -> Option<String> {
    None
}

/// The reason of windows-11's focus ring.
const W11_INNER_RING: &str = "windows-11 states the focus visual's 1px inner ring in \
     FocusStrokeColorInner, #B3FFFFFF / #B3000000 (docs/platform-facts.md §2.1.5 \
     focus_ring_color and focus_ring_offset; Common_themeresources_any.xaml:259, :55 at \
     microsoft-ui-xaml 8463f45), where 0.7.2 has no colour for it and states no inner ring";

/// The reason of material-3's radio dot.
const M3_RADIO_DOT: &str = "material-3 states the checked radio's dot in primary, \
     selected-icon-color (radio/internal/_radio.scss:126-127, \
     tokens/versions/v0_192/_md-comp-radio-button.scss:45 at material-web cbd34a8; \
     native-theme/tests/next_homes.toml, row checkbox.indicator_color, a listed difference), \
     where 0.7.2 draws it in the accent's text colour, as its check mark";

/// The reason of the macOS 26 pop-up button's padding.
const NO_POPUP_LINE: &str = "macos-tahoe states its pop-up button's outline as none (M2.FS4, \
     measured: os-probe run 45198750893, macOS 26, `*.normal.popup` `edge_*`; \
     native-theme/tests/next_homes.toml, rows combo_box.border.color and \
     combo_box.border.line_width, listed differences): the padding inside the border is the \
     stated padding plus a width of 0 (spec §10, rule 2), where 0.7.2 added its 0.5 hairline";

/// The reason of the macOS 26 push button's padding.
const NO_BEZEL_LINE: &str = "macos-tahoe states its push button's bezel stroked as none (M2.FS4, \
     measured: `docs/facts/macos/button.toml` on `v0.8.0-rc1`, `button.body.stroke` absent): the \
     padding inside the border is the stated padding plus a width of 0 (spec §10, rule 2), where \
     0.7.2 added its 0.5 hairline, and the content's least size is the body's less that padding";

/// The reason of Breeze's dialog size limits.
const Z2: &str = "the 0.8.0 theme states no dialog size limits where no source gives them \
     (docs/v0.8.0-spec.md §5, issue #4's leftovers, Z2: `dialog.panel.min_size` / `max_size`, \
     `None`); 0.7.2's kde-breeze preset states values no source gives";

/// The reason of Breeze's button height floor.
const Z1: &str = "the 0.8.0 theme states a button's minimum width and no minimum height \
     (docs/v0.8.0-spec.md §5, issue #4's leftovers, Z1: `button.body.min_size`, \
     `[Some(w), None]`); 0.7.2's kde-breeze preset states a height";

const DIFFERENCES: &[Difference] = &[
    Difference {
        name: "dialog_max_width",
        preset: "kde-breeze",
        new: New::Is("None"),
        reason: Z2,
    },
    Difference {
        name: "dialog_max_height",
        preset: "kde-breeze",
        new: New::Is("None"),
        reason: Z2,
    },
    Difference {
        name: "dialog_min_width",
        preset: "kde-breeze",
        new: New::Is("None"),
        reason: Z2,
    },
    Difference {
        name: "dialog_min_height",
        preset: "kde-breeze",
        new: New::Is("None"),
        reason: Z2,
    },
    Difference {
        name: "button_content_min_size",
        preset: "kde-breeze",
        new: New::Is("Ok([Some(68.0), None])"),
        reason: Z1,
    },
    Difference {
        name: "button_padding",
        preset: "macos-tahoe",
        new: New::Computed(stated_button_padding),
        reason: NO_BEZEL_LINE,
    },
    Difference {
        name: "button_content_min_size",
        preset: "macos-tahoe",
        new: New::Computed(stated_content_min_size),
        reason: NO_BEZEL_LINE,
    },
    Difference {
        name: "combo_box_padding",
        preset: "macos-tahoe",
        new: New::Computed(stated_combo_box_padding),
        reason: NO_POPUP_LINE,
    },
    Difference {
        name: "focus_ring",
        preset: "adwaita",
        new: New::Computed(stated_inset_ring_square),
        reason: INSET_RING,
    },
    Difference {
        name: "focus_ring",
        preset: "adwaita",
        new: New::Computed(stated_inset_ring_rounded),
        reason: INSET_RING,
    },
    Difference {
        name: "radio",
        preset: "material-3",
        new: New::Computed(stated_radio_dot),
        reason: M3_RADIO_DOT,
    },
    Difference {
        name: "focus_ring",
        preset: "windows-11",
        new: New::Computed(stated_square_ring),
        reason: W11_INNER_RING,
    },
    Difference {
        name: "focus_ring",
        preset: "windows-11",
        new: New::Computed(stated_rounded_ring),
        reason: W11_INNER_RING,
    },
];

/// The pairs compared on one preset and scheme.
struct Pairs {
    label: String,
    preset: &'static str,
    /// The value each entry of [`DIFFERENCES`] asserts on this side's
    /// 0.8.0 theme, by its place.
    asserted: Vec<Option<String>>,
    failures: Vec<String>,
    seen: Vec<(&'static str, &'static str)>,
}

impl Pairs {
    fn pair(&mut self, name: &'static str, old: impl Debug, new: impl Debug) {
        let (old, new) = (format!("{old:?}"), format!("{new:?}"));
        if old == new {
            return;
        }
        if DIFFERENCES.iter().zip(&self.asserted).any(|(d, asserted)| {
            d.name == name
                && d.preset == self.preset
                && asserted.as_deref() == Some(new.as_str())
                && !d.reason.is_empty()
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

/// The accessors the parity script holds iced's `next` to.
fn accessors(p: &mut Pairs, old: &crate::ResolvedTheme, new: &native_theme::next::ResolvedTheme) {
    let d = &old.defaults;
    p.pair(
        "border_line_width",
        d.border.line_width,
        super::border_line_width(new),
    );
    p.pair(
        "border_opacity",
        d.border.opacity,
        super::border_opacity(new),
    );
    p.pair(
        "border_shadow_enabled",
        d.border.shadow_enabled,
        super::border_shadow_enabled(new),
    );
    p.pair(
        "disabled_opacity",
        crate::disabled_opacity(old),
        super::disabled_opacity(new),
    );
    p.pair(
        "disabled_text_color",
        crate::palette::to_color(d.disabled_text_color),
        super::disabled_text_color(new),
    );
    p.pair(
        "focus_ring_width",
        ok(d.focus_ring_width),
        super::focus_ring_width(new),
    );
    p.pair(
        "focus_ring_offset",
        d.focus_ring_offset,
        super::focus_ring_offset(new),
    );
    let (oi, ni) = (crate::icon_sizes(old), super::icon_sizes(new));
    p.pair(
        "icon_sizes",
        [oi.toolbar, oi.small, oi.large, oi.dialog, oi.panel].map(Some),
        [ni.toolbar, ni.small, ni.large, ni.dialog, ni.panel],
    );
    let (ot, nt) = (crate::text_scale(old), super::text_scale(new));
    for (role, o, n) in [
        ("text_scale.caption", &ot.caption, &nt.caption),
        (
            "text_scale.section_heading",
            &ot.section_heading,
            &nt.section_heading,
        ),
        (
            "text_scale.dialog_title",
            &ot.dialog_title,
            &nt.dialog_title,
        ),
        ("text_scale.display", &ot.display, &nt.display),
    ] {
        p.pair(
            role,
            (o.size, o.weight, o.line_height),
            (n.size_px, n.weight, n.line_height),
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
        "dialog_button_order",
        ok(format!("{:?}", crate::dialog_button_order(old))),
        super::dialog_button_order(new).map(|o| format!("{o:?}")),
    );
    p.pair(
        "dialog_button_gap",
        crate::dialog_button_gap(old),
        super::dialog_button_gap(new),
    );
    p.pair(
        "scrollbar_groove_width",
        ok(crate::scrollbar_groove_width(old)),
        super::scrollbar_groove_width(new),
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
}

/// The crate root's helpers.
fn helpers(p: &mut Pairs, old: &crate::ResolvedTheme, new: &native_theme::next::ResolvedTheme) {
    p.pair(
        "border_color",
        ok(crate::border_color(old)),
        super::border_color(new),
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
        "focus_ring_color",
        crate::focus_ring_color(old),
        super::focus_ring_color(new),
    );
    p.pair("link_color", crate::link_color(old), super::link_color(new));
    p.pair(
        "selection_background",
        crate::selection_background(old),
        super::selection_background(new),
    );
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
        "dialog_title_font_color",
        ok(crate::dialog_title_font_color(old)),
        super::dialog_title_font_color(new),
    );
    p.pair(
        "expander_arrow_color",
        ok(crate::expander_arrow_color(old)),
        super::expander_arrow_color(new),
    );
    p.pair(
        "font_family",
        crate::font_family(old),
        super::font_family(new),
    );
    p.pair(
        "mono_font_family",
        crate::mono_font_family(old),
        super::mono_font_family(new),
    );
    let prefs = crate::AccessibilityPreferences::default();
    p.pair(
        "font_size",
        crate::font_size(old, &prefs),
        super::font_size(new),
    );
    p.pair(
        "mono_font_size",
        crate::mono_font_size(old, &prefs),
        super::mono_font_size(new),
    );
    p.pair(
        "dialog_max_width",
        Some(crate::dialog_max_width(old)),
        super::dialog_max_width(new),
    );
    p.pair(
        "dialog_max_height",
        Some(crate::dialog_max_height(old)),
        super::dialog_max_height(new),
    );
    p.pair(
        "dialog_min_width",
        Some(crate::dialog_min_width(old)),
        super::dialog_min_width(new),
    );
    p.pair(
        "dialog_min_height",
        Some(crate::dialog_min_height(old)),
        super::dialog_min_height(new),
    );
    p.pair(
        "list_header_height",
        crate::list_header_height(old),
        super::list_header_height(new),
    );
    p.pair(
        "toolbar_bar_height",
        crate::toolbar_bar_height(old),
        super::toolbar_bar_height(new),
    );
    p.pair(
        "toolbar_item_gap",
        crate::toolbar_item_gap(old),
        super::toolbar_item_gap(new),
    );
    p.pair(
        "toolbar_border_line_width",
        ok(crate::toolbar_border_line_width(old)),
        super::toolbar_border_line_width(new),
    );
    let own = iced_core::Padding::new(1.0);
    p.pair(
        "dialog_padding",
        ok(crate::dialog_padding(old, own)),
        super::dialog_padding(new, own),
    );
    p.pair(
        "toolbar_padding",
        crate::toolbar_padding(old, own),
        super::toolbar_padding(new, own),
    );
    let (size, min) = (old.defaults.font.size, old.input.min_height);
    p.pair(
        "control_line_height",
        crate::control_line_height(old, size, min, own),
        super::control_line_height(new, size, min, own),
    );
    #[cfg(feature = "widgets")]
    {
        p.pair(
            "button_padding",
            ok(crate::button_padding(old)),
            super::button_padding(new),
        );
        p.pair(
            "input_padding",
            ok(crate::input_padding(old)),
            super::input_padding(new),
        );
        p.pair(
            "text_area_padding",
            ok(crate::text_area_padding(old)),
            super::text_area_padding(new),
        );
        p.pair(
            "combo_box_padding",
            ok(crate::combo_box_padding(old)),
            super::combo_box_padding(new),
        );
        let s = crate::button_content_min_size(old);
        p.pair(
            "button_content_min_size",
            ok([Some(s.width), Some(s.height)]),
            super::button_content_min_size(new),
        );
    }
}

/// The crate root's widgets and widget parts, compared by what each builds
/// its widget from: the sizes, colours and look its shared builder takes.
#[cfg(feature = "widgets")]
fn widgets(p: &mut Pairs, old: &crate::ResolvedTheme, new: &native_theme::next::ResolvedTheme) {
    let switch = super::switch_look(new);
    p.pair(
        "switch",
        ok((
            crate::switch_look(old),
            crate::palette::to_color(old.defaults.background_color),
        )),
        switch.and_then(|look| {
            super::stated(new.colors.window.background, "colors.window.background")
                .map(|window| (look, super::to_color(window)))
        }),
    );
    p.pair("switch_over", ok(crate::switch_look(old)), switch);
    p.pair("radio", ok(crate::radio_look(old)), super::radio_look(new));
    p.pair(
        "pick_list_handle",
        Some(old.combo_box.arrow_icon_size),
        super::chevron_size(new),
    );
    let indicator = super::tab_indicator_look(new);
    p.pair(
        "tab_indicator",
        ok(crate::tab_indicator_look(&old.tab)),
        indicator,
    );
    p.pair(
        "tab_indicator_line",
        ok(crate::tab_indicator_look(&old.tab)),
        indicator,
    );
    p.pair(
        "tab_flares",
        ok(crate::tab_flares::tab_flares_look(&old.tab)),
        super::tab_flares_look(new),
    );
    for radius in [0.0, crate::border_corner_radius(old)] {
        p.pair(
            "focus_ring",
            ok(crate::focus_ring::ring_look(old, radius)),
            super::ring_look(new, radius),
        );
    }
}

/// The spinner, compared by what it reads of the theme.
#[cfg(feature = "spinner")]
fn spinner(p: &mut Pairs, old: &crate::ResolvedTheme, new: &native_theme::next::ResolvedTheme) {
    p.pair(
        "spinner",
        ok(crate::spinner::Look::of(old)),
        super::spinner_look(new),
    );
}

/// The fonts of the body and the monospace text.
#[cfg(feature = "system-fonts")]
fn fonts(p: &mut Pairs, old: &crate::ResolvedTheme, new: &native_theme::next::ResolvedTheme) {
    let d = &old.defaults;
    let t = &new.typography;
    for (o, n) in [(&d.font, &t.body), (&d.mono_font, &t.monospace)] {
        p.pair("font", ok(crate::font(o)), super::font(n));
        p.pair(
            "system_font_family",
            ok(crate::system_font_family(o)),
            super::system_font_family(n),
        );
    }
    // 0.7.2's consumers drew a role in `defaults.font`'s family and slant at
    // the role's weight.
    let (ot, nt) = (crate::text_scale(old), super::text_scale(new));
    for (role, o, n) in [
        ("text_scale.caption font", &ot.caption, &nt.caption),
        (
            "text_scale.section_heading font",
            &ot.section_heading,
            &nt.section_heading,
        ),
        (
            "text_scale.dialog_title font",
            &ot.dialog_title,
            &nt.dialog_title,
        ),
        ("text_scale.display font", &ot.display, &nt.display),
    ] {
        let drawn = native_theme::theme::ResolvedFontSpec {
            weight: o.weight,
            ..d.font.clone()
        };
        p.pair(role, ok(crate::font(&drawn)), super::font(n));
    }
}

#[test]
fn the_crate_roots_functions_agree_on_both_models() -> crate::Result<()> {
    let mut failures = Vec::new();
    let mut seen = Vec::new();
    for &(key, preset) in PRESETS {
        for is_dark in [false, true] {
            let (old, _, _, new) = resolve(key, preset, is_dark)?;
            let mut p = Pairs {
                label: format!("{key} {}", if is_dark { "dark" } else { "light" }),
                preset: key,
                asserted: DIFFERENCES.iter().map(|d| d.new.on(&new)).collect(),
                failures: Vec::new(),
                seen: Vec::new(),
            };
            accessors(&mut p, &old, &new);
            helpers(&mut p, &old, &new);
            #[cfg(feature = "widgets")]
            widgets(&mut p, &old, &new);
            #[cfg(feature = "spinner")]
            spinner(&mut p, &old, &new);
            #[cfg(feature = "system-fonts")]
            fonts(&mut p, &old, &new);
            failures.extend(p.failures);
            seen.extend(p.seen);
        }
    }
    for d in DIFFERENCES {
        // The button's, the combo box's, the radio's helpers and the focus
        // ring are compared with the `widgets` feature only.
        if matches!(
            d.name,
            "button_content_min_size"
                | "button_padding"
                | "combo_box_padding"
                | "radio"
                | "focus_ring"
        ) && !cfg!(feature = "widgets")
        {
            continue;
        }
        if !seen.contains(&(d.name, d.preset)) {
            failures.push(format!(
                "{} on {}: listed, but differs nowhere",
                d.name, d.preset
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} values differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}
