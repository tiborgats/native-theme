//! Test-only: the exhaustive-destructuring diff of spec §13.1 — its path spelling is
//! normative for `mapping.toml` — and the atlas walk that T2, T4, T5, T10, T13 and T16 share.
//!
//! An `f32` compares by value, with `NaN == NaN`, never through `Vec2`'s `PartialEq`, whose
//! debug assert rejects a `NaN` component (`emath/src/vec2.rs:349-359`): T4 walks hostile
//! atlases with this. `CornerRadius` is one path; `Stroke`, `Margin`, `Shadow`, `Vec2` and
//! `FontId` are split into their fields; `text_styles` keys are spelled `text_styles[Body]`.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]

use std::collections::BTreeSet;
use std::sync::Arc;

use egui::epaint::text::TextOptions;
use egui::style::{
    ImeComposition, Interaction, ScrollAnimation, ScrollFadeStyle, ScrollStyle, Selection, Spacing,
    Style, TextCursorStyle, Visuals, WidgetVisuals, Widgets,
};
use egui::{FontId, Frame, Margin, Shadow, Stroke, Vec2};

use crate::{Role, RoleVariant, Surface, ThemeAtlas};

/// Where a sink lives (§13.1): the base style, a role cell, or a surface frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Location {
    Base(egui::Theme),
    Cell(egui::Theme, Role, RoleVariant),
    Frame(egui::Theme, Surface),
}

/// One field that differs between two atlases.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Change {
    pub location: Location,
    pub path: String,
}

/// `v == v`, spelled so that `clippy::eq_op` sees two operands: `false` exactly when a `NaN` is
/// inside (T4, T10's sweep).
pub(crate) fn self_equal<T: PartialEq>(v: &T) -> bool {
    let other = v;
    v == other
}

/// The visitor: `(parent path, field suffix, changed)`; the caller joins the two only when it
/// keeps the path, so a walk allocates per struct, not per leaf.
type Emit<'a> = &'a mut dyn FnMut(&str, &str, bool);

fn f32_eq(a: f32, b: f32) -> bool {
    a == b || (a.is_nan() && b.is_nan())
}

fn leaf_f32(p: &str, field: &str, a: f32, b: f32, f: Emit<'_>) {
    f(p, field, !f32_eq(a, b));
}

fn leaf<T: PartialEq>(p: &str, field: &str, a: &T, b: &T, f: Emit<'_>) {
    f(p, field, a != b);
}

fn vec2(p: &str, field: &str, a: Vec2, b: Vec2, f: Emit<'_>) {
    let Vec2 { x: ax, y: ay } = a;
    let Vec2 { x: bx, y: by } = b;
    let p = format!("{p}{field}");
    leaf_f32(&p, ".x", ax, bx, f);
    leaf_f32(&p, ".y", ay, by, f);
}

fn margin(p: &str, field: &str, a: Margin, b: Margin, f: Emit<'_>) {
    let Margin {
        left: al,
        right: ar,
        top: at,
        bottom: ab,
    } = a;
    let Margin {
        left: bl,
        right: br,
        top: bt,
        bottom: bb,
    } = b;
    let p = format!("{p}{field}");
    leaf(&p, ".left", &al, &bl, f);
    leaf(&p, ".right", &ar, &br, f);
    leaf(&p, ".top", &at, &bt, f);
    leaf(&p, ".bottom", &ab, &bb, f);
}

fn stroke(p: &str, field: &str, a: Stroke, b: Stroke, f: Emit<'_>) {
    let Stroke {
        width: aw,
        color: ac,
    } = a;
    let Stroke {
        width: bw,
        color: bc,
    } = b;
    let p = format!("{p}{field}");
    leaf_f32(&p, ".width", aw, bw, f);
    leaf(&p, ".color", &ac, &bc, f);
}

fn shadow(p: &str, field: &str, a: Shadow, b: Shadow, f: Emit<'_>) {
    let Shadow {
        offset: ao,
        blur: abl,
        spread: asp,
        color: ac,
    } = a;
    let Shadow {
        offset: bo,
        blur: bbl,
        spread: bsp,
        color: bc,
    } = b;
    let p = format!("{p}{field}");
    leaf(&p, ".offset", &ao, &bo, f);
    leaf(&p, ".blur", &abl, &bbl, f);
    leaf(&p, ".spread", &asp, &bsp, f);
    leaf(&p, ".color", &ac, &bc, f);
}

fn font_id(p: &str, a: &FontId, b: &FontId, f: Emit<'_>) {
    let FontId {
        size: asz,
        family: af,
    } = a;
    let FontId {
        size: bsz,
        family: bf,
    } = b;
    leaf_f32(p, ".size", *asz, *bsz, f);
    leaf(p, ".family", af, bf, f);
}

fn opt_font_id(p: &str, field: &str, a: Option<&FontId>, b: Option<&FontId>, f: Emit<'_>) {
    let p = format!("{p}{field}");
    match (a, b) {
        (Some(a), Some(b)) => font_id(&p, a, b, f),
        (None, None) => {
            f(&p, ".size", false);
            f(&p, ".family", false);
        }
        _ => {
            f(&p, ".size", true);
            f(&p, ".family", true);
        }
    }
}

fn widget_visuals(p: &str, field: &str, a: &WidgetVisuals, b: &WidgetVisuals, f: Emit<'_>) {
    let WidgetVisuals {
        bg_fill: abf,
        weak_bg_fill: awf,
        bg_stroke: abs,
        corner_radius: acr,
        fg_stroke: afs,
        expansion: ae,
    } = *a;
    let WidgetVisuals {
        bg_fill: bbf,
        weak_bg_fill: bwf,
        bg_stroke: bbs,
        corner_radius: bcr,
        fg_stroke: bfs,
        expansion: be,
    } = *b;
    let p = format!("{p}{field}");
    leaf(&p, ".bg_fill", &abf, &bbf, f);
    leaf(&p, ".weak_bg_fill", &awf, &bwf, f);
    stroke(&p, ".bg_stroke", abs, bbs, f);
    leaf(&p, ".corner_radius", &acr, &bcr, f);
    stroke(&p, ".fg_stroke", afs, bfs, f);
    leaf_f32(&p, ".expansion", ae, be, f);
}

fn widgets(p: &str, field: &str, a: &Widgets, b: &Widgets, f: Emit<'_>) {
    let Widgets {
        noninteractive: an,
        inactive: ai,
        hovered: ah,
        active: aa,
        open: ao,
    } = a;
    let Widgets {
        noninteractive: bn,
        inactive: bi,
        hovered: bh,
        active: ba,
        open: bo,
    } = b;
    let p = format!("{p}{field}");
    widget_visuals(&p, ".noninteractive", an, bn, f);
    widget_visuals(&p, ".inactive", ai, bi, f);
    widget_visuals(&p, ".hovered", ah, bh, f);
    widget_visuals(&p, ".active", aa, ba, f);
    widget_visuals(&p, ".open", ao, bo, f);
}

fn selection(p: &str, field: &str, a: &Selection, b: &Selection, f: Emit<'_>) {
    let Selection {
        bg_fill: abf,
        stroke: ast,
    } = *a;
    let Selection {
        bg_fill: bbf,
        stroke: bst,
    } = *b;
    let p = format!("{p}{field}");
    leaf(&p, ".bg_fill", &abf, &bbf, f);
    stroke(&p, ".stroke", ast, bst, f);
}

fn ime(p: &str, field: &str, a: &ImeComposition, b: &ImeComposition, f: Emit<'_>) {
    let ImeComposition {
        active_underline_stroke: aa,
        inactive_underline_stroke: ai,
        legacy_visuals: al,
    } = *a;
    let ImeComposition {
        active_underline_stroke: ba,
        inactive_underline_stroke: bi,
        legacy_visuals: bl,
    } = *b;
    let p = format!("{p}{field}");
    stroke(&p, ".active_underline_stroke", aa, ba, f);
    stroke(&p, ".inactive_underline_stroke", ai, bi, f);
    leaf(&p, ".legacy_visuals", &al, &bl, f);
}

fn text_cursor(p: &str, field: &str, a: &TextCursorStyle, b: &TextCursorStyle, f: Emit<'_>) {
    let TextCursorStyle {
        stroke: ast,
        preview: ap,
        blink: abl,
        on_duration: aon,
        off_duration: aoff,
    } = *a;
    let TextCursorStyle {
        stroke: bst,
        preview: bp,
        blink: bbl,
        on_duration: bon,
        off_duration: boff,
    } = *b;
    let p = format!("{p}{field}");
    stroke(&p, ".stroke", ast, bst, f);
    leaf(&p, ".preview", &ap, &bp, f);
    leaf(&p, ".blink", &abl, &bbl, f);
    leaf_f32(&p, ".on_duration", aon, bon, f);
    leaf_f32(&p, ".off_duration", aoff, boff, f);
}

fn text_options(p: &str, field: &str, a: &TextOptions, b: &TextOptions, f: Emit<'_>) {
    let TextOptions {
        max_texture_side: am,
        color_transfer_function: ac,
        font_hinting: ah,
        subpixel_binning: asb,
    } = *a;
    let TextOptions {
        max_texture_side: bm,
        color_transfer_function: bc,
        font_hinting: bh,
        subpixel_binning: bsb,
    } = *b;
    let p = format!("{p}{field}");
    leaf(&p, ".max_texture_side", &am, &bm, f);
    leaf(&p, ".color_transfer_function", &ac, &bc, f);
    leaf(&p, ".font_hinting", &ah, &bh, f);
    leaf(&p, ".subpixel_binning", &asb, &bsb, f);
}

fn scroll_fade(p: &str, field: &str, a: &ScrollFadeStyle, b: &ScrollFadeStyle, f: Emit<'_>) {
    let ScrollFadeStyle {
        strength: ast,
        size: asz,
    } = *a;
    let ScrollFadeStyle {
        strength: bst,
        size: bsz,
    } = *b;
    let p = format!("{p}{field}");
    leaf_f32(&p, ".strength", ast, bst, f);
    leaf_f32(&p, ".size", asz, bsz, f);
}

fn scroll_animation(p: &str, field: &str, a: &ScrollAnimation, b: &ScrollAnimation, f: Emit<'_>) {
    let ScrollAnimation {
        points_per_second: aps,
        duration: ad,
    } = *a;
    let ScrollAnimation {
        points_per_second: bps,
        duration: bd,
    } = *b;
    let egui::Rangef {
        min: admin,
        max: admax,
    } = ad;
    let egui::Rangef {
        min: bdmin,
        max: bdmax,
    } = bd;
    let p = format!("{p}{field}");
    leaf_f32(&p, ".points_per_second", aps, bps, f);
    leaf_f32(&p, ".duration.min", admin, bdmin, f);
    leaf_f32(&p, ".duration.max", admax, bdmax, f);
}

fn scroll(p: &str, field: &str, a: &ScrollStyle, b: &ScrollStyle, f: Emit<'_>) {
    let ScrollStyle {
        floating: afl,
        content_margin: acm,
        bar_width: abw,
        handle_min_length: ahm,
        bar_inner_margin: abi,
        bar_outer_margin: abo,
        floating_width: afw,
        floating_allocated_width: afa,
        foreground_color: afc,
        dormant_background_opacity: adb,
        active_background_opacity: aab,
        interact_background_opacity: aib,
        dormant_handle_opacity: adh,
        active_handle_opacity: aah,
        interact_handle_opacity: aih,
        fade: afd,
    } = a;
    let ScrollStyle {
        floating: bfl,
        content_margin: bcm,
        bar_width: bbw,
        handle_min_length: bhm,
        bar_inner_margin: bbi,
        bar_outer_margin: bbo,
        floating_width: bfw,
        floating_allocated_width: bfa,
        foreground_color: bfc,
        dormant_background_opacity: bdb,
        active_background_opacity: bab,
        interact_background_opacity: bib,
        dormant_handle_opacity: bdh,
        active_handle_opacity: bah,
        interact_handle_opacity: bih,
        fade: bfd,
    } = b;
    let p = format!("{p}{field}");
    leaf(&p, ".floating", afl, bfl, f);
    margin(&p, ".content_margin", *acm, *bcm, f);
    leaf_f32(&p, ".bar_width", *abw, *bbw, f);
    leaf_f32(&p, ".handle_min_length", *ahm, *bhm, f);
    leaf_f32(&p, ".bar_inner_margin", *abi, *bbi, f);
    leaf_f32(&p, ".bar_outer_margin", *abo, *bbo, f);
    leaf_f32(&p, ".floating_width", *afw, *bfw, f);
    leaf_f32(&p, ".floating_allocated_width", *afa, *bfa, f);
    leaf(&p, ".foreground_color", afc, bfc, f);
    leaf_f32(&p, ".dormant_background_opacity", *adb, *bdb, f);
    leaf_f32(&p, ".active_background_opacity", *aab, *bab, f);
    leaf_f32(&p, ".interact_background_opacity", *aib, *bib, f);
    leaf_f32(&p, ".dormant_handle_opacity", *adh, *bdh, f);
    leaf_f32(&p, ".active_handle_opacity", *aah, *bah, f);
    leaf_f32(&p, ".interact_handle_opacity", *aih, *bih, f);
    scroll_fade(&p, ".fade", afd, bfd, f);
}

fn interaction(p: &str, field: &str, a: &Interaction, b: &Interaction, f: Emit<'_>) {
    let Interaction {
        interact_radius: air,
        resize_grab_radius_side: ars,
        resize_grab_radius_corner: arc,
        show_tooltips_only_when_still: ast,
        tooltip_delay: atd,
        tooltip_grace_time: atg,
        selectable_labels: asl,
        multi_widget_text_select: amw,
    } = *a;
    let Interaction {
        interact_radius: bir,
        resize_grab_radius_side: brs,
        resize_grab_radius_corner: brc,
        show_tooltips_only_when_still: bst,
        tooltip_delay: btd,
        tooltip_grace_time: btg,
        selectable_labels: bsl,
        multi_widget_text_select: bmw,
    } = *b;
    let p = format!("{p}{field}");
    leaf_f32(&p, ".interact_radius", air, bir, f);
    leaf_f32(&p, ".resize_grab_radius_side", ars, brs, f);
    leaf_f32(&p, ".resize_grab_radius_corner", arc, brc, f);
    leaf(&p, ".show_tooltips_only_when_still", &ast, &bst, f);
    leaf_f32(&p, ".tooltip_delay", atd, btd, f);
    leaf_f32(&p, ".tooltip_grace_time", atg, btg, f);
    leaf(&p, ".selectable_labels", &asl, &bsl, f);
    leaf(&p, ".multi_widget_text_select", &amw, &bmw, f);
}

fn spacing(p: &str, field: &str, a: &Spacing, b: &Spacing, f: Emit<'_>) {
    let Spacing {
        item_spacing: ais,
        window_margin: awm,
        button_padding: abp,
        menu_margin: amm,
        indent: ain,
        interact_size: aiz,
        slider_width: asw,
        slider_rail_height: asr,
        combo_width: acw,
        text_edit_width: atw,
        extra_text_line_spacing: aet,
        icon_width: aiw,
        icon_width_inner: aii,
        icon_spacing: aic,
        default_area_size: ada,
        tooltip_width: atp,
        menu_width: amw,
        menu_spacing: ams,
        indent_ends_with_horizontal_line: aih,
        combo_height: ach,
        scroll: asc,
    } = a;
    let Spacing {
        item_spacing: bis,
        window_margin: bwm,
        button_padding: bbp,
        menu_margin: bmm,
        indent: bin,
        interact_size: biz,
        slider_width: bsw,
        slider_rail_height: bsr,
        combo_width: bcw,
        text_edit_width: btw,
        extra_text_line_spacing: bet,
        icon_width: biw,
        icon_width_inner: bii,
        icon_spacing: bic,
        default_area_size: bda,
        tooltip_width: btp,
        menu_width: bmw,
        menu_spacing: bms,
        indent_ends_with_horizontal_line: bih,
        combo_height: bch,
        scroll: bsc,
    } = b;
    let p = format!("{p}{field}");
    vec2(&p, ".item_spacing", *ais, *bis, f);
    margin(&p, ".window_margin", *awm, *bwm, f);
    vec2(&p, ".button_padding", *abp, *bbp, f);
    margin(&p, ".menu_margin", *amm, *bmm, f);
    leaf_f32(&p, ".indent", *ain, *bin, f);
    vec2(&p, ".interact_size", *aiz, *biz, f);
    leaf_f32(&p, ".slider_width", *asw, *bsw, f);
    leaf_f32(&p, ".slider_rail_height", *asr, *bsr, f);
    leaf_f32(&p, ".combo_width", *acw, *bcw, f);
    leaf_f32(&p, ".text_edit_width", *atw, *btw, f);
    leaf_f32(&p, ".extra_text_line_spacing", *aet, *bet, f);
    leaf_f32(&p, ".icon_width", *aiw, *biw, f);
    leaf_f32(&p, ".icon_width_inner", *aii, *bii, f);
    leaf_f32(&p, ".icon_spacing", *aic, *bic, f);
    vec2(&p, ".default_area_size", *ada, *bda, f);
    leaf_f32(&p, ".tooltip_width", *atp, *btp, f);
    leaf_f32(&p, ".menu_width", *amw, *bmw, f);
    leaf_f32(&p, ".menu_spacing", *ams, *bms, f);
    leaf(&p, ".indent_ends_with_horizontal_line", aih, bih, f);
    leaf_f32(&p, ".combo_height", *ach, *bch, f);
    scroll(&p, ".scroll", asc, bsc, f);
}

/// `clip_rect_margin` is deprecated upstream (`egui/src/style.rs:1086`) and still a field.
#[expect(deprecated)]
fn visuals(p: &str, field: &str, a: &Visuals, b: &Visuals, f: Emit<'_>) {
    let Visuals {
        dark_mode: adm,
        text_options: ato,
        override_text_color: aot,
        weak_text_alpha: awa,
        weak_text_color: awc,
        widgets: awg,
        selection: asl,
        ime_composition: aim,
        hyperlink_color: ahl,
        faint_bg_color: afb,
        extreme_bg_color: aeb,
        text_edit_bg_color: ate,
        code_bg_color: acb,
        warn_fg_color: awf,
        error_fg_color: aef,
        window_corner_radius: awr,
        window_shadow: aws,
        window_fill: awi,
        window_stroke: awk,
        window_highlight_topmost: awh,
        menu_corner_radius: amr,
        panel_fill: apf,
        popup_shadow: aps,
        resize_corner_size: arc,
        text_cursor: atc,
        clip_rect_margin: acr,
        button_frame: abf,
        collapsing_header_frame: ach,
        indent_has_left_vline: aiv,
        striped: ast,
        slider_trailing_fill: asf,
        handle_shape: ahs,
        interact_cursor: aic,
        image_loading_spinners: ail,
        numeric_color_space: anc,
        disabled_alpha: ada,
    } = a;
    let Visuals {
        dark_mode: bdm,
        text_options: bto,
        override_text_color: bot,
        weak_text_alpha: bwa,
        weak_text_color: bwc,
        widgets: bwg,
        selection: bsl,
        ime_composition: bim,
        hyperlink_color: bhl,
        faint_bg_color: bfb,
        extreme_bg_color: beb,
        text_edit_bg_color: bte,
        code_bg_color: bcb,
        warn_fg_color: bwf,
        error_fg_color: bef,
        window_corner_radius: bwr,
        window_shadow: bws,
        window_fill: bwi,
        window_stroke: bwk,
        window_highlight_topmost: bwh,
        menu_corner_radius: bmr,
        panel_fill: bpf,
        popup_shadow: bps,
        resize_corner_size: brc,
        text_cursor: btc,
        clip_rect_margin: bcr,
        button_frame: bbf,
        collapsing_header_frame: bch,
        indent_has_left_vline: biv,
        striped: bst,
        slider_trailing_fill: bsf,
        handle_shape: bhs,
        interact_cursor: bic,
        image_loading_spinners: bil,
        numeric_color_space: bnc,
        disabled_alpha: bda,
    } = b;
    let p = format!("{p}{field}");
    leaf(&p, ".dark_mode", adm, bdm, f);
    text_options(&p, ".text_options", ato, bto, f);
    leaf(&p, ".override_text_color", aot, bot, f);
    leaf_f32(&p, ".weak_text_alpha", *awa, *bwa, f);
    leaf(&p, ".weak_text_color", awc, bwc, f);
    widgets(&p, ".widgets", awg, bwg, f);
    selection(&p, ".selection", asl, bsl, f);
    ime(&p, ".ime_composition", aim, bim, f);
    leaf(&p, ".hyperlink_color", ahl, bhl, f);
    leaf(&p, ".faint_bg_color", afb, bfb, f);
    leaf(&p, ".extreme_bg_color", aeb, beb, f);
    leaf(&p, ".text_edit_bg_color", ate, bte, f);
    leaf(&p, ".code_bg_color", acb, bcb, f);
    leaf(&p, ".warn_fg_color", awf, bwf, f);
    leaf(&p, ".error_fg_color", aef, bef, f);
    leaf(&p, ".window_corner_radius", awr, bwr, f);
    shadow(&p, ".window_shadow", *aws, *bws, f);
    leaf(&p, ".window_fill", awi, bwi, f);
    stroke(&p, ".window_stroke", *awk, *bwk, f);
    leaf(&p, ".window_highlight_topmost", awh, bwh, f);
    leaf(&p, ".menu_corner_radius", amr, bmr, f);
    leaf(&p, ".panel_fill", apf, bpf, f);
    shadow(&p, ".popup_shadow", *aps, *bps, f);
    leaf_f32(&p, ".resize_corner_size", *arc, *brc, f);
    text_cursor(&p, ".text_cursor", atc, btc, f);
    leaf_f32(&p, ".clip_rect_margin", *acr, *bcr, f);
    leaf(&p, ".button_frame", abf, bbf, f);
    leaf(&p, ".collapsing_header_frame", ach, bch, f);
    leaf(&p, ".indent_has_left_vline", aiv, biv, f);
    leaf(&p, ".striped", ast, bst, f);
    leaf(&p, ".slider_trailing_fill", asf, bsf, f);
    leaf(&p, ".handle_shape", ahs, bhs, f);
    leaf(&p, ".interact_cursor", aic, bic, f);
    leaf(&p, ".image_loading_spinners", ail, bil, f);
    leaf(&p, ".numeric_color_space", anc, bnc, f);
    leaf_f32(&p, ".disabled_alpha", *ada, *bda, f);
}

/// Walk every field of two `Style`s (17 fields, `egui/src/style.rs:244-342`). The path of a
/// leaf is the field chain rooted at `Style` with the `Style.` prefix dropped (§13.1).
pub(crate) fn walk_style(a: &Style, b: &Style, f: Emit<'_>) {
    let Style {
        override_text_style: aots,
        override_font_id: aofi,
        override_text_valign: aotv,
        text_styles: ats,
        drag_value_text_style: adv,
        number_formatter: anf,
        wrap_mode: awm,
        spacing: asp,
        interaction: ain,
        visuals: avi,
        animation_time: aat,
        #[cfg(debug_assertions)]
            debug: adb,
        explanation_tooltips: aet,
        url_in_tooltip: aut,
        always_scroll_the_only_direction: aas,
        scroll_animation: asa,
        compact_menu_style: acm,
    } = a;
    let Style {
        override_text_style: bots,
        override_font_id: bofi,
        override_text_valign: botv,
        text_styles: bts,
        drag_value_text_style: bdv,
        number_formatter: bnf,
        wrap_mode: bwm,
        spacing: bsp,
        interaction: bin,
        visuals: bvi,
        animation_time: bat,
        #[cfg(debug_assertions)]
            debug: bdb,
        explanation_tooltips: bet,
        url_in_tooltip: but,
        always_scroll_the_only_direction: bas,
        scroll_animation: bsa,
        compact_menu_style: bcm,
    } = b;
    leaf("", "override_text_style", aots, bots, f);
    opt_font_id("", "override_font_id", aofi.as_ref(), bofi.as_ref(), f);
    leaf("", "override_text_valign", aotv, botv, f);
    let keys: BTreeSet<&egui::TextStyle> = ats.keys().chain(bts.keys()).collect();
    for k in keys {
        let p = format!("text_styles[{k:?}]");
        match (ats.get(k), bts.get(k)) {
            (Some(x), Some(y)) => font_id(&p, x, y, f),
            _ => {
                f(&p, ".size", true);
                f(&p, ".family", true);
            }
        }
    }
    leaf("", "drag_value_text_style", adv, bdv, f);
    // Not a path: a callback no theme leaf reaches, and its `PartialEq` is `Arc::ptr_eq`
    // (`egui/src/style.rs:57-62`) over the fresh `Arc` every `Style::default` makes (`:1435`),
    // so two separate builds would always differ here. egui's own style UI skips it too (`:1793`).
    let _ = (anf, bnf);
    leaf("", "wrap_mode", awm, bwm, f);
    spacing("", "spacing", asp, bsp, f);
    interaction("", "interaction", ain, bin, f);
    visuals("", "visuals", avi, bvi, f);
    leaf_f32("", "animation_time", *aat, *bat, f);
    #[cfg(debug_assertions)]
    leaf("", "debug", adb, bdb, f);
    leaf("", "explanation_tooltips", aet, bet, f);
    leaf("", "url_in_tooltip", aut, but, f);
    leaf("", "always_scroll_the_only_direction", aas, bas, f);
    scroll_animation("", "scroll_animation", asa, bsa, f);
    leaf("", "compact_menu_style", acm, bcm, f);
}

/// Walk the six fields of two `Frame`s (`egui/src/containers/frame.rs:104-140`), spelled
/// against `egui::Frame`: `fill`, `stroke.color`, `inner_margin.left`, `corner_radius`, `shadow.blur`.
pub(crate) fn walk_frame(a: &Frame, b: &Frame, f: Emit<'_>) {
    let Frame {
        inner_margin: aim,
        fill: afi,
        stroke: ast,
        corner_radius: acr,
        outer_margin: aom,
        shadow: ash,
    } = *a;
    let Frame {
        inner_margin: bim,
        fill: bfi,
        stroke: bst,
        corner_radius: bcr,
        outer_margin: bom,
        shadow: bsh,
    } = *b;
    margin("", "inner_margin", aim, bim, f);
    leaf("", "fill", &afi, &bfi, f);
    stroke("", "stroke", ast, bst, f);
    leaf("", "corner_radius", &acr, &bcr, f);
    margin("", "outer_margin", aom, bom, f);
    shadow("", "shadow", ash, bsh, f);
}

fn collect(walk: impl FnOnce(Emit<'_>), only_changed: bool) -> Vec<String> {
    let mut out = Vec::new();
    walk(&mut |p, field, changed| {
        if changed || !only_changed {
            out.push(format!("{p}{field}"));
        }
    });
    out
}

/// The paths at which `a` and `b` differ.
pub(crate) fn style_diff(a: &Style, b: &Style) -> Vec<String> {
    collect(|f| walk_style(a, b, f), true)
}

pub(crate) fn frame_diff(a: &Frame, b: &Frame) -> Vec<String> {
    collect(|f| walk_frame(a, b, f), true)
}

/// Every path `style_diff` can emit for egui's stock key set (T10b's slot-side list).
pub(crate) fn all_style_paths() -> Vec<String> {
    let s = Style::default();
    collect(|f| walk_style(&s, &s, f), false)
}

/// Every path `frame_diff` can emit.
pub(crate) fn all_frame_paths() -> Vec<String> {
    let fr = Frame::NONE;
    collect(|f| walk_frame(&fr, &fr, f), false)
}

/// Both base styles and every cell, in every variant, in both schemes.
pub(crate) fn all_styles(atlas: &ThemeAtlas) -> Vec<(Location, Arc<Style>)> {
    let mut out = Vec::new();
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        let s = atlas.scheme(theme);
        out.push((Location::Base(theme), Arc::clone(&s.base)));
        for role in Role::all() {
            for variant in RoleVariant::all() {
                out.push((
                    Location::Cell(theme, *role, *variant),
                    Arc::clone(s.cell(*role, *variant)),
                ));
            }
        }
    }
    out
}

/// Every surface frame in both schemes.
pub(crate) fn all_frames(atlas: &ThemeAtlas) -> Vec<(Location, Frame)> {
    let mut out = Vec::new();
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        let s = atlas.scheme(theme);
        for surface in Surface::all() {
            out.push((Location::Frame(theme, *surface), s.frame(*surface)));
        }
    }
    out
}

/// Every field that differs between two atlases, at every location — the base styles, the
/// 75 cells and the 11 frames of each scheme.
pub(crate) fn atlas_diff(a: &ThemeAtlas, b: &ThemeAtlas) -> Vec<Change> {
    let mut out = Vec::new();
    for ((loc, sa), (_, sb)) in all_styles(a).into_iter().zip(all_styles(b)) {
        out.extend(style_diff(&sa, &sb).into_iter().map(|path| Change {
            location: loc,
            path,
        }));
    }
    for ((loc, fa), (_, fb)) in all_frames(a).into_iter().zip(all_frames(b)) {
        out.extend(frame_diff(&fa, &fb).into_iter().map(|path| Change {
            location: loc,
            path,
        }));
    }
    out
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]
mod tests {
    use super::*;

    /// A diff of a style with itself is empty; every emitted path is unique; and the
    /// normative spellings of §13.1 are among them.
    #[test]
    fn the_walk_is_exhaustive_and_spells_paths_as_the_manifest_does() {
        let s = egui::Theme::Dark.default_style();
        assert!(style_diff(&s, &s).is_empty());
        let paths = all_style_paths();
        let set: BTreeSet<&String> = paths.iter().collect();
        assert_eq!(set.len(), paths.len(), "no path is emitted twice");
        for want in [
            "visuals.widgets.hovered.weak_bg_fill",
            "visuals.widgets.inactive.bg_stroke.color",
            "spacing.button_padding.x",
            "spacing.interact_size.y",
            "text_styles[Body].size",
            "override_font_id.size",
            "visuals.window_shadow.color",
            "spacing.scroll.bar_width",
            "visuals.selection.stroke.width",
        ] {
            assert!(set.contains(&want.to_owned()), "{want}");
        }
        let fp = all_frame_paths();
        for want in [
            "fill",
            "stroke.color",
            "stroke.width",
            "inner_margin.left",
            "corner_radius",
            "shadow.blur",
        ] {
            assert!(fp.contains(&want.to_owned()), "{want}");
        }
    }

    /// One changed field is reported once, by its own path; a `NaN` on both sides is no change.
    /// The `self_equal` clause puts its `NaN` in a scalar field: a `NaN` in a `Vec2` makes
    /// `Style`'s derived `PartialEq` reach `Vec2::eq`, whose debug assert panics
    /// (`emath/src/vec2.rs:349-359`) before `self_equal` could return `false`.
    #[test]
    fn a_changed_field_is_reported_by_its_path() {
        let a = egui::Theme::Light.default_style();
        let mut b = a.clone();
        b.visuals.widgets.hovered.weak_bg_fill = egui::Color32::RED;
        assert_eq!(
            style_diff(&a, &b),
            vec!["visuals.widgets.hovered.weak_bg_fill".to_owned()]
        );

        let mut c = a.clone();
        c.spacing.interact_size.y = f32::NAN;
        let mut d = a.clone();
        d.spacing.interact_size.y = f32::NAN;
        assert!(
            style_diff(&c, &d).is_empty(),
            "NaN equals NaN here, and no Vec2 comparison runs"
        );
        assert_eq!(
            style_diff(&a, &c),
            vec!["spacing.interact_size.y".to_owned()]
        );
        let mut e = a.clone();
        e.spacing.indent = f32::NAN;
        assert!(
            !self_equal(&e),
            "the NaN sweep sees a NaN in a scalar field"
        );
        assert!(self_equal(&a));
    }
}
