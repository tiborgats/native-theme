//! §13 T1 and T1b: compile-time drift tripwires over the egui types this crate writes, and
//! three default-value tripwires. A field egui adds or removes fails the compile here,
//! naming the field; a default that moves fails an assertion.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]

use egui::epaint::text::TextOptions;
use egui::style::{
    ImeComposition, Interaction, ScrollAnimation, ScrollFadeStyle, ScrollStyle, Selection, Spacing,
    Style, TextCursorStyle, Visuals, WidgetVisuals, Widgets,
};
use egui::{CornerRadius, FontId, Frame, Margin, Shadow, Stroke};

/// T1. One chain of destructurings with **no `..` rest pattern**, starting from
/// `Style::default()`. A destructure binds a nested struct by name and stops there, so every
/// type reachable from a destructured one is destructured separately below.
#[test]
fn every_style_type_is_destructured_exhaustively() {
    // `Style`, 17 fields (egui/src/style.rs:244-342). `debug` exists only under
    // `debug_assertions` (:323-324): a `cfg` on a field pattern is honoured, so this one
    // pattern compiles in both profiles (Step 2 builds both).
    let Style {
        override_text_style: _,
        override_font_id: _,
        override_text_valign: _,
        text_styles,
        drag_value_text_style: _,
        number_formatter: _,
        wrap_mode: _,
        spacing,
        interaction,
        visuals,
        animation_time: _,
        #[cfg(debug_assertions)]
            debug: _,
        explanation_tooltips: _,
        url_in_tooltip: _,
        always_scroll_the_only_direction: _,
        scroll_animation,
        compact_menu_style: _,
    } = Style::default();

    // `FontId`, 2 fields (epaint/src/text/fonts.rs:23, :26): the sizes the atlas writes.
    let FontId { size: _, family: _ } = text_styles
        .get(&egui::TextStyle::Body)
        .cloned()
        .expect("egui's defaults hold Body");

    // `ScrollAnimation`, 2 fields (egui/src/style.rs:833, :836): reduced motion writes it.
    let ScrollAnimation {
        points_per_second: _,
        duration: _,
    } = scroll_animation;

    // `Spacing`, 21 fields (egui/src/style.rs:392-465).
    let Spacing {
        item_spacing: _,
        window_margin,
        button_padding: _,
        menu_margin: _,
        indent: _,
        interact_size: _,
        slider_width: _,
        slider_rail_height: _,
        combo_width: _,
        text_edit_width: _,
        extra_text_line_spacing: _,
        icon_width: _,
        icon_width_inner: _,
        icon_spacing: _,
        default_area_size: _,
        tooltip_width: _,
        menu_width: _,
        menu_spacing: _,
        indent_ends_with_horizontal_line: _,
        combo_height: _,
        scroll,
    } = spacing;

    // `Margin`, 4 fields (epaint/src/margin.rs:15-20).
    let Margin {
        left: _,
        right: _,
        top: _,
        bottom: _,
    } = window_margin;

    // `ScrollStyle`, 16 fields (egui/src/style.rs:503-582).
    let ScrollStyle {
        floating: _,
        content_margin: _,
        bar_width: _,
        handle_min_length: _,
        bar_inner_margin: _,
        bar_outer_margin: _,
        floating_width: _,
        floating_allocated_width: _,
        foreground_color: _,
        dormant_background_opacity: _,
        active_background_opacity: _,
        interact_background_opacity: _,
        dormant_handle_opacity: _,
        active_handle_opacity: _,
        interact_handle_opacity: _,
        fade,
    } = scroll;

    // `ScrollFadeStyle`, 2 fields (egui/src/style.rs:788, :792).
    let ScrollFadeStyle {
        strength: _,
        size: _,
    } = fade;

    // `Interaction`, 8 fields (egui/src/style.rs:916-944): `tooltip.show_delay` writes
    // `tooltip_delay`.
    let Interaction {
        interact_radius: _,
        resize_grab_radius_side: _,
        resize_grab_radius_corner: _,
        show_tooltips_only_when_still: _,
        tooltip_delay: _,
        tooltip_grace_time: _,
        selectable_labels: _,
        multi_widget_text_select: _,
    } = interaction;

    // `Visuals`, 36 fields (egui/src/style.rs:989-1130); `clip_rect_margin` is deprecated
    // (:1086), so its pattern carries `#[expect(deprecated)]` — measured to compile clean
    // under `#![deny(warnings)]` on rustc 1.98.1 (2026-09-26).
    let Visuals {
        dark_mode: _,
        text_options,
        override_text_color: _,
        weak_text_alpha: _,
        weak_text_color: _,
        widgets,
        selection,
        ime_composition,
        hyperlink_color: _,
        faint_bg_color: _,
        extreme_bg_color: _,
        text_edit_bg_color: _,
        code_bg_color: _,
        warn_fg_color: _,
        error_fg_color: _,
        window_corner_radius,
        window_shadow,
        window_fill: _,
        window_stroke,
        window_highlight_topmost: _,
        menu_corner_radius: _,
        panel_fill: _,
        popup_shadow: _,
        resize_corner_size: _,
        text_cursor,
        #[expect(deprecated)]
            clip_rect_margin: _,
        button_frame: _,
        collapsing_header_frame: _,
        indent_has_left_vline: _,
        striped: _,
        slider_trailing_fill: _,
        handle_shape: _,
        interact_cursor: _,
        image_loading_spinners: _,
        numeric_color_space: _,
        disabled_alpha: _,
    } = visuals;

    // `TextOptions`, 4 fields (epaint/src/text/mod.rs:29-53).
    let TextOptions {
        max_texture_side: _,
        color_transfer_function: _,
        font_hinting: _,
        subpixel_binning: _,
    } = text_options;

    // `Widgets`, 5 fields (egui/src/style.rs:1255-1269).
    let Widgets {
        noninteractive,
        inactive: _,
        hovered: _,
        active: _,
        open: _,
    } = widgets;

    // `WidgetVisuals`, 6 fields (egui/src/style.rs:1295-1319).
    let WidgetVisuals {
        bg_fill: _,
        weak_bg_fill: _,
        bg_stroke: _,
        corner_radius: _,
        fg_stroke: _,
        expansion: _,
    } = noninteractive;

    // `Selection`, 2 fields (egui/src/style.rs:1196-1199).
    let Selection {
        bg_fill: _,
        stroke: _,
    } = selection;

    // `ImeComposition`, 3 fields (egui/src/style.rs:1208, :1211, :1229).
    let ImeComposition {
        active_underline_stroke: _,
        inactive_underline_stroke: _,
        legacy_visuals: _,
    } = ime_composition;

    // `TextCursorStyle`, 5 fields (egui/src/style.rs:953-965).
    let TextCursorStyle {
        stroke: _,
        preview: _,
        blink: _,
        on_duration: _,
        off_duration: _,
    } = text_cursor;

    // `CornerRadius` (epaint/src/corner_radius.rs:13-25), `Shadow` (epaint/src/shadow.rs:15-26)
    // and `Stroke` (epaint/src/stroke.rs:13-16).
    let CornerRadius {
        nw: _,
        ne: _,
        sw: _,
        se: _,
    } = window_corner_radius;
    let Shadow {
        offset: _,
        blur: _,
        spread: _,
        color: _,
    } = window_shadow;
    let Stroke { width: _, color: _ } = window_stroke;

    // `egui::Frame`, 6 fields (egui/src/containers/frame.rs:104-140): the `Surface` axis's
    // sink, guarded upstream only by a `size_of` test (:143-154).
    let Frame {
        inner_margin: _,
        fill: _,
        stroke: _,
        corner_radius: _,
        outer_margin: _,
        shadow: _,
    } = Frame::default();
}

/// T1b: the three upstream defaults this design reasons **from** (§6.15, §7.5, §7.4).
#[test]
fn the_defaults_this_design_reasons_from_have_not_moved() {
    // egui/src/style.rs:1465
    assert_eq!(Style::default().spacing.extra_text_line_spacing, 0.0);
    // egui/src/style.rs:1460
    assert_eq!(Style::default().spacing.interact_size.x, 40.0);
    // egui/src/style.rs:1560 (the field at :1126)
    assert_eq!(Visuals::dark().disabled_alpha, 0.5);
}

/// What names a `§` citation's document on the line that holds it: `spec §` (`Spec §` at a
/// sentence's start), the connector spec the crate docs name, or a document by its own name.
const CITATION_NAMES: &[&str] = &["spec §", "Spec §", ".md", "CSS Fonts"];

/// M7-06: every `///` or `//!` line holding a `§` in a file docs.rs renders names the
/// document it cites (M6-38's ruling), so a docs.rs reader can look it up. The files are the
/// crate's top-level modules but those `lib.rs` declares under `#[cfg(test)]`; `style/` is a
/// private module docs.rs does not render, and a file's test module, at its end, is left out.
#[test]
fn every_section_citation_in_the_rendered_docs_names_its_document() {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let lib = std::fs::read_to_string(src.join("lib.rs")).unwrap();
    let lib_lines: Vec<&str> = lib.lines().collect();
    let test_only: Vec<&str> = lib_lines
        .windows(2)
        .filter(|pair| pair[0].trim() == "#[cfg(test)]")
        .filter_map(|pair| pair[1].trim().strip_prefix("mod ")?.strip_suffix(';'))
        .collect();
    assert!(
        test_only.contains(&"tripwires"),
        "lib.rs read wrong: {test_only:?}"
    );

    let mut scanned = Vec::new();
    let mut bare = Vec::new();
    for entry in std::fs::read_dir(&src).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let Some(stem) = name.strip_suffix(".rs") else {
            continue;
        };
        if test_only.contains(&stem) {
            continue;
        }
        let text = std::fs::read_to_string(&path).unwrap();
        for (number, line) in rendered_lines(&text) {
            let doc = line.trim_start();
            let is_doc = doc.starts_with("///") || doc.starts_with("//!");
            if is_doc && doc.contains('§') && !CITATION_NAMES.iter().any(|n| doc.contains(n)) {
                bare.push(format!("src/{name}:{number}: {doc}"));
            }
        }
        scanned.push(name);
    }
    assert!(
        scanned.iter().any(|n| n == "atlas.rs"),
        "nothing scanned: {scanned:?}"
    );
    assert!(
        bare.is_empty(),
        "a `§` that names no document (write `spec §N`):\n{}",
        bare.join("\n")
    );
}

/// The lines of `text`, numbered from 1, before its inline `#[cfg(test)]` module: a
/// `#[cfg(test)]` line, any attributes after it, then a `mod name {` line. A `#[cfg(test)]`
/// on anything else (`mod tests;` in `lib.rs`) does not end the scan.
fn rendered_lines(text: &str) -> Vec<(usize, &str)> {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Vec::new();
    for (i, &line) in lines.iter().enumerate() {
        if line.starts_with("#[cfg(test)]") && opens_inline_module(&lines[i + 1..]) {
            break;
        }
        out.push((i + 1, line));
    }
    out
}

/// Whether the item after a `#[cfg(test)]` is an inline `mod name {`, past its attributes,
/// single-line or spread over lines up to one ending in `)]`.
fn opens_inline_module(after: &[&str]) -> bool {
    let mut in_attribute = false;
    for line in after {
        let line = line.trim();
        if in_attribute {
            in_attribute = !line.ends_with(")]");
        } else if line.starts_with("#[") {
            in_attribute = !line.ends_with(']');
        } else {
            return line.starts_with("mod ") && line.ends_with('{');
        }
    }
    false
}
