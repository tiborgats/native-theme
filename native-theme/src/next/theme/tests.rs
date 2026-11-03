//! S16's `Theme` over the bundled presets, and S19.1's build test. The
//! mutations' own tests are the stack's (`resolve/mutation_tests.rs`).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::{preset_stack, synthetic_stack};
use crate::next::blocks::text::ResolvedFont;
use crate::next::declare::DefaultSize;
use crate::next::error::{LayerError, ProblemKind, ThemeError};
use crate::next::gate::synthetic_layers;
use crate::next::layer::Layer;
use crate::next::presets::PresetId;
use crate::next::query::{Element, FontMetrics, RootInputs, Styled, Unavailable};
use crate::next::resolve::{WindowConditions, WindowState};
use crate::next::selector::{ControlSize, Direction, InputKind, PartRef, Scheme};
use crate::next::toml::layer::read_preset;
use crate::next::units::{Ms, Px, Size};
use crate::next::widgets::{ButtonPart, TypedPart};

/// S19.1's build test: every bundled preset reads, states its own key as
/// its `id`, and validates, so `Theme::from_preset` never meets its error
/// in a release (S16): through `from_preset` where its `[pending]` table is
/// empty, else through `from_preset_synthetic` with one synthetic layer per
/// listed path, all at once (S17, ADD-A128). `PresetId::all` lists every
/// variant, as the one list in `presets` writes it. One scheme is enough:
/// the stack's validation covers every scheme whichever is in force (S12).
#[test]
fn every_bundled_preset_validates() {
    for &id in PresetId::all() {
        let file = read_preset(id.source()).unwrap_or_else(|e| panic!("{}: {e}", id.key()));
        assert_eq!(file.id, id.key());
        assert_eq!(PresetId::from_key(id.key()), Some(id));
        validates(Some(id), id.source()).unwrap_or_else(|e| panic!("{}: {e}", id.key()));
    }
}

/// The build test's route for the preset `text` states, as `Theme` reads
/// the bundled preset `id`: the plain one (`from_preset`'s) where its
/// `[pending]` table is empty, else the synthetic one
/// (`from_preset_synthetic`'s) with one synthetic layer per listed path.
fn validates(id: Option<PresetId>, text: &str) -> Result<(), String> {
    let file = read_preset(text).map_err(|e| e.to_string())?;
    if file.pending.is_empty() {
        preset_stack(id, text, Scheme::Light).map_err(|e| e.to_string())?;
    } else {
        let layers = synthetic_layers(&file.pending)?;
        synthetic_stack(id, text, Scheme::Light, layers).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// The build test's two routes, over the fixture preset with a listed
/// path (no preset is bundled with one): the build test takes the
/// synthetic route for it, whose layers state the listed path, so it
/// validates, where the plain route refuses it; and the plain route for
/// the complete base.
#[test]
fn build_test_takes_both_routes() {
    let text = listed_preset();
    assert!(!read_preset(&text).unwrap().pending.is_empty());
    validates(None, &text).unwrap();
    assert!(preset_stack(None, &text, Scheme::Light).is_err());
    let complete = include_str!("../../../tests/fixtures/facts_gate/base.toml");
    assert!(read_preset(complete).unwrap().pending.is_empty());
    validates(None, complete).unwrap();
}

/// Two listed paths, a set-whole block (`Option<KeyRepeat>`) and a leaf
/// the focus indicator's table, built whole, leaves out (S6.1, S17,
/// ADD-A128): the reader leaves the indicator a patch, the Checker exempts
/// both, the synthetic layers, validated all at once, state them, and each
/// resolves to its synthetic value; one layer alone leaves the other
/// missing.
#[test]
fn synthetic_route_takes_every_layer_at_once() {
    let base = include_str!("../../../tests/fixtures/facts_gate/base.toml");
    let text = format!(
        "[pending]\n\"tokens.behavior.key_repeat\" = \"U1\"\n\"tokens.focus.width_px\" = \"U2\"\n{}",
        base.replacen("\nkey_repeat = \"none\"\n", "\n", 1)
            .replacen("\nwidth_px = 2\n", "\n", 1)
    );
    let file = read_preset(&text).unwrap();
    let layers = synthetic_layers(&file.pending).unwrap();
    assert_eq!(layers.len(), 2);
    let stack = synthetic_stack(None, &text, Scheme::Light, layers.clone()).unwrap();
    let theme = stack
        .resolve(
            &mut WindowState::default(),
            &WindowConditions::new(InputKind::Mouse),
        )
        .theme;
    let tokens = theme.tokens(&theme.root_scope(&root())).unwrap();
    let repeat = tokens.behavior.key_repeat.unwrap();
    assert_eq!(
        (repeat.delay, repeat.interval),
        (Some(Ms(1.0)), Some(Ms(1.0)))
    );
    assert_eq!(tokens.focus.width, Some(Px(1.0)));
    for (alone, other) in [(0, "tokens.focus.width"), (1, "tokens.behavior.key_repeat")] {
        let Err(ThemeError::Preset(LayerError::Invalid(problems))) =
            synthetic_stack(None, &text, Scheme::Light, vec![layers[alone].clone()])
        else {
            panic!("one layer alone leaves the other listed path missing");
        };
        assert_eq!(
            problems.iter().map(ToString::to_string).collect::<Vec<_>>(),
            [format!("{other}: missing")]
        );
    }
}

/// Synthetic measurement: every metric zero.
struct NoMetrics;

impl FontMetrics for NoMetrics {
    fn char_width(&self, _: &ResolvedFont) -> Px {
        Px(0.0)
    }
    fn max_char_width(&self, _: &ResolvedFont) -> Px {
        Px(0.0)
    }
    fn x_height(&self, _: &ResolvedFont) -> Px {
        Px(0.0)
    }
    fn height(&self, _: &ResolvedFont) -> Px {
        Px(0.0)
    }
    fn line_spacing(&self, _: &ResolvedFont) -> Px {
        Px(0.0)
    }
    fn underline_offset(&self, _: &ResolvedFont) -> Px {
        Px(0.0)
    }
    fn underline_thickness(&self, _: &ResolvedFont) -> Px {
        Px(0.0)
    }
    fn strikeout_offset(&self, _: &ResolvedFont) -> Px {
        Px(0.0)
    }
    fn strikeout_thickness(&self, _: &ResolvedFont) -> Px {
        Px(0.0)
    }
}

/// A pending default size (S10's *A pending default*, S17, ADD-A131):
/// listed by its declaration's path, it is S12's missing default size on
/// the plain route; the synthetic route states no layer for it, waives the
/// checks that read it, and answers an element that states no size and
/// whose default is pending `NotAvailable(Size)`.
#[test]
fn synthetic_route_waives_a_pending_default_size() {
    let base = include_str!("../../../tests/fixtures/facts_gate/base.toml");
    let text = format!(
        "[pending]\n\"declare.widget.button.default_size\" = \"U1\"\n{base}\n\
         [declare.widget.button]\nsizes = [\"small\", \"regular\"]\n"
    );
    let Err(ThemeError::Preset(LayerError::Invalid(problems))) =
        preset_stack(None, &text, Scheme::Light)
    else {
        panic!("the plain route refuses a pending default size");
    };
    assert_eq!(
        problems.iter().map(ToString::to_string).collect::<Vec<_>>(),
        ["declare.widget.button.default_size: no default size where sizes are taken"]
    );
    let file = read_preset(&text).unwrap();
    let layers = synthetic_layers(&file.pending).unwrap();
    assert!(layers.is_empty());
    let stack = synthetic_stack(None, &text, Scheme::Light, layers).unwrap();
    let theme = stack
        .resolve(
            &mut WindowState::default(),
            &WindowConditions::new(InputKind::Mouse),
        )
        .theme;
    let scope = theme.root_scope(&root());
    let body = PartRef::Typed(TypedPart::Button(ButtonPart::Body));
    let element = Element::new(body.clone(), &scope, &NoMetrics);
    assert_eq!(
        theme.style(&element),
        Styled::NotAvailable(Unavailable::Size)
    );
    let sized = Element::new(body, &scope, &NoMetrics).with_size(ControlSize::Small);
    assert!(matches!(theme.style(&sized), Styled::Style(_)));
}

/// S12 validates a listed path's synthetic value as stated (S17,
/// ADD-A131): `text_scale` read where the listed `text.scaling` is stated
/// `linear` is accepted, as the preset stating it is, and refused where
/// the layer states `dynamic_type`, never read as each variant of its type.
#[test]
fn a_listed_enum_is_validated_as_stated() {
    let base = include_str!("../../../tests/fixtures/facts_gate/base.toml");
    let line = "\nscaling = \"linear\"\n";
    assert!(base.contains(line));
    let rule = "\n[[rules]]\ntarget = \"tokens\"\nset = { text.keyed.body.letter_spacing = \"= mul(1px, text_scale)\" }\n";
    preset_stack(None, &format!("{base}{rule}"), Scheme::Light).unwrap();
    let text = format!(
        "[pending]\n\"tokens.text.scaling\" = \"U1\"\n{}{rule}",
        base.replacen(line, "\n", 1)
    );
    let linear = synthetic_layer("[tokens.text]\nscaling = \"linear\"\n");
    synthetic_stack(None, &text, Scheme::Light, vec![linear]).unwrap();
    let dynamic = synthetic_layer("[tokens.text]\nscaling = \"dynamic_type\"\n");
    let Err(ThemeError::Preset(LayerError::Invalid(problems))) =
        synthetic_stack(None, &text, Scheme::Light, vec![dynamic])
    else {
        panic!("text_scale is refused where the listed scaling is dynamic_type");
    };
    assert!(
        problems
            .iter()
            .any(|problem| problem.kind == ProblemKind::TextScaleUnderDynamicType),
        "{problems:?}"
    );
}

/// S12 validates a listed block S6.1 sets whole as stated (S17,
/// ADD-A131): the listed `key_repeat` stated as a block, its delay read
/// where a duration may not be absent is accepted, as the preset stating
/// it is, never read as `none` beside the stated block.
#[test]
fn a_listed_option_block_is_validated_as_stated() {
    let base = include_str!("../../../tests/fixtures/facts_gate/base.toml");
    let line = "\nkey_repeat = \"none\"\n";
    assert!(base.contains(line));
    let rule = "\n[[rules]]\ntarget = \"tokens\"\nset = { caret.blink_on_ms = \"= token.behavior.key_repeat.delay\" }\n";
    let stated = base.replacen(
        line,
        "\nkey_repeat = { delay_ms = 500, interval_ms = 40 }\n",
        1,
    );
    preset_stack(None, &format!("{stated}{rule}"), Scheme::Light).unwrap();
    let text = format!(
        "[pending]\n\"tokens.behavior.key_repeat\" = \"U1\"\n{}{rule}",
        base.replacen(line, "\n", 1)
    );
    let block = synthetic_layer("[tokens.behavior.key_repeat]\ndelay_ms = 1\ninterval_ms = 1\n");
    synthetic_stack(None, &text, Scheme::Light, vec![block]).unwrap();
}

/// A window's root inputs, synthetic.
fn root() -> RootInputs {
    RootInputs::new(
        Size {
            width: Px(800.0),
            height: Px(600.0),
        },
        Size {
            width: Px(1920.0),
            height: Px(1080.0),
        },
        Direction::Ltr,
    )
}

/// A preset that does not read, or fails validation, is
/// `ThemeError::Preset` with the preset's problems (S1, S16).
#[test]
fn invalid_preset_is_a_theme_error() {
    let unread = preset_stack(None, "[preset]\nid = 1\n", Scheme::Light);
    assert!(matches!(
        unread,
        Err(ThemeError::Preset(LayerError::Invalid(_)))
    ));
    let incomplete = preset_stack(
        None,
        "[preset]\nid = \"probe\"\ngeneration = \"probe-1\"\n",
        Scheme::Dark,
    );
    assert!(matches!(
        incomplete,
        Err(ThemeError::Preset(LayerError::Invalid(_)))
    ));
}

/// The facts gate fixtures' base preset (every field stated, every value
/// synthetic), its touch delay listed in `[pending]` as a register id of
/// the fixture's (S17, ADD-A128), and the line stating it left out.
fn listed_preset() -> String {
    let base = include_str!("../../../tests/fixtures/facts_gate/base.toml");
    let line = "\ndelay_ms = \"absent\"\n";
    assert!(base.contains(line));
    format!(
        "[pending]\n\"tokens.behavior.touch.delay_ms\" = \"U1\"\n{}",
        base.replacen(line, "\n", 1)
    )
}

/// A layer stating the listed path, its value synthetic.
fn synthetic_layer(text: &str) -> Layer {
    Layer::from_toml(text).unwrap()
}

/// The reader reads `[pending]` with its register ids (S11), one or a list
/// of distinct ones, and leaves a listed path out of a block built whole;
/// it reads an input's (S17: a listed path is any path the preset leaves
/// unstated); it refuses an entry whose id is not `U<n>` or whose list is
/// empty or repeats one, or whose path names no field of `colors`, `tokens`
/// or a typed widget and no input (a rule's key, a field the model lacks,
/// an input's name that does not read) or is
/// spelled with a unit suffix its field's type does not take (S17,
/// ADD-A128). A path the preset states too it reads: the facts gate fails
/// that entry.
#[test]
fn pending_table_read() {
    let file = read_preset(&listed_preset()).unwrap();
    assert_eq!(
        file.pending,
        [(
            "tokens.behavior.touch.delay_ms".to_owned(),
            vec!["U1".to_owned()]
        )]
    );
    let base = include_str!("../../../tests/fixtures/facts_gate/base.toml");
    let refused = |table: &str| match read_preset(&format!("[pending]\n{table}\n{base}")) {
        Err(LayerError::Invalid(problems)) => {
            problems.iter().map(ToString::to_string).collect::<Vec<_>>()
        }
        other => panic!("{table}: {other:?}"),
    };
    let not_an_id = [
        "pending.\"tokens.behavior.drag_time_ms\": expected a register id, `U<n>`, \
                      or a list of distinct ones",
    ];
    for ids in [
        "\"151\"",
        "151",
        "\"U\"",
        "\"U12a\"",
        "[]",
        "[\"U1\", \"U1\"]",
        "[\"U1\", 2]",
        "[\"U1\", \"V2\"]",
    ] {
        assert_eq!(
            refused(&format!("\"tokens.behavior.drag_time_ms\" = {ids}")),
            not_an_id,
            "{ids}"
        );
    }
    let ids = read_preset(&format!(
        "[pending]\n\"tokens.behavior.key_repeat\" = [\"U1\", \"U2\"]\n{}",
        base.replacen("\nkey_repeat = \"none\"\n", "\n", 1)
    ))
    .unwrap();
    assert_eq!(
        ids.pending,
        [(
            "tokens.behavior.key_repeat".to_owned(),
            vec!["U1".to_owned(), "U2".to_owned()]
        )]
    );
    for path in [
        "tokens.behavior.bogus_ms",
        "tokens.behavior.touch.delay_px",
        "tokens.behavior.touch.delay",
        "inputs.Accent",
        "rules[0].target",
        "widgets.button.bogus.fill",
    ] {
        assert_eq!(
            refused(&format!("\"{path}\" = \"U1\"")),
            [format!("pending.\"{path}\": unknown key")]
        );
    }
    let stated = read_preset(&format!(
        "[pending]\n\"tokens.behavior.touch.delay_ms\" = \"U1\"\n{base}"
    ))
    .unwrap();
    assert_eq!(stated.pending.len(), 1);
    for path in [
        "colors.light.global.accent",
        "widgets.button.values.icon_text_gap_px",
        "widgets.button.label.text.font.size",
        "widgets.button.label.text.font.size_pt",
        "widgets.button.body.corners.top_left",
        "tokens.motion.durations.backdrop_ms",
    ] {
        assert!(
            crate::next::toml::read::listed_field(path).is_some(),
            "{path}"
        );
    }
    let input = read_preset(&format!("[pending]\n\"inputs.accent\" = \"U1\"\n{base}")).unwrap();
    assert_eq!(input.pending.len(), 1);
}

/// A pending default size is listed by its declaration's path (S11, S17,
/// ADD-A131): the widget's, a `sizes_by_variant` entry's by its variant,
/// `@` included, a `sizes_by_context` entry's by its position, a
/// flavour's under `declare.flavor.<f>.`; the reader sets each pending
/// with its ids, and refuses a path naming no section or entry the
/// declarations state, or no default size.
#[test]
fn pending_default_sizes_read() {
    let base = include_str!("../../../tests/fixtures/facts_gate/base.toml");
    let declare = "[declare.widget.button]\nvariants = [\"@gray\"]\nsizes = [\"regular\"]\n\
                   sizes_by_variant = [{ variant = \"@gray\", sizes = [\"small\"] }]\n\
                   sizes_by_context = [{ context = { widget = \"button\" }, sizes = [\"regular\"] }]\n\
                   [declare.flavor.kirigami.widget.button]\nsizes = [\"small\"]\n";
    let read = |pending: &str| read_preset(&format!("[pending]\n{pending}\n{base}\n{declare}"));
    let file = read(
        "\"declare.widget.button.default_size\" = \"U1\"\n\
         \"declare.widget.button.sizes_by_variant.@gray.default_size\" = [\"U2\", \"U3\"]\n\
         \"declare.widget.button.sizes_by_context.0.default_size\" = \"U4\"\n\
         \"declare.flavor.kirigami.widget.button.default_size\" = \"U5\"",
    )
    .unwrap();
    let pending = |ids: &[&str]| {
        Some(DefaultSize::Pending(
            ids.iter().map(|id| (*id).to_owned()).collect(),
        ))
    };
    let (_, button) = &file.declarations.widgets[0];
    assert_eq!(button.default_size, pending(&["U1"]));
    assert_eq!(
        button.sizes_by_variant.as_ref().unwrap()[0].default_size,
        pending(&["U2", "U3"])
    );
    assert_eq!(
        button.sizes_by_context.as_ref().unwrap()[0].default_size,
        pending(&["U4"])
    );
    let (_, flavor) = &file.declarations.flavor[0];
    assert_eq!(flavor.widgets[0].1.default_size, pending(&["U5"]));
    for path in [
        "declare.widget.button.sizes_by_variant.flat.default_size",
        "declare.widget.button.sizes_by_context.1.default_size",
        "declare.flavor.widgets.widget.button.default_size",
        "declare.widget.button.sizes",
        "declare.widget.button.sizes_by_context.x.default_size",
        "declare.component.rule_editor.default_size",
    ] {
        let Err(LayerError::Invalid(problems)) = read(&format!("\"{path}\" = \"U1\"")) else {
            panic!("{path}");
        };
        assert_eq!(
            problems.iter().map(ToString::to_string).collect::<Vec<_>>(),
            [format!("pending.\"{path}\": unknown key")],
            "{path}"
        );
    }
}

/// The reader's exemption covers exactly the listed leaves: a map entry
/// built whole may leave out its listed leaf, never another (S17,
/// ADD-A128).
#[test]
fn pending_exemption_is_the_listed_leaves() {
    let base = include_str!("../../../tests/fixtures/facts_gate/base.toml");
    let entry = "rendering_mode = \"template\"\nweight = \"absent\"\n";
    let with_entry = |listed: &str, entry: &str| {
        format!(
            "[pending]\n{listed}\n{}\n[tokens.icons.sizes.keyed.check]\nsize_px = 16\n\
             fill = \"#000000\"\nscale = \"absent\"\naxes = {{}}\nstate_effect = \"absent\"\n\
             symbol_variants = []\nsymbolic_colors = {{}}\nanimation = \"absent\"\n{entry}",
            base.replace("keyed = {}\n[tokens.focus]", "[tokens.focus]")
        )
    };
    let listed = "\"tokens.icons.sizes.keyed.check.rendering_mode\" = \"U2\"";
    // A listed leaf the entry states too reads; the facts gate fails the
    // entry (S17, ADD-A128).
    read_preset(&with_entry(listed, entry)).unwrap();
    read_preset(&with_entry(listed, "weight = \"absent\"\n")).unwrap();
    let Err(LayerError::Invalid(problems)) = read_preset(&with_entry(listed, "")) else {
        panic!("an unlisted leaf left out is refused");
    };
    let problems: Vec<String> = problems.iter().map(ToString::to_string).collect();
    assert_eq!(problems, ["tokens.icons.sizes.keyed.check.weight: missing"]);
}

/// Through the plain route a listed path is the missing field it is (S16,
/// S17, ADD-A128): `ThemeError::Preset(LayerError::Invalid)`, its problem
/// S12's own.
#[test]
fn from_preset_refuses_a_listed_path_as_missing() {
    let Err(ThemeError::Preset(LayerError::Invalid(problems))) =
        preset_stack(None, &listed_preset(), Scheme::Light)
    else {
        panic!("a listed path is missing through the plain route");
    };
    assert_eq!(
        problems.iter().map(ToString::to_string).collect::<Vec<_>>(),
        ["tokens.behavior.touch.delay: missing"]
    );
}

/// Through the plain route a listed input the preset does not state is
/// S12's missing field at `inputs.<name>`, as the synthetic route reports
/// it with no layer stating it (S16, S17, ADD-A128); one the preset states
/// is not refused for it.
#[test]
fn from_preset_refuses_a_listed_input_as_missing() {
    let base = include_str!("../../../tests/fixtures/facts_gate/base.toml");
    let stated = format!("[pending]\n\"inputs.accent\" = \"U1\"\n{base}");
    let listed = stated.replacen(
        "[inputs.accent]\nlight = \"#3366cc\"\ndark = \"#3366cc\"\n",
        "",
        1,
    );
    assert_ne!(listed, stated);
    let Err(ThemeError::Preset(LayerError::Invalid(problems))) =
        preset_stack(None, &listed, Scheme::Light)
    else {
        panic!("a listed input is missing through the plain route");
    };
    assert_eq!(
        problems.iter().map(ToString::to_string).collect::<Vec<_>>(),
        ["inputs.accent: missing"]
    );
    let Err(ThemeError::Preset(LayerError::Invalid(synthetic))) =
        synthetic_stack(None, &listed, Scheme::Light, Vec::new())
    else {
        panic!("a listed input no layer states is missing");
    };
    assert_eq!(problems, synthetic, "the synthetic route's problem");
    assert!(preset_stack(None, &stated, Scheme::Light).is_ok());
}

/// The synthetic route validates the preset with a layer stating the
/// listed path, which then resolves as that layer states it; without one
/// the listed path is the missing field (S16, S17, ADD-A128).
#[test]
fn from_preset_synthetic_validates_and_resolves() {
    let layer = synthetic_layer("[tokens.behavior.touch]\ndelay_ms = 7\n");
    let stack = synthetic_stack(None, &listed_preset(), Scheme::Light, vec![layer]).unwrap();
    let theme = stack
        .resolve(
            &mut WindowState::default(),
            &WindowConditions::new(InputKind::Mouse),
        )
        .theme;
    let tokens = theme.tokens(&theme.root_scope(&root())).unwrap();
    assert_eq!(tokens.behavior.touch.delay, Some(Ms(7.0)));
    let Err(ThemeError::Preset(LayerError::Invalid(problems))) =
        synthetic_stack(None, &listed_preset(), Scheme::Light, Vec::new())
    else {
        panic!("a listed path no layer states is missing");
    };
    assert_eq!(
        problems.iter().map(ToString::to_string).collect::<Vec<_>>(),
        ["tokens.behavior.touch.delay: missing"],
        "the plain route's problem, at its path"
    );
}

/// A theme builds and answers its queries on a small stack: every bundled
/// preset through `Theme::from_preset`, and the facts gate's complete base
/// as a preset, in both schemes, validated, resolved, and queried (the
/// tokens, which `ResolvedTheme::tokens` returns by value, each button
/// part's style and the button's values) on a 256 KiB thread, an eighth of
/// a spawned thread's default. The patches a layer holds are boxed where
/// they fan out (`layer.rs`'s `patch_holders_stay_small`), and
/// `ResolvedTokens` is bounded (`resolved_tokens_stay_bounded`), so the
/// stack this takes does not grow with the catalogue's widgets. A failure
/// shows the panic's message.
#[test]
fn presets_build_on_a_small_stack() {
    fn query(theme: &crate::next::resolve::ResolvedTheme) {
        let scope = theme.root_scope(&root());
        assert!(theme.tokens(&scope).is_some());
        let parts = crate::next::registry::widget("button")
            .map(|info| info.parts)
            .unwrap_or_default();
        for part in parts {
            let typed = TypedPart::from_names(crate::next::widgets::WidgetKind::Button, part.name)
                .unwrap_or_else(|| panic!("button.{}", part.name));
            let element = Element::new(PartRef::Typed(typed), &scope, &NoMetrics);
            let _ = theme.style(&element);
        }
        let body = PartRef::Typed(TypedPart::Button(ButtonPart::Body));
        let element = Element::new(body, &scope, &NoMetrics);
        let _ = theme.values::<crate::next::widgets::ButtonValues>(&element);
    }
    let run = || {
        let complete = include_str!("../../../tests/fixtures/facts_gate/base.toml");
        for scheme in [Scheme::Light, Scheme::Dark] {
            for &id in PresetId::all() {
                query(
                    &super::Theme::from_preset(id, scheme)
                        .unwrap_or_else(|e| panic!("{}: {e}", id.key()))
                        .resolve(
                            &mut WindowState::default(),
                            &WindowConditions::new(InputKind::Mouse),
                        )
                        .theme,
                );
            }
            let resolution = preset_stack(None, complete, scheme).unwrap().resolve(
                &mut WindowState::default(),
                &WindowConditions::new(InputKind::Mouse),
            );
            assert!(!resolution.theme.tokens.is_empty());
            query(&resolution.theme);
        }
    };
    let thread = std::thread::Builder::new()
        .stack_size(256 << 10)
        .spawn(run)
        .unwrap();
    if let Err(payload) = thread.join() {
        let message = payload
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| {
                payload
                    .downcast_ref::<&str>()
                    .map(|text| (*text).to_owned())
            })
            .unwrap_or_default();
        panic!("the preset did not build and answer on 256 KiB: {message}");
    }
}
