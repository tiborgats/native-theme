//! Property-based tests for TOML round-trip, Rgba hex parsing, and merge semantics.
//!
//! Uses proptest to generate random theme values and verify algebraic properties
//! that must hold for all inputs, catching edge cases deterministic tests miss.
//!
//! Strategy composition avoids `prop_flat_map` nesting to prevent stack overflow
//! on the deeply nested ThemeVariant type. Instead, we generate flat vectors of
//! random data and map them into the target structs in a single `prop_map` step.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use native_theme::*;
use proptest::prelude::*;

// ── Leaf strategies ─────────────────────────────────────────────────────────

fn arb_rgba() -> impl Strategy<Value = Rgba> {
    (any::<u8>(), any::<u8>(), any::<u8>(), any::<u8>())
        .prop_map(|(r, g, b, a)| Rgba::rgba(r, g, b, a))
}

fn arb_font_style() -> impl Strategy<Value = FontStyle> {
    prop_oneof![
        Just(FontStyle::Normal),
        Just(FontStyle::Italic),
        Just(FontStyle::Oblique),
    ]
}

prop_compose! {
    fn arb_font_spec()(
        family in proptest::option::of("[a-zA-Z ]{1,20}"),
        size_raw in proptest::option::of(1.0f32..200.0),
        use_pt in any::<bool>(),
        weight in proptest::option::of(100u16..900u16),
        style in proptest::option::of(arb_font_style()),
        color in proptest::option::of(arb_rgba()),
    ) -> FontSpec {
        let size = size_raw.map(|v| if use_pt { FontSize::Pt(v) } else { FontSize::Px(v) });
        FontSpec { family, size, weight, style, color }
    }
}

prop_compose! {
    fn arb_border_spec()(
        color in proptest::option::of(arb_rgba()),
        corner_radius in proptest::option::of(0.0f32..100.0),
        corner_radius_lg in proptest::option::of(0.0f32..100.0),
        line_width in proptest::option::of(0.0f32..10.0),
        opacity in proptest::option::of(0.0f32..1.0),
        shadow_enabled in proptest::option::of(any::<bool>()),
        padding_horizontal in proptest::option::of(0.0f32..100.0),
        padding_vertical in proptest::option::of(0.0f32..100.0),
    ) -> BorderSpec {
        BorderSpec {
            color, corner_radius, corner_radius_lg, line_width,
            opacity, shadow_enabled, padding_horizontal, padding_vertical,
        }
    }
}

prop_compose! {
    fn arb_icon_sizes()(
        toolbar in proptest::option::of(8.0f32..128.0),
        small in proptest::option::of(8.0f32..128.0),
        large in proptest::option::of(8.0f32..128.0),
        dialog in proptest::option::of(8.0f32..128.0),
        panel in proptest::option::of(8.0f32..128.0),
    ) -> IconSizes {
        IconSizes { toolbar, small, large, dialog, panel }
    }
}

prop_compose! {
    fn arb_text_scale_entry()(
        size_raw in proptest::option::of(1.0f32..200.0),
        use_pt in any::<bool>(),
        weight in proptest::option::of(100u16..900u16),
        line_height in proptest::option::of(0.5f32..3.0),
    ) -> TextScaleEntry {
        let size = size_raw.map(|v| if use_pt { FontSize::Pt(v) } else { FontSize::Px(v) });
        TextScaleEntry { size, weight, line_height }
    }
}

prop_compose! {
    fn arb_text_scale()(
        caption in proptest::option::of(arb_text_scale_entry()),
        section_heading in proptest::option::of(arb_text_scale_entry()),
        dialog_title in proptest::option::of(arb_text_scale_entry()),
        display in proptest::option::of(arb_text_scale_entry()),
    ) -> TextScale {
        TextScale { caption, section_heading, dialog_title, display }
    }
}

// ── ThemeDefaults strategy ──────────────────────────────────────────────────
//
// Uses a flat color vec to stay within prop_compose! binding limits.

fn arb_theme_defaults() -> impl Strategy<Value = ThemeDefaults> {
    (
        arb_font_spec(),
        arb_font_spec(),
        arb_border_spec(),
        arb_icon_sizes(),
        // 6 scalar Option fields
        proptest::option::of(0.5f32..3.0),  // line_height
        proptest::option::of(0.0f32..1.0),  // disabled_opacity
        proptest::option::of(0.0f32..10.0), // focus_ring_width
        proptest::option::of(0.0f32..10.0), // focus_ring_offset
        proptest::option::of(0.5f32..3.0),  // text_scaling_factor
    )
        .prop_flat_map(|parts| {
            (
                Just(parts),
                proptest::collection::vec(proptest::option::of(arb_rgba()), 24..=24),
                proptest::option::of(any::<bool>()),
                proptest::option::of(any::<bool>()),
                proptest::option::of(any::<bool>()),
                proptest::option::of(48.0f32..288.0), // font_dpi (valid DPI range)
            )
        })
        .prop_map(
            |(
                (
                    font,
                    mono_font,
                    border,
                    icon_sizes,
                    line_height,
                    disabled_opacity,
                    focus_ring_width,
                    focus_ring_offset,
                    text_scaling_factor,
                ),
                colors,
                reduce_motion,
                high_contrast,
                reduce_transparency,
                font_dpi,
            )| {
                ThemeDefaults {
                    font,
                    mono_font,
                    border,
                    icon_sizes,
                    line_height,
                    background_color: colors[0],
                    text_color: colors[1],
                    accent_color: colors[2],
                    accent_text_color: colors[3],
                    surface_color: colors[4],
                    muted_color: colors[5],
                    shadow_color: colors[6],
                    link_color: colors[7],
                    selection_background: colors[8],
                    selection_text_color: colors[9],
                    selection_inactive_background: colors[10],
                    text_selection_background: colors[11],
                    text_selection_color: colors[12],
                    disabled_text_color: colors[13],
                    danger_color: colors[14],
                    danger_text_color: colors[15],
                    warning_color: colors[16],
                    warning_text_color: colors[17],
                    success_color: colors[18],
                    success_text_color: colors[19],
                    info_color: colors[20],
                    info_text_color: colors[21],
                    disabled_opacity,
                    focus_ring_color: colors[22],
                    focus_ring_width,
                    focus_ring_offset,
                    font_dpi,
                    text_scaling_factor,
                    reduce_motion,
                    high_contrast,
                    reduce_transparency,
                }
            },
        )
}

// ── Widget strategies (5 complex widgets with full field coverage) ───────────
//
// Each widget strategy avoids nested prop_flat_map chains. Fields are generated
// as flat tuples in a single layer to keep the proptest value tree shallow.

fn arb_window_theme() -> impl Strategy<Value = WindowTheme> {
    (
        proptest::option::of(arb_rgba()),
        proptest::option::of(arb_rgba()),
        proptest::option::of(arb_rgba()),
        proptest::option::of(arb_rgba()),
        proptest::option::of(arb_font_spec()),
        proptest::option::of(arb_border_spec()),
    )
        .prop_map(
            |(
                background_color,
                title_bar_background,
                inactive_title_bar_background,
                inactive_title_bar_text_color,
                title_bar_font,
                border,
            )| {
                WindowTheme {
                    background_color,
                    title_bar_background,
                    inactive_title_bar_background,
                    inactive_title_bar_text_color,
                    title_bar_font,
                    border,
                }
            },
        )
}

fn arb_button_theme() -> impl Strategy<Value = ButtonTheme> {
    // Generate all 9 color/scalar fields + 4 more colors + font + border
    // in a single flat_map to keep tree depth at 2.
    (
        proptest::collection::vec(proptest::option::of(arb_rgba()), 11..=11),
        proptest::option::of(1.0f32..200.0),
        proptest::option::of(1.0f32..200.0),
        proptest::option::of(0.0f32..50.0),
        proptest::option::of(0.0f32..1.0),
        proptest::option::of(arb_font_spec()),
        proptest::option::of(arb_border_spec()),
    )
        .prop_map(
            |(colors, min_w, min_h, gap, dis_op, font, border)| ButtonTheme {
                background_color: colors[0],
                primary_background: colors[1],
                primary_text_color: colors[2],
                min_width: min_w,
                min_height: min_h,
                icon_text_gap: gap,
                disabled_opacity: dis_op,
                hover_background: colors[3],
                hover_text_color: colors[4],
                active_text_color: colors[5],
                disabled_text_color: colors[6],
                active_background: colors[7],
                disabled_background: colors[8],
                font,
                border,
            },
        )
}

fn arb_input_theme() -> impl Strategy<Value = InputTheme> {
    (
        proptest::collection::vec(proptest::option::of(arb_rgba()), 8..=8),
        proptest::option::of(1.0f32..200.0),
        proptest::option::of(0.0f32..1.0),
        proptest::option::of(arb_font_spec()),
        proptest::option::of(arb_border_spec()),
    )
        .prop_map(|(colors, min_h, dis_op, font, border)| InputTheme {
            background_color: colors[0],
            placeholder_color: colors[1],
            caret_color: colors[2],
            selection_background: colors[3],
            selection_text_color: colors[4],
            min_height: min_h,
            disabled_opacity: dis_op,
            disabled_text_color: colors[5],
            hover_border_color: colors[6],
            focus_border_color: colors[7],
            disabled_background: colors[5],
            font,
            border,
        })
}

fn arb_checkbox_theme() -> impl Strategy<Value = CheckboxTheme> {
    (
        proptest::collection::vec(proptest::option::of(arb_rgba()), 7..=7),
        proptest::option::of(1.0f32..50.0),
        proptest::option::of(0.0f32..50.0),
        proptest::option::of(0.0f32..1.0),
        proptest::option::of(arb_font_spec()),
        proptest::option::of(arb_border_spec()),
    )
        .prop_map(|(colors, ind_w, gap, dis_op, font, border)| CheckboxTheme {
            background_color: colors[0],
            checked_background: colors[1],
            indicator_color: colors[2],
            indicator_width: ind_w,
            label_gap: gap,
            disabled_opacity: dis_op,
            disabled_text_color: colors[3],
            hover_background: colors[4],
            disabled_background: colors[5],
            unchecked_background: colors[6],
            unchecked_border_color: colors[3],
            font,
            border,
        })
}

fn arb_dialog_button_order() -> impl Strategy<Value = DialogButtonOrder> {
    prop_oneof![
        Just(DialogButtonOrder::PrimaryRight),
        Just(DialogButtonOrder::PrimaryLeft),
    ]
}

fn arb_dialog_theme() -> impl Strategy<Value = DialogTheme> {
    (
        proptest::option::of(arb_rgba()),
        proptest::option::of(1.0f32..1000.0),
        proptest::option::of(1.0f32..2000.0),
        proptest::option::of(1.0f32..1000.0),
        proptest::option::of(1.0f32..2000.0),
        proptest::option::of(0.0f32..50.0),
        proptest::option::of(8.0f32..128.0),
        proptest::option::of(arb_dialog_button_order()),
        proptest::option::of(arb_font_spec()),
        proptest::option::of(arb_font_spec()),
        proptest::option::of(arb_border_spec()),
    )
        .prop_map(
            |(
                bg,
                min_w,
                max_w,
                min_h,
                max_h,
                gap,
                icon_sz,
                order,
                title_font,
                body_font,
                border,
            )| {
                DialogTheme {
                    background_color: bg,
                    min_width: min_w,
                    max_width: max_w,
                    min_height: min_h,
                    max_height: max_h,
                    button_gap: gap,
                    icon_size: icon_sz,
                    button_order: order,
                    title_font,
                    body_font,
                    border,
                }
            },
        )
}

// ── IconSet strategy ────────────────────────────────────────────────────────

fn arb_icon_set() -> impl Strategy<Value = IconSet> {
    prop_oneof![
        Just(IconSet::SfSymbols),
        Just(IconSet::SegoeIcons),
        Just(IconSet::Freedesktop),
        Just(IconSet::Material),
        Just(IconSet::Lucide),
    ]
}

// ── LayoutTheme strategy ────────────────────────────────────────────────────

prop_compose! {
    fn arb_layout_theme()(
        widget_gap in proptest::option::of(0.0f32..100.0),
        container_margin in proptest::option::of(0.0f32..100.0),
        window_margin in proptest::option::of(0.0f32..100.0),
        section_gap in proptest::option::of(0.0f32..100.0),
    ) -> LayoutTheme {
        LayoutTheme { widget_gap, container_margin, window_margin, section_gap }
    }
}

// ── ThemeVariant strategy ───────────────────────────────────────────────────

fn arb_theme_variant() -> impl Strategy<Value = ThemeVariant> {
    (
        arb_theme_defaults(),
        arb_text_scale(),
        arb_window_theme(),
        arb_button_theme(),
        arb_input_theme(),
        arb_checkbox_theme(),
        arb_dialog_theme(),
        proptest::option::of(arb_icon_set()),
        proptest::option::of("[a-zA-Z]{1,15}"),
    )
        .prop_map(
            |(
                defaults,
                text_scale,
                window,
                button,
                input,
                checkbox,
                dialog,
                icon_set,
                icon_theme,
            )| {
                ThemeVariant {
                    defaults,
                    text_scale,
                    window,
                    button,
                    input,
                    checkbox,
                    dialog,
                    icon_set,
                    icon_theme,
                    // Remaining widgets use defaults -- exercises their empty round-trip paths
                    menu: MenuTheme::default(),
                    tooltip: TooltipTheme::default(),
                    scrollbar: ScrollbarTheme::default(),
                    slider: SliderTheme::default(),
                    progress_bar: ProgressBarTheme::default(),
                    tab: TabTheme::default(),
                    sidebar: SidebarTheme::default(),
                    toolbar: ToolbarTheme::default(),
                    status_bar: StatusBarTheme::default(),
                    list: ListTheme::default(),
                    popover: PopoverTheme::default(),
                    splitter: SplitterTheme::default(),
                    separator: SeparatorTheme::default(),
                    switch: SwitchTheme::default(),
                    spinner: SpinnerTheme::default(),
                    combo_box: ComboBoxTheme::default(),
                    segmented_control: SegmentedControlTheme::default(),
                    card: CardTheme::default(),
                    expander: ExpanderTheme::default(),
                    link: LinkTheme::default(),
                }
            },
        )
}

// ── ThemeSpec strategy ──────────────────────────────────────────────────────

fn arb_theme_spec() -> impl Strategy<Value = ThemeSpec> {
    // Generate one variant and use booleans to place it in light/dark/both slots.
    // This keeps the proptest value tree shallow enough to avoid stack overflow
    // while still exercising all ThemeSpec serialization paths.
    (
        "[a-zA-Z ]{1,20}",
        arb_theme_variant(),
        any::<bool>(), // has_light
        any::<bool>(), // has_dark
        arb_layout_theme(),
    )
        .prop_map(|(name, variant, has_light, has_dark, layout)| ThemeSpec {
            name,
            light: if has_light {
                Some(variant.clone())
            } else {
                None
            },
            dark: if has_dark { Some(variant) } else { None },
            layout,
        })
}

// ── Property tests: Round-trips ─────────────────────────────────────────────

proptest! {
    #[test]
    fn rgba_hex_round_trip(c in arb_rgba()) {
        let hex = c.to_string();
        let parsed: Rgba = hex.parse().unwrap();
        prop_assert_eq!(c, parsed);
    }
}

proptest! {
    #[test]
    fn font_spec_toml_round_trip(fs in arb_font_spec()) {
        let toml_str = toml::to_string(&fs).unwrap();
        let deserialized: FontSpec = toml::from_str(&toml_str).unwrap();
        prop_assert_eq!(fs, deserialized);
    }
}

proptest! {
    #[test]
    fn border_spec_toml_round_trip(bs in arb_border_spec()) {
        let toml_str = toml::to_string(&bs).unwrap();
        let deserialized: BorderSpec = toml::from_str(&toml_str).unwrap();
        prop_assert_eq!(bs, deserialized);
    }
}

proptest! {
    #[test]
    fn theme_defaults_toml_round_trip(d in arb_theme_defaults()) {
        // Round-trip is idempotent: the first pass may normalize Some(empty) → None
        // (D-2 fix), but a second pass must be stable.
        let toml1 = toml::to_string(&d).unwrap();
        let round1: ThemeDefaults = toml::from_str(&toml1).unwrap();
        let toml2 = toml::to_string(&round1).unwrap();
        let round2: ThemeDefaults = toml::from_str(&toml2).unwrap();
        prop_assert_eq!(round1, round2, "TOML round-trip must be idempotent");
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn theme_variant_toml_round_trip(v in arb_theme_variant()) {
        let mut theme = ThemeSpec::new("Test");
        theme.light = Some(v);
        let toml1 = toml::to_string_pretty(&theme).unwrap();
        let round1: ThemeSpec = toml::from_str(&toml1).unwrap();
        let toml2 = toml::to_string_pretty(&round1).unwrap();
        let round2: ThemeSpec = toml::from_str(&toml2).unwrap();
        prop_assert_eq!(round1, round2, "TOML round-trip must be idempotent");
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn theme_spec_toml_round_trip(theme in arb_theme_spec()) {
        let toml1 = toml::to_string_pretty(&theme).unwrap();
        let round1: ThemeSpec = toml::from_str(&toml1).unwrap();
        let toml2 = toml::to_string_pretty(&round1).unwrap();
        let round2: ThemeSpec = toml::from_str(&toml2).unwrap();
        prop_assert_eq!(round1, round2, "TOML round-trip must be idempotent");
    }
}

// ── Property tests: Merge semantics ─────────────────────────────────────────

// Merge properties tested at ThemeDefaults level (exercises all merge paths:
// option, nested font/border/icon_sizes). ThemeVariant merge delegates to
// ThemeDefaults merge + per-widget merge, which all use the same impl_merge!
// macro -- no additional coverage from testing at ThemeVariant level.

proptest! {
    #[test]
    fn merge_identity(d in arb_theme_defaults()) {
        let original = d.clone();
        let mut result = d;
        result.merge(&ThemeDefaults::default());
        prop_assert_eq!(result, original);
    }
}

proptest! {
    #[test]
    fn merge_idempotent(d in arb_theme_defaults()) {
        let overlay = d.clone();
        let mut result = d.clone();
        result.merge(&overlay);
        prop_assert_eq!(result, d);
    }
}

proptest! {
    #[test]
    fn merge_overlay_dominance_font(base in arb_font_spec(), overlay in arb_font_spec()) {
        let mut result = base;
        result.merge(&overlay);
        if overlay.family.is_some() {
            prop_assert_eq!(result.family, overlay.family);
        }
        if overlay.size.is_some() {
            prop_assert_eq!(result.size, overlay.size);
        }
        if overlay.weight.is_some() {
            prop_assert_eq!(result.weight, overlay.weight);
        }
        if overlay.style.is_some() {
            prop_assert_eq!(result.style, overlay.style);
        }
        if overlay.color.is_some() {
            prop_assert_eq!(result.color, overlay.color);
        }
    }
}

proptest! {
    #[test]
    fn merge_overlay_dominance_defaults(base in arb_theme_defaults(), overlay in arb_theme_defaults()) {
        let mut result = base;
        result.merge(&overlay);
        // Check a representative sample of fields
        if overlay.accent_color.is_some() {
            prop_assert_eq!(result.accent_color, overlay.accent_color);
        }
        if overlay.background_color.is_some() {
            prop_assert_eq!(result.background_color, overlay.background_color);
        }
        if overlay.line_height.is_some() {
            prop_assert_eq!(result.line_height, overlay.line_height);
        }
        if overlay.disabled_opacity.is_some() {
            prop_assert_eq!(result.disabled_opacity, overlay.disabled_opacity);
        }
        // Nested: font overlay dominance
        if overlay.font.family.is_some() {
            prop_assert_eq!(result.font.family, overlay.font.family);
        }
        if overlay.font.size.is_some() {
            prop_assert_eq!(result.font.size, overlay.font.size);
        }
    }
}
