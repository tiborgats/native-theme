use super::*;

use crate::next::geometry::{
    ComplexStroke, CornerCurve, CornerRadius, Corners, ShadowLayer, Stroke, StrokeAlign,
    StrokeSide, Transform,
};
use crate::next::layout::LabelSide;
use crate::next::paint::{BlendMode, ComplexPaint, GradientStop, MaterialKind, Paint, PaintLayer};
use crate::next::patch::{
    ButtonRule, IconPatch, PartPatch, SwitchPatch, TextPatch, WidgetPatchMap,
};
use crate::next::query::{ButtonShape, ButtonVariant, Scheme, ThemeContext, WidgetState};
use crate::next::style::{FocusIndicator, FocusKind, FontWidth};

type TestResult = Result<(), Box<dyn std::error::Error>>;

const RED: Color = Color::rgb(1.0, 0.0, 0.0);
const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);

/// Spec §7's example file, as the spec writes it.
#[cfg(feature = "palettes")]
const SPEC_EXAMPLE: &str = r##"
name = "Neon Glass"
base = "auto"                    # "auto" (Preset::detect(), zero OS I/O) or any Preset::as_str()
accent = "#7b2ff7"               # optional; #rgb, #rgba, #rrggbb, #rrggbbaa, srgb(...), or p3(...)
palette = "catppuccin-mocha"     # optional; bundled name OR inline `[palette]` table (mutually exclusive with `[palettes]`)
# Or dual-scheme:
# [palettes]
# light = "catppuccin-latte"
# dark = "catppuccin-mocha"

[metrics]                        # optional scaling overrides
corner_scale = 1.25
spacing_scale = 1.0

[typography]                     # optional font family overrides
font_family = "Inter"

[colors.window]                  # optional concrete overrides on any ColorRoles field
background = "#0b0d14"

[[widgets.button.rules]]
variant = "primary"
shape = "pill"
states = ["hover"]               # optional `not_states`, `size`, `surface`, `position`, `orientation`
body.fill = { kind = "linear_gradient", angle_deg = 135.0, stops = [{ offset = 0.0, color = "#7b2ff7" }, { offset = 1.0, color = "#00e5ff" }] }
body.shadows = [{ offset_y = 4.0, blur = 18.0, color = "#7b2ff780" }]
body.transform = { scale = [1.03, 1.03] }

[custom_colors]                  # optional app-specific tokens
editor_gutter = "#1a1e2e"
"##;

/// The body patch of the one button rule `lines` state.
fn body(lines: &str) -> Result<PartPatch, Box<dyn std::error::Error>> {
    let patch = ThemePatch::from_toml(&format!("[[widgets.button.rules]]\n{lines}\n"))?;
    let rule = patch.widget_rules.button.rules.first().ok_or("no rule")?;
    Ok(rule.body.clone())
}

/// The path of the `InvalidTheme` error `result` holds.
fn invalid<T: std::fmt::Debug>(result: Result<T, Error>) -> Result<String, String> {
    match result {
        Err(Error::InvalidTheme { path, .. }) => Ok(path),
        other => Err(format!("not InvalidTheme: {other:?}")),
    }
}

fn complex(layers: Vec<PaintLayer>) -> Paint {
    Paint::Complex(Arc::new(ComplexPaint {
        layers: Arc::from(layers),
    }))
}

fn sample_palette() -> Palette {
    Palette {
        name: Arc::from("Sample"),
        scheme: Scheme::Dark,
        window_bg: Color::from_srgb8(0x10, 0x11, 0x12, 0xff),
        view_bg: Color::from_srgb8(0x13, 0x14, 0x15, 0xff),
        surface_bg: Color::from_srgb8(0x16, 0x17, 0x18, 0xff),
        header_bg: Color::from_srgb8(0x19, 0x1a, 0x1b, 0xff),
        sidebar_bg: Color::from_srgb8(0x1c, 0x1d, 0x1e, 0xff),
        tooltip_bg: Color::from_srgb8(0x1f, 0x20, 0x21, 0xff),
        foreground: Color::WHITE,
        foreground_muted: Color::WHITE.with_alpha(0.6),
        border: RED,
        divider: GREEN,
        accent: BLUE,
        accent_fg: Color::WHITE,
        selection_bg: BLUE.with_alpha(0.3),
        selection_fg: Color::WHITE,
        link: BLUE,
        caret: Color::WHITE,
        error: RED,
        warning: Color::rgb(1.0, 1.0, 0.0),
        success: GREEN,
        info: Color::rgb(0.25, 0.5, 0.75),
    }
}

/// A patch that states something of every form a file spells.
fn sample_patch() -> ThemePatch {
    let gradient = complex(vec![
        PaintLayer::RadialGradient {
            center: [0.5, 0.25],
            radius: 0.75,
            stops: Arc::from([
                GradientStop {
                    offset: 0.0,
                    color: RED,
                },
                GradientStop {
                    offset: 1.0,
                    color: BLUE,
                },
            ]),
            blend: BlendMode::Multiply,
        },
        PaintLayer::Solid {
            color: GREEN,
            blend: BlendMode::Normal,
        },
    ]);
    let focus = FocusIndicator::new(
        FocusKind::Ring,
        Stroke::Uniform {
            width: 2.0,
            color: BLUE,
            align: StrokeAlign::Outside,
        },
        Stroke::None,
        1.0,
        None,
        None,
    );
    ThemePatch::new()
        .corner_scale(1.5)
        .spacing_scale(0.75)
        .font_family("Inter")
        .monospace_family("Fira Code")
        .custom_color("gutter", GREEN)
        .custom_metric("rail", 3.0)
        .button(
            ButtonRule::new()
                .variant(ButtonVariant::Primary)
                .shape(ButtonShape::Pill)
                .states(WidgetState::HOVER)
                .not_states(WidgetState::DISABLED)
                .body(
                    PartPatch::new()
                        .fill(gradient)
                        .stroke(Stroke::Uniform {
                            width: 1.0,
                            color: RED,
                            align: StrokeAlign::Inside,
                        })
                        .corners(Corners {
                            radius: CornerRadius::Uniform(6.0),
                            curve: CornerCurve::Continuous,
                        })
                        .shadows(None)
                        .opacity(0.5)
                        .min_size([None, Some(28.0)])
                        .focus(Some(Arc::new(focus))),
                )
                .label(
                    TextPatch::new()
                        .color(BLUE)
                        .optical_size(None)
                        .width(FontWidth::new(3)),
                )
                .icon(IconPatch::new().size(16.0).stroke_width(None)),
        )
}

// The example names bundled palettes.
#[cfg(feature = "palettes")]
#[test]
fn spec_example_reads() -> TestResult {
    let theme = Theme::from_toml(SPEC_EXAMPLE)?;
    let resolved = theme.resolve_current();
    assert_eq!(&*resolved.name, "Neon Glass");
    assert_eq!(resolved.preset, Preset::detect());
    // `catppuccin-mocha` is a dark palette, which locks the scheme.
    assert_eq!(resolved.context.scheme, Scheme::Dark);
    assert_eq!(
        resolved.colors.window.background,
        Some(Color::from_hex("#0b0d14")?)
    );
    assert_eq!(
        resolved.custom_colors.get("editor_gutter"),
        Some(&Color::from_hex("#1a1e2e")?)
    );
    let body = resolved.button.body(
        crate::next::query::ButtonQuery::primary()
            .pill()
            .state(WidgetState::HOVER),
    );
    assert_eq!(
        body.fill,
        complex(vec![PaintLayer::LinearGradient {
            angle_deg: 135.0,
            stops: Arc::from([
                GradientStop {
                    offset: 0.0,
                    color: Color::from_hex("#7b2ff7")?
                },
                GradientStop {
                    offset: 1.0,
                    color: Color::from_hex("#00e5ff")?
                },
            ]),
            blend: BlendMode::Normal,
        }])
    );
    assert_eq!(body.transform.scale, [1.03, 1.03]);
    Ok(())
}

#[cfg(feature = "palettes")]
#[test]
fn spec_example_dual_scheme_reads() -> TestResult {
    let text = SPEC_EXAMPLE
        .replace("palette = \"catppuccin-mocha\"", "")
        .replace("# [palettes]", "[palettes]")
        .replace("# light =", "light =")
        .replace("# dark =", "dark =");
    let theme = Theme::from_toml(&text)?;
    assert_eq!(&*theme.resolve_current().name, "Neon Glass");
    // Each context takes its own scheme's palette, and with it the palette's
    // scheme; the file's `[colors.window]` stands in both.
    let background = Some(Color::from_hex("#0b0d14")?);
    for (ctx, scheme) in [
        (ThemeContext::light(), Scheme::Light),
        (ThemeContext::dark(), Scheme::Dark),
    ] {
        let resolved = theme.resolve(&ctx);
        assert_eq!(resolved.context.scheme, scheme);
        assert_eq!(resolved.colors.window.background, background);
    }
    Ok(())
}

#[test]
fn paint_shorthands() -> TestResult {
    assert_eq!(body("body.fill = \"none\"")?.fill, Some(Paint::None));
    assert_eq!(
        body("body.fill = \"#ff0000\"")?.fill,
        Some(Paint::Solid(RED))
    );
    assert_eq!(
        body("body.fill = { kind = \"solid\", color = \"#00ff00\" }")?.fill,
        Some(complex(vec![PaintLayer::Solid {
            color: GREEN,
            blend: BlendMode::Normal
        }]))
    );
    assert_eq!(
        body("body.fill = [{ kind = \"solid\", color = \"#00ff00\", blend = \"screen\" }, { kind = \"radial_gradient\", center = [0.5, 0.5], radius = 1, stops = [] }]")?.fill,
        Some(complex(vec![
            PaintLayer::Solid { color: GREEN, blend: BlendMode::Screen },
            PaintLayer::RadialGradient {
                center: [0.5, 0.5],
                radius: 1.0,
                stops: Arc::from([]),
                blend: BlendMode::Normal,
            },
        ]))
    );
    let material = body(
        "body.fill = { kind = \"material\", material = \"acrylic\", blur_radius = 30.0, tint = \"#0000ff\", saturation = 1.25, luminosity_opacity = 0.5, noise_opacity = 0.25, fallback = \"#ff0000\" }",
    )?
    .fill
    .ok_or("no fill")?;
    let Paint::Complex(complex) = material else {
        return Err("not a layered paint".into());
    };
    let [PaintLayer::Material(spec)] = &*complex.layers else {
        return Err("not one material layer".into());
    };
    assert_eq!(spec.kind, MaterialKind::Acrylic);
    assert_eq!(spec.fallback, RED);
    Ok(())
}

#[test]
fn stroke_shorthands() -> TestResult {
    assert_eq!(body("body.stroke = \"none\"")?.stroke, Some(Stroke::None));
    assert_eq!(
        body("body.stroke = { width = 1, color = \"#ff0000\" }")?.stroke,
        Some(Stroke::Uniform {
            width: 1.0,
            color: RED,
            align: StrokeAlign::Inside
        })
    );
    assert_eq!(
        body("body.stroke = { width = 2.0, color = \"#ff0000\", align = \"center\" }")?.stroke,
        Some(Stroke::Uniform {
            width: 2.0,
            color: RED,
            align: StrokeAlign::Center
        })
    );
    let side = |width: f32, paint: Paint| StrokeSide { width, paint };
    assert_eq!(
        body(
            "body.stroke = { top = { width = 1.0, paint = \"#ff0000\" }, right = { width = 0.0, paint = \"none\" }, bottom = { width = 2.0, paint = \"#00ff00\" }, left = { width = 0.0, paint = \"none\" }, align = \"outside\" }"
        )?
        .stroke,
        Some(Stroke::Complex(Arc::new(ComplexStroke {
            top: side(1.0, Paint::Solid(RED)),
            right: side(0.0, Paint::None),
            bottom: side(2.0, Paint::Solid(GREEN)),
            left: side(0.0, Paint::None),
            align: StrokeAlign::Outside,
        })))
    );
    assert!(body("body.stroke = { width = 1.0 }").is_err());
    assert!(body("body.stroke = \"thin\"").is_err());
    Ok(())
}

#[test]
fn corners_shorthands() -> TestResult {
    let circular = |radius| Corners {
        radius,
        curve: CornerCurve::Circular,
    };
    assert_eq!(
        body("body.corners = \"none\"")?.corners,
        Some(circular(CornerRadius::None))
    );
    assert_eq!(
        body("body.corners = \"pill\"")?.corners,
        Some(circular(CornerRadius::Pill))
    );
    assert_eq!(
        body("body.corners = 4")?.corners,
        Some(circular(CornerRadius::Uniform(4.0)))
    );
    assert_eq!(
        body("body.corners = 4.5")?.corners,
        Some(circular(CornerRadius::Uniform(4.5)))
    );
    assert_eq!(
        body("body.corners = { radius = 8.0, curve = \"continuous\" }")?.corners,
        Some(Corners {
            radius: CornerRadius::Uniform(8.0),
            curve: CornerCurve::Continuous
        })
    );
    assert_eq!(
        body("body.corners = { radius = { top_left = 1, top_right = 2, bottom_right = 3, bottom_left = 4 } }")?.corners,
        Some(circular(CornerRadius::PerCorner {
            top_left: 1.0,
            top_right: 2.0,
            bottom_right: 3.0,
            bottom_left: 4.0,
        }))
    );
    assert_eq!(
        body("body.corners = { radius = { inset = 2.0, min_radius = 4.0 } }")?.corners,
        Some(circular(CornerRadius::Concentric {
            inset: 2.0,
            min_radius: 4.0
        }))
    );
    assert!(body("body.corners = { radius = { top_left = 1 } }").is_err());
    Ok(())
}

#[test]
fn shadow_and_transform_defaults() -> TestResult {
    let patch = body("body.shadows = [{ blur = 3.0 }]\nbody.transform = { rotation_deg = 90.0 }")?;
    assert_eq!(
        patch.shadows,
        Some(Some(Arc::from([ShadowLayer {
            blur: 3.0,
            ..ShadowLayer::default()
        }])))
    );
    assert_eq!(
        patch.transform,
        Some(Transform {
            rotation_deg: 90.0,
            ..Transform::default()
        })
    );
    Ok(())
}

#[test]
fn none_spellings() -> TestResult {
    let patch =
        body("body.shadows = \"none\"\nbody.min_size = [10, \"none\"]\nbody.focus = \"none\"")?;
    assert_eq!(patch.shadows, Some(None));
    assert_eq!(patch.min_size, Some([Some(10.0), None]));
    assert_eq!(patch.focus, Some(None));
    Ok(())
}

#[test]
fn widget_tables() -> TestResult {
    let patch = ThemePatch::from_toml(
        "[widgets.spinner]\ncolor = \"#ffffff\"\n\n[widgets.tooltip]\npanel.fill = \"#202020\"\n\n[widgets.button]\nicon_text_gap = 6.0\n",
    )?;
    assert_eq!(patch.widget_rules.spinner.color, Some(Color::WHITE));
    assert_eq!(
        patch.widget_rules.tooltip.panel.fill,
        Some(Paint::Solid(Color::from_hex("#202020")?))
    );
    assert_eq!(patch.widget_rules.button.icon_text_gap, Some(6.0));
    Ok(())
}

#[test]
fn switch_label_placement_reads_and_round_trips() -> TestResult {
    let patch = ThemePatch::from_toml(
        "[widgets.switch]\nlabel_side = \"leading\"\nlabel_gap_stretches = true\n",
    )?;
    assert_eq!(
        patch.widget_rules.switch.label_side,
        Some(LabelSide::Leading)
    );
    assert_eq!(patch.widget_rules.switch.label_gap_stretches, Some(true));
    let expected = ThemePatch::new().widget_rules(
        WidgetPatchMap::new().switch(
            SwitchPatch::new()
                .label_side(LabelSide::Leading)
                .label_gap_stretches(true),
        ),
    );
    assert_eq!(patch, expected);
    let trailing = ThemePatch::new().widget_rules(
        WidgetPatchMap::new().switch(
            SwitchPatch::new()
                .label_side(LabelSide::Trailing)
                .label_gap_stretches(false),
        ),
    );
    for patch in [expected, trailing] {
        let text = toml::to_string(&patch)?;
        assert_eq!(ThemePatch::from_toml(&text)?, patch, "{text}");
    }
    Ok(())
}

#[test]
fn unknown_keys_are_invalid() -> TestResult {
    invalid(ThemePatch::from_toml("bogus = 1"))?;
    invalid(ThemePatch::from_toml("[metrics]\ncorner_radius = 1.0"))?;
    invalid(ThemePatch::from_toml("[[widgets.button.rules]]\nbogus = 1"))?;
    invalid(ThemePatch::from_toml(
        "[[widgets.button.rules]]\nbody.filll = \"none\"",
    ))?;
    invalid(ThemePatch::from_toml("[widgets.knob]\ncolor = \"#ffffff\""))?;
    invalid(ThemePatch::from_toml(
        "[[widgets.button.rules]]\nstates = [\"hovered\"]",
    ))?;
    // A theme file's own keys are not a patch's.
    invalid(ThemePatch::from_toml("name = \"x\""))?;
    invalid(Theme::from_toml(
        "name = \"x\"\nbase = \"adwaita\"\nbogus = 1",
    ))?;
    invalid(Palette::from_toml("name = \"x\""))?;
    let path = invalid(ThemePatch::from_toml("[metrics]\n  corner_radius = 1.0"))?;
    assert_eq!(path, "line 2, column 3");
    Ok(())
}

#[test]
fn selectors_on_static_widgets_are_invalid() -> TestResult {
    invalid(ThemePatch::from_toml(
        "[[widgets.tooltip.rules]]\nstates = [\"hover\"]\npanel.fill = \"#202020\"",
    ))?;
    invalid(ThemePatch::from_toml(
        "[widgets.dialog]\nstates = [\"hover\"]",
    ))?;
    invalid(ThemePatch::from_toml(
        "[widgets.popover]\nvariant = \"primary\"",
    ))?;
    Ok(())
}

#[test]
fn not_toml_is_a_toml_error() {
    assert!(matches!(
        ThemePatch::from_toml("[metrics"),
        Err(Error::Toml(_))
    ));
    assert!(matches!(Theme::from_toml("name = "), Err(Error::Toml(_))));
    assert!(matches!(Palette::from_toml("= 1"), Err(Error::Toml(_))));
    assert!(matches!(
        Palette::from_toml("= 1").map_err(|err| err.kind()),
        Err(crate::next::ErrorKind::Parse)
    ));
}

#[test]
fn unknown_names() {
    assert!(matches!(
        Theme::from_toml("name = \"x\"\nbase = \"beos\""),
        Err(Error::UnknownPreset { name }) if name == "beos"
    ));
    let unknown_palette = Theme::from_toml("name = \"x\"\nbase = \"adwaita\"\npalette = \"nope\"");
    if cfg!(feature = "palettes") {
        assert!(matches!(unknown_palette, Err(Error::UnknownPalette { name }) if name == "nope"));
    } else {
        assert!(matches!(
            unknown_palette,
            Err(Error::FeatureDisabled {
                feature: "palettes",
                ..
            })
        ));
    }
}

#[cfg(not(feature = "palettes"))]
#[test]
fn bundled_names_need_the_palettes_feature() {
    assert!(matches!(
        Theme::from_toml("name = \"x\"\nbase = \"adwaita\"\npalette = \"nord\""),
        Err(Error::FeatureDisabled {
            feature: "palettes",
            ..
        })
    ));
}

#[test]
fn palette_and_palettes_are_exclusive() -> TestResult {
    let path = invalid(Theme::from_toml(
        "name = \"x\"\nbase = \"adwaita\"\npalette = \"nord\"\n[palettes]\nlight = \"nord\"\ndark = \"nord\"",
    ))?;
    assert_eq!(path, "palettes");
    Ok(())
}

#[test]
fn values_failing_the_strict_check_are_invalid() -> TestResult {
    let theme =
        |section: &str| Theme::from_toml(&format!("name = \"x\"\nbase = \"adwaita\"\n{section}"));
    assert_eq!(
        invalid(ThemePatch::from_toml("[metrics]\ncorner_scale = nan"))?,
        "metrics.corner_scale"
    );
    assert_eq!(
        invalid(theme("[metrics]\nspacing_scale = inf"))?,
        "metrics.spacing_scale"
    );
    assert_eq!(
        invalid(theme("[metrics]\ncorner_scale = -1.0"))?,
        "metrics.corner_scale"
    );
    assert_eq!(
        invalid(ThemePatch::from_toml(
            "[[widgets.button.rules]]\nbody.corners = -2.0"
        ))?,
        "widgets.button.rules[0].body.corners.radius"
    );
    assert_eq!(
        invalid(ThemePatch::from_toml(
            "[[widgets.button.rules]]\nbody.stroke = { width = -1.0, color = \"#ff0000\" }"
        ))?,
        "widgets.button.rules[0].body.stroke.width"
    );
    assert_eq!(
        invalid(ThemePatch::from_toml(
            "[[widgets.button.rules]]\nbody.shadows = [{ blur = -4.0 }]"
        ))?,
        "widgets.button.rules[0].body.shadows[0].blur"
    );
    assert_eq!(
        invalid(ThemePatch::from_toml(
            "[[widgets.button.rules]]\nbody.opacity = 1.5"
        ))?,
        "widgets.button.rules[0].body.opacity"
    );
    assert_eq!(
        invalid(ThemePatch::from_toml(
            "[widgets.tooltip]\npanel.opacity = -0.5"
        ))?,
        "widgets.tooltip.panel.opacity"
    );
    assert_eq!(
        invalid(theme(
            "accent = \"srgb(1, 0, 0, 1)\"\n[custom_metrics]\nrail = nan"
        ))?,
        "custom_metrics.rail"
    );
    let mut palette = toml::to_string(&sample_palette())?;
    palette = palette.replace("caret = \"#ffffff\"", "caret = \"srgb(inf, 0, 0, 1)\"");
    invalid(Palette::from_toml(&palette))?;
    Ok(())
}

#[test]
fn inline_palettes() -> TestResult {
    let inline = toml::to_string(&sample_palette())?;
    let theme = Theme::from_toml(&format!(
        "name = \"x\"\nbase = \"adwaita\"\n[palette]\n{inline}"
    ))?;
    assert_eq!(theme.resolve_current().context.scheme, Scheme::Dark);
    let light_palette = Palette {
        name: Arc::from("Sample Light"),
        scheme: Scheme::Light,
        window_bg: Color::from_srgb8(0xf0, 0xf1, 0xf2, 0xff),
        ..sample_palette()
    };
    let light = toml::to_string(&light_palette)?;
    let pair = Theme::from_toml(&format!(
        "name = \"x\"\nbase = \"kde-breeze\"\n[palettes.light]\n{light}\n[palettes.dark]\n{inline}"
    ))?;
    // Each context resolves in its palette's scheme, with that palette's
    // window background.
    assert_ne!(light_palette.window_bg, sample_palette().window_bg);
    for (ctx, palette) in [
        (ThemeContext::light(), &light_palette),
        (ThemeContext::dark(), &sample_palette()),
    ] {
        let resolved = pair.resolve(&ctx);
        assert_eq!(resolved.context.scheme, palette.scheme);
        assert_eq!(resolved.colors.window.background, Some(palette.window_bg));
    }
    Ok(())
}

#[test]
fn theme_file_palettes_and_accent_are_checked() -> TestResult {
    let reason = |result: Result<Theme, Error>| match result {
        Err(Error::InvalidTheme { reason, .. }) => Ok(reason),
        other => Err(format!("not InvalidTheme: {other:?}")),
    };
    let missing = reason(Theme::from_toml(
        "name = \"x\"\nbase = \"kde-breeze\"\n[palettes]\nlight = \"nord\"",
    ))?;
    assert!(missing.contains("missing field `dark`"), "{missing}");
    let accent = reason(Theme::from_toml(
        "name = \"x\"\nbase = \"kde-breeze\"\naccent = \"not a colour\"",
    ))?;
    assert!(accent.contains("not a colour"), "{accent}");
    // A colour that is not finite does not parse.
    invalid(Theme::from_toml(
        "name = \"x\"\nbase = \"kde-breeze\"\naccent = \"srgb(inf, 0, 0, 1)\"",
    ))?;
    let inline = toml::to_string(&sample_palette())?
        .replace("caret = \"#ffffff\"", "caret = \"srgb(1, inf, 1, 1)\"");
    invalid(Theme::from_toml(&format!(
        "name = \"x\"\nbase = \"kde-breeze\"\n[palette]\n{inline}"
    )))?;
    let partial = reason(Theme::from_toml(
        "name = \"x\"\nbase = \"kde-breeze\"\n[palette]\nname = \"p\"\nscheme = \"dark\"",
    ))?;
    assert!(partial.contains("missing field"), "{partial}");
    Ok(())
}

#[test]
fn palette_round_trips() -> TestResult {
    let palette = sample_palette();
    let text = toml::to_string(&palette)?;
    assert_eq!(Palette::from_toml(&text)?, palette, "{text}");
    let json = serde_json::to_string(&palette)?;
    assert_eq!(serde_json::from_str::<Palette>(&json)?, palette, "{json}");
    for name in Palette::BUNDLED_NAMES {
        let bundled = Palette::from_name(name).ok_or("a bundled name without its palette")?;
        assert_eq!(
            Palette::from_toml(&toml::to_string(&bundled)?)?,
            bundled,
            "{name}"
        );
        assert_eq!(
            serde_json::from_str::<Palette>(&serde_json::to_string(&bundled)?)?,
            bundled
        );
    }
    Ok(())
}

#[test]
fn patch_round_trips() -> TestResult {
    let patch = sample_patch();
    let text = toml::to_string(&patch)?;
    assert_eq!(ThemePatch::from_toml(&text)?, patch, "{text}");
    let json = serde_json::to_string(&patch)?;
    assert_eq!(serde_json::from_str::<ThemePatch>(&json)?, patch, "{json}");
    let text = toml::to_string(&ThemePatch::new())?;
    assert_eq!(ThemePatch::from_toml(&text)?, ThemePatch::new());
    Ok(())
}
