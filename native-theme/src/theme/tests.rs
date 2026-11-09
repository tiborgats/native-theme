//! S16's `Theme` over the bundled presets, and S19.1's build test. The
//! mutations' own tests are the stack's (`resolve/mutation_tests.rs`).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::{preset_stack, synthetic_stack};
use crate::blocks::text::ResolvedFont;
use crate::declare::DefaultSize;
use crate::error::{LayerError, ProblemKind, ReaderStatus, ThemeError};
use crate::gate::synthetic_layers;
use crate::layer::Layer;
use crate::presets::PresetId;
use crate::query::{Element, FontMetrics, RootInputs, Styled, Unavailable};
use crate::resolve::{Stack, WindowConditions, WindowState};
use crate::selector::{ControlSize, Direction, InputKind, PartRef, Scheme};
use crate::toml::layer::read_preset;
use crate::units::{Ms, Px, Size};
use crate::widgets::{ButtonPart, TypedPart};

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
/// (`from_preset_synthetic`'s) with one synthetic layer per listed path,
/// a refusal naming the listed paths whose synthetic values the refused
/// stack holds (S17, ADD-A131): a listed default size and a listed
/// presence hold none, the route waiving the one and drawing the other.
fn validates(id: Option<PresetId>, text: &str) -> Result<(), String> {
    let file = read_preset(text).map_err(|e| e.to_string())?;
    if file.pending.is_empty() {
        preset_stack(id, text, Scheme::Light).map_err(|e| e.to_string())?;
    } else {
        let layers = synthetic_layers(&file.pending, &file.layer)?;
        synthetic_stack(id, text, Scheme::Light, layers).map_err(|e| {
            let listed: Vec<&str> = file
                .pending
                .iter()
                .map(|(path, _)| path.as_str())
                .filter(|path| {
                    crate::toml::read::listed_default(path).is_none()
                        && crate::toml::read::listed_part(path).is_none()
                })
                .collect();
            format!(
                "{e}; the stack holds the synthetic values of {}",
                listed.join(", ")
            )
        })?;
    }
    Ok(())
}

/// A synthetic value S12 refuses: the build test fails with S12's problem
/// and names the listed paths whose synthetic values the stack holds (S17,
/// ADD-A131): the listed touch delay's synthetic value, `absent`, read by a
/// preset rule where a duration may not be absent. A stated delay would be
/// accepted there, so the refusal is the gate's choice of values to fix,
/// not S17's case of no single accepted value; a listed presence beside it
/// holds no synthetic value and is not named.
#[test]
fn build_test_names_a_refused_synthetic_value() {
    let rule = "\n[[rules]]\ntarget = \"tokens\"\nset = { caret.blink_on_ms = \"= token.behavior.touch.delay\" }\n";
    let text = format!(
        "{}{rule}",
        listed_preset().replacen(
            "[pending]\n",
            "[pending]\n\"widgets.button.subtitle\" = \"U3\"\n",
            1
        )
    );
    let refused = validates(None, &text).unwrap_err();
    assert!(refused.contains("may not be absent"), "{refused}");
    assert!(
        refused.ends_with("the stack holds the synthetic values of tokens.behavior.touch.delay_ms"),
        "{refused}"
    );
}

/// A listed `token.behavior.focus_visible_when` a widget's own value
/// references: its synthetic value is stated, not `absent`, as the
/// widget's `focus_visible_when` may not be absent (S17, ADD-A131: never
/// `Absent` where a check refuses it there), so the build test loads it.
#[test]
fn a_listed_focus_visible_token_a_widget_reads_loads() {
    let base = include_str!("../../tests/fixtures/facts_gate/base.toml");
    let token = "\nfocus_visible_when = \"absent\"\n";
    let value = "\nfocus_visible_when = \"any_focus\"\n";
    assert!(base.contains(token) && base.contains(value));
    let text = format!(
        "[pending]\n\"tokens.behavior.focus_visible_when\" = \"U1\"\n{}",
        base.replacen(token, "\n", 1).replacen(
            value,
            "\nfocus_visible_when = \"= token.behavior.focus_visible_when\"\n",
            1
        )
    );
    validates(None, &text).unwrap();
}

/// `adwaita` while its `[pending]` table lists paths (S17, ADD-A128):
/// `Theme::from_preset` refuses it with S12's missing field at each listed
/// path no layer states, and `Theme::from_preset_synthetic` loads it with
/// one synthetic layer per listed path.
#[test]
fn adwaita_refused_plain_and_loaded_synthetic() {
    let file = read_preset(PresetId::Adwaita.source()).unwrap();
    assert!(!file.pending.is_empty());
    let Err(ThemeError::Preset(LayerError::Invalid(problems))) =
        super::Theme::from_preset(PresetId::Adwaita, Scheme::Light)
    else {
        panic!("adwaita's listed paths are missing on the plain route");
    };
    let missing: Vec<String> = problems
        .iter()
        .filter(|problem| problem.kind == ProblemKind::Missing)
        .map(|problem| problem.path.clone())
        .collect();
    assert_eq!(missing.len(), file.pending.len(), "{problems:?}");
    assert_eq!(missing.len(), problems.len(), "{problems:?}");
    let layers = synthetic_layers(&file.pending, &file.layer).unwrap();
    let theme =
        super::Theme::from_preset_synthetic(PresetId::Adwaita, Scheme::Light, layers).unwrap();
    assert_eq!(theme.preset(), PresetId::Adwaita);
}

/// `macos-tahoe` while its `[pending]` table lists paths (S16, S17,
/// ADD-A128, ADD-A133): `Theme::from_preset` refuses it, each listed path
/// a field no layer states among the problems (a listed default size and
/// a listed input by their own problems), and
/// `Theme::from_preset_synthetic` loads it with its synthetic layers.
#[test]
fn macos_tahoe_refused_plain_and_loaded_synthetic() {
    let file = read_preset(PresetId::MacosTahoe.source()).unwrap();
    assert!(!file.pending.is_empty());
    let Err(ThemeError::Preset(LayerError::Invalid(problems))) =
        super::Theme::from_preset(PresetId::MacosTahoe, Scheme::Light)
    else {
        panic!("macos-tahoe's listed paths are missing on the plain route");
    };
    for (path, _) in &file.pending {
        if crate::toml::read::listed_default(path).is_some()
            || crate::toml::read::listed_input(path).is_some()
        {
            continue;
        }
        let listed = crate::toml::read::listed_path(path);
        assert!(
            problems
                .iter()
                .any(|problem| problem.kind == ProblemKind::Missing && problem.path == listed),
            "{listed}"
        );
    }
    let layers = synthetic_layers(&file.pending, &file.layer).unwrap();
    let theme =
        super::Theme::from_preset_synthetic(PresetId::MacosTahoe, Scheme::Light, layers).unwrap();
    assert_eq!(theme.preset(), PresetId::MacosTahoe);
}

/// `macos-monterey` while its `[pending]` table lists paths (S16, S17,
/// ADD-A128, ADD-A133): `Theme::from_preset` refuses it, each listed path
/// a field no layer states among the problems (a listed default size and
/// a listed input by their own problems), and
/// `Theme::from_preset_synthetic` loads it with its synthetic layers.
#[test]
fn macos_monterey_refused_plain_and_loaded_synthetic() {
    let file = read_preset(PresetId::MacosMonterey.source()).unwrap();
    assert!(!file.pending.is_empty());
    let Err(ThemeError::Preset(LayerError::Invalid(problems))) =
        super::Theme::from_preset(PresetId::MacosMonterey, Scheme::Light)
    else {
        panic!("macos-monterey's listed paths are missing on the plain route");
    };
    for (path, _) in &file.pending {
        if crate::toml::read::listed_default(path).is_some()
            || crate::toml::read::listed_input(path).is_some()
        {
            continue;
        }
        let listed = crate::toml::read::listed_path(path);
        assert!(
            problems
                .iter()
                .any(|problem| problem.kind == ProblemKind::Missing && problem.path == listed),
            "{listed}"
        );
    }
    let layers = synthetic_layers(&file.pending, &file.layer).unwrap();
    let theme = super::Theme::from_preset_synthetic(PresetId::MacosMonterey, Scheme::Light, layers)
        .unwrap();
    assert_eq!(theme.preset(), PresetId::MacosMonterey);
}

/// A bundled preset while its `[pending]` table lists paths (S16, S17,
/// ADD-A128, ADD-A133): `Theme::from_preset` refuses it, each listed path
/// a field no layer states among the problems (a listed default size and
/// a listed input by their own problems), and
/// `Theme::from_preset_synthetic` loads it with its synthetic layers.
fn refused_plain_and_loaded_synthetic(id: PresetId) {
    let file = read_preset(id.source()).unwrap();
    assert!(!file.pending.is_empty());
    let Err(ThemeError::Preset(LayerError::Invalid(problems))) =
        super::Theme::from_preset(id, Scheme::Light)
    else {
        panic!("{}'s listed paths are missing on the plain route", id.key());
    };
    for (path, _) in &file.pending {
        if crate::toml::read::listed_default(path).is_some()
            || crate::toml::read::listed_input(path).is_some()
        {
            continue;
        }
        let listed = crate::toml::read::listed_path(path);
        assert!(
            problems
                .iter()
                .any(|problem| problem.kind == ProblemKind::Missing && problem.path == listed),
            "{}: {listed}",
            id.key()
        );
    }
    let layers = synthetic_layers(&file.pending, &file.layer).unwrap();
    let theme = super::Theme::from_preset_synthetic(id, Scheme::Light, layers).unwrap();
    assert_eq!(theme.preset(), id);
}

/// `material-3` through both routes, as [`refused_plain_and_loaded_synthetic`].
#[test]
fn material_3_refused_plain_and_loaded_synthetic() {
    refused_plain_and_loaded_synthetic(PresetId::Material3);
}

/// `ios-26` through both routes, as [`refused_plain_and_loaded_synthetic`].
#[test]
fn ios_26_refused_plain_and_loaded_synthetic() {
    refused_plain_and_loaded_synthetic(PresetId::Ios26);
}

/// `ios-13` through both routes, as [`refused_plain_and_loaded_synthetic`].
#[test]
fn ios_13_refused_plain_and_loaded_synthetic() {
    refused_plain_and_loaded_synthetic(PresetId::Ios13);
}

/// `kde-breeze` through both routes, as [`refused_plain_and_loaded_synthetic`].
#[test]
fn kde_breeze_refused_plain_and_loaded_synthetic() {
    refused_plain_and_loaded_synthetic(PresetId::KdeBreeze);
}

/// `windows-11` through both routes, as [`refused_plain_and_loaded_synthetic`].
#[test]
fn windows_11_refused_plain_and_loaded_synthetic() {
    refused_plain_and_loaded_synthetic(PresetId::Windows11);
}

/// The build test's two routes, over the fixture preset with a listed
/// path (a small preset, beside `adwaita`'s whole one): the build test takes the
/// synthetic route for it, whose layers state the listed path, so it
/// validates, where the plain route refuses it; and the plain route for
/// the complete base.
#[test]
fn build_test_takes_both_routes() {
    let text = listed_preset();
    assert!(!read_preset(&text).unwrap().pending.is_empty());
    validates(None, &text).unwrap();
    assert!(preset_stack(None, &text, Scheme::Light).is_err());
    let complete = include_str!("../../tests/fixtures/facts_gate/base.toml");
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
    let base = include_str!("../../tests/fixtures/facts_gate/base.toml");
    let text = format!(
        "[pending]\n\"tokens.behavior.key_repeat\" = \"U1\"\n\"tokens.focus.width_px\" = \"U2\"\n{}",
        base.replacen("\nkey_repeat = \"none\"\n", "\n", 1)
            .replacen("\nwidth_px = 2\n", "\n", 1)
    );
    let file = read_preset(&text).unwrap();
    let layers = synthetic_layers(&file.pending, &file.layer).unwrap();
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
    let base = include_str!("../../tests/fixtures/facts_gate/base.toml");
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
    let layers = synthetic_layers(&file.pending, &file.layer).unwrap();
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
    let base = include_str!("../../tests/fixtures/facts_gate/base.toml");
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
    let base = include_str!("../../tests/fixtures/facts_gate/base.toml");
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
    let base = include_str!("../../tests/fixtures/facts_gate/base.toml");
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

/// The synthetic route checks its layers in one pass, which finds what each
/// one's check over the layers below it finds only where they state
/// constant values at disjoint places (S17, ADD-A131): a layer stating the
/// listed delay as an expression reading itself, which S12 refuses as a
/// cycle, is refused though a later layer states the delay as a constant
/// and the pass would never meet the expression, and two layers stating one
/// place with constants are refused.
#[test]
fn synthetic_route_refuses_expressions_and_shared_places() {
    let delay =
        |value: &str| synthetic_layer(&format!("[tokens.behavior.touch]\ndelay_ms = {value}"));
    let refused = |layers: Vec<Layer>| -> Vec<String> {
        match synthetic_stack(None, &listed_preset(), Scheme::Light, layers) {
            Ok(_) => Vec::new(),
            Err(ThemeError::Preset(LayerError::Invalid(problems))) => {
                problems.iter().map(ToString::to_string).collect()
            }
            Err(other) => panic!("{other}"),
        }
    };
    let cycle = || delay("\"= token.behavior.touch.delay\"");
    let expression = "tokens.behavior.touch.delay_ms: an expression in a synthetic layer, which \
                      states constant values";
    let shared = "tokens.behavior.touch.delay_ms: a place an earlier synthetic layer states \
                  too, or one above or below it";
    assert_eq!(refused(vec![cycle()]), [expression]);
    assert_eq!(refused(vec![cycle(), delay("1")]), [expression, shared]);
    assert_eq!(refused(vec![delay("1"), delay("2")]), [shared]);
    assert_eq!(refused(vec![delay("1")]), Vec::<String>::new());
}

/// The reader reads `[pending]` with its register ids (S11), one as the
/// string or a list of two or more distinct ones, and leaves a listed path
/// out of a block built whole; it reads an input's at a scheme
/// (`inputs.<name>.<scheme>`, ADD-A132) and a typed part's presence
/// (`widgets.<widget>.<part>`, ADD-A131); it refuses an entry whose id is
/// not `U<n>` or whose list is empty, holds one id (ADD-A131) or repeats
/// one, or whose path names no field of `colors`, `tokens` or a typed
/// widget, no input at a scheme and no part (a rule's key, a field the
/// model lacks, an input's name that does not read or names no scheme) or
/// is spelled with a unit suffix its field's type does not take (S17,
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
    let base = include_str!("../../tests/fixtures/facts_gate/base.toml");
    let refused = |table: &str| match read_preset(&format!("[pending]\n{table}\n{base}")) {
        Err(LayerError::Invalid(problems)) => {
            problems.iter().map(ToString::to_string).collect::<Vec<_>>()
        }
        other => panic!("{table}: {other:?}"),
    };
    let not_an_id = [
        "pending.\"tokens.behavior.drag_time_ms\": expected a register id, `U<n>`, \
                      or a list of two or more distinct ones",
    ];
    for ids in [
        "\"151\"",
        "151",
        "\"U\"",
        "\"U12a\"",
        "[]",
        "[\"U1\"]",
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
        "inputs.Accent.light",
        "inputs.accent",
        "inputs.accent.contrast.high",
        "widgets.button.bogus",
        "widgets.bogus.label",
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
        assert!(crate::toml::read::listed_field(path).is_some(), "{path}");
    }
    for path in ["inputs.accent.dark", "widgets.button.label"] {
        let listed = read_preset(&format!("[pending]\n\"{path}\" = \"U1\"\n{base}")).unwrap();
        assert_eq!(listed.pending.len(), 1, "{path}");
    }
}

/// A pending default size is listed by its declaration's path (S11, S17,
/// ADD-A131): the widget's, a `sizes_by_variant` entry's by its variant,
/// `@` included, a `sizes_by_context` entry's by its position, a
/// flavour's under `declare.flavor.<f>.`; the reader sets each pending
/// with its ids, and refuses a path naming no section or entry the
/// declarations state, or no default size.
#[test]
fn pending_default_sizes_read() {
    let base = include_str!("../../tests/fixtures/facts_gate/base.toml");
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
    let base = include_str!("../../tests/fixtures/facts_gate/base.toml");
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

/// A block whose every leaf is listed (S12, S17, ADD-A131): the preset's
/// bases leave the focus indicator's `offset` out, each of its sides
/// listed; the listed leaves' synthetic values complete the block, so the
/// synthetic route loads it and the plain route refuses each listed leaf,
/// never the block. A side left unlisted is the missing field at its own
/// path, the listed sides being the layers'.
#[test]
fn a_block_whose_every_leaf_is_listed() {
    let base = include_str!("../../tests/fixtures/facts_gate/base.toml");
    let line = "\noffset = { top_px = 0, right_px = 0, bottom_px = 0, left_px = 0 }\n";
    assert!(base.contains(line));
    let sides = ["top", "right", "bottom", "left"];
    let listing = |listed: &[&str]| {
        let entries: String = listed
            .iter()
            .enumerate()
            .map(|(n, side)| format!("\"tokens.focus.offset.{side}_px\" = \"U{}\"\n", n + 1))
            .collect();
        format!("[pending]\n{entries}{}", base.replacen(line, "\n", 1))
    };
    let every = listing(&sides);
    let file = read_preset(&every).unwrap();
    let layers = synthetic_layers(&file.pending, &file.layer).unwrap();
    synthetic_stack(None, &every, Scheme::Light, layers).unwrap();
    let Err(ThemeError::Preset(LayerError::Invalid(problems))) =
        preset_stack(None, &every, Scheme::Light)
    else {
        panic!("the listed sides are missing through the plain route");
    };
    assert_eq!(
        problems.iter().map(ToString::to_string).collect::<Vec<_>>(),
        sides.map(|side| format!("tokens.focus.offset.{side}: missing"))
    );
    let three = listing(&sides[..3]);
    let file = read_preset(&three).unwrap();
    let layers = synthetic_layers(&file.pending, &file.layer).unwrap();
    let Err(ThemeError::Preset(LayerError::Invalid(problems))) =
        synthetic_stack(None, &three, Scheme::Light, layers)
    else {
        panic!("the unlisted side is missing");
    };
    assert_eq!(
        problems.iter().map(ToString::to_string).collect::<Vec<_>>(),
        ["tokens.focus.offset.left: missing"]
    );
}

/// A part whose every field is listed, its table left out of the preset's
/// bases (S6.3, S12, S17, ADD-A131): the listed fields' synthetic values
/// state the part, which no presence hides, so the synthetic route loads it
/// and the plain route refuses the listed field, never the part.
#[test]
fn a_part_whose_every_field_is_listed() {
    let base = include_str!("../../tests/fixtures/facts_gate/base.toml");
    let table = "\n[widgets.button.subtitle]\ntext = \"= token.text.roles.body\"\n";
    assert!(base.contains(table));
    let text = format!(
        "[pending]\n\"widgets.button.subtitle.text\" = \"U1\"\n{}",
        base.replacen(table, "\n", 1)
    );
    let file = read_preset(&text).unwrap();
    let layers = synthetic_layers(&file.pending, &file.layer).unwrap();
    synthetic_stack(None, &text, Scheme::Light, layers).unwrap();
    let Err(ThemeError::Preset(LayerError::Invalid(problems))) =
        preset_stack(None, &text, Scheme::Light)
    else {
        panic!("the listed field is missing through the plain route");
    };
    assert_eq!(
        problems.iter().map(ToString::to_string).collect::<Vec<_>>(),
        ["widgets.button.subtitle.text: missing"]
    );
}

/// A declared surface set whose every role is listed at both schemes, its
/// table left out of the preset's bases (S2.2, S3, S12, S17, ADD-A131):
/// the listed roles' synthetic layers state the set, so the synthetic
/// route loads it and the plain route refuses each listed role, never the
/// set.
#[test]
fn a_surface_set_whose_every_role_is_listed() {
    let base = include_str!("../../tests/fixtures/facts_gate/base.toml");
    let declared = "surface_sets = [\"window\"]\n";
    assert!(base.contains(declared));
    let mut roles = Vec::new();
    crate::registry::leaves(
        "",
        <crate::roles::SurfaceRoles as crate::blocks::Table>::FIELDS,
        &mut roles,
    );
    let listed: Vec<String> = ["light", "dark"]
        .iter()
        .flat_map(|scheme| {
            roles
                .iter()
                .map(move |role| format!("colors.{scheme}.surfaces.selection.{role}"))
        })
        .collect();
    let entries: String = listed
        .iter()
        .map(|path| format!("\"{path}\" = \"U1\"\n"))
        .collect();
    let text = format!(
        "[pending]\n{entries}{}",
        base.replacen(declared, "surface_sets = [\"window\", \"selection\"]\n", 1)
    );
    let file = read_preset(&text).unwrap();
    let layers = synthetic_layers(&file.pending, &file.layer).unwrap();
    synthetic_stack(None, &text, Scheme::Light, layers).unwrap();
    let Err(ThemeError::Preset(LayerError::Invalid(problems))) =
        preset_stack(None, &text, Scheme::Light)
    else {
        panic!("the listed roles are missing through the plain route");
    };
    let missing: Vec<String> = problems
        .iter()
        .filter(|problem| problem.kind == ProblemKind::Missing)
        .map(|problem| problem.path.clone())
        .collect();
    assert_eq!(missing, listed, "{problems:?}");
}

/// Two listed entries of one map, each its own synthetic layer (S17,
/// ADD-A128, ADD-A134): `tokens.elevation.menu` and `.popup`, each an
/// empty list, state the map's two entries, disjoint places, not the map
/// whole, so the synthetic route loads them, the declared names used.
#[test]
fn listed_entries_of_one_map_do_not_overlap() {
    let base = include_str!("../../tests/fixtures/facts_gate/base.toml");
    let declared = "text_styles = [\"body\"]\n";
    assert!(base.contains(declared));
    let text = format!(
        "[pending]\n\"tokens.elevation.menu\" = \"U1\"\n\"tokens.elevation.popup\" = \"U2\"\n{}",
        base.replacen(
            declared,
            "text_styles = [\"body\"]\nelevation = [\"menu\", \"popup\"]\n",
            1
        )
    );
    let file = read_preset(&text).unwrap();
    let layers = synthetic_layers(&file.pending, &file.layer).unwrap();
    assert_eq!(layers.len(), 2);
    synthetic_stack(None, &text, Scheme::Light, layers).unwrap();
}

/// Every field of the keyed icon context `check` listed, a field of its
/// entry stated in the preset's bases where `stated` names one (S17,
/// ADD-A133): the base preset with `check` declared.
fn listed_entry(stated: Option<&str>) -> String {
    let base = include_str!("../../tests/fixtures/facts_gate/base.toml");
    let fields = [
        "size_px",
        "fill",
        "rendering_mode",
        "weight",
        "scale",
        "axes",
        "state_effect",
        "symbol_variants",
        "symbolic_colors",
        "animation",
    ];
    let listed: String = fields
        .iter()
        .filter(|field| stated.is_none_or(|stated| !stated.starts_with(*field)))
        .enumerate()
        .map(|(n, field)| {
            format!(
                "\"tokens.icons.sizes.keyed.check.{field}\" = \"U{}\"\n",
                n + 1
            )
        })
        .collect();
    let entry = stated.map_or_else(String::new, |stated| {
        format!("[tokens.icons.sizes.keyed.check]\n{stated}\n")
    });
    let base = if stated.is_some() {
        base.replacen("keyed = {}\n[tokens.focus]", "[tokens.focus]", 1)
    } else {
        base.to_owned()
    };
    format!(
        "[pending]\n{listed}{}{entry}",
        base.replacen(
            "text_styles = [\"body\"]\n",
            "text_styles = [\"body\"]\nicon_contexts = [\"check\"]\n",
            1
        )
    )
}

/// A map entry whose every field is listed, which no base of the preset
/// states (S12, S17, S19.1, ADD-A133): its listed paths share one
/// synthetic layer, which creates the entry with every field stated and
/// states its keyed name, so the synthetic route loads it, the name used;
/// the plain route refuses each listed path.
#[test]
fn a_listed_entry_shares_one_synthetic_layer() {
    let text = listed_entry(None);
    let file = read_preset(&text).unwrap();
    assert_eq!(file.pending.len(), 10);
    let layers = synthetic_layers(&file.pending, &file.layer).unwrap();
    assert_eq!(layers.len(), 1);
    synthetic_stack(None, &text, Scheme::Light, layers).unwrap();
    let Err(ThemeError::Preset(LayerError::Invalid(problems))) =
        preset_stack(None, &text, Scheme::Light)
    else {
        panic!("the listed fields are missing through the plain route");
    };
    let problems: Vec<String> = problems.iter().map(ToString::to_string).collect();
    for (path, _) in &file.pending {
        let listed = crate::toml::read::listed_path(path);
        assert!(
            problems.contains(&format!("{listed}: missing")),
            "{listed}: {problems:?}"
        );
    }
}

/// An entry a base of the preset creates keeps one synthetic layer per
/// listed path (S17, ADD-A133): the base states its `size`, the layers
/// state the other fields one each.
#[test]
fn an_entry_a_base_creates_keeps_a_layer_per_path() {
    let text = listed_entry(Some("size_px = 16"));
    let file = read_preset(&text).unwrap();
    assert_eq!(file.pending.len(), 9);
    let layers = synthetic_layers(&file.pending, &file.layer).unwrap();
    assert_eq!(layers.len(), 9);
    synthetic_stack(None, &text, Scheme::Light, layers).unwrap();
}

/// A refused stack holding a shared layer still names each listed path
/// whose synthetic value it holds (S17, ADD-A131, ADD-A133).
#[test]
fn a_refusal_names_each_listed_path_of_a_shared_layer() {
    let rule = "\n[[rules]]\ntarget = \"tokens\"\nset = { caret.blink_on_ms = \"= token.behavior.touch.delay\" }\n";
    let text = format!(
        "{}{rule}",
        listed_entry(None)
            .replacen("\ndelay_ms = \"absent\"\n", "\n", 1)
            .replacen(
                "[pending]\n",
                "[pending]\n\"tokens.behavior.touch.delay_ms\" = \"U11\"\n",
                1
            )
    );
    let refused = validates(None, &text).unwrap_err();
    assert!(refused.contains("may not be absent"), "{refused}");
    for path in [
        "tokens.behavior.touch.delay_ms",
        "tokens.icons.sizes.keyed.check.size_px",
        "tokens.icons.sizes.keyed.check.animation",
    ] {
        assert!(refused.contains(path), "{path}: {refused}");
    }
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

/// Through the plain route a listed input the preset does not state at its
/// scheme is S12's missing field at `inputs.<name>.<scheme>`, as the
/// synthetic route reports it with no layer stating it (S16, S17,
/// ADD-A132); one the preset states there is not refused for it.
#[test]
fn from_preset_refuses_a_listed_input_as_missing() {
    let base = include_str!("../../tests/fixtures/facts_gate/base.toml");
    let stated = format!("[pending]\n\"inputs.accent.dark\" = \"U1\"\n{base}");
    let listed = stated.replacen(
        "[inputs.accent]\nlight = \"#3366cc\"\ndark = \"#3366cc\"\n",
        "[inputs.accent]\nlight = \"#3366cc\"\n",
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
        ["inputs.accent.dark: missing"]
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
/// preset through `Theme::from_preset`, or `Theme::from_preset_synthetic`
/// while its `[pending]` table lists paths (S17, as the build test takes
/// it), and the facts gate's complete base
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
    fn query(theme: &crate::resolve::ResolvedTheme) {
        let scope = theme.root_scope(&root());
        assert!(theme.tokens(&scope).is_some());
        let parts = crate::registry::widget("button")
            .map(|info| info.parts)
            .unwrap_or_default();
        for part in parts {
            let typed = TypedPart::from_names(crate::widgets::WidgetKind::Button, part.name)
                .unwrap_or_else(|| panic!("button.{}", part.name));
            let element = Element::new(PartRef::Typed(typed), &scope, &NoMetrics);
            let _ = theme.style(&element);
        }
        let body = PartRef::Typed(TypedPart::Button(ButtonPart::Body));
        let element = Element::new(body, &scope, &NoMetrics);
        let _ = theme.values::<crate::widgets::ButtonValues>(&element);
    }
    let run = || {
        let complete = include_str!("../../tests/fixtures/facts_gate/base.toml");
        for scheme in [Scheme::Light, Scheme::Dark] {
            for &id in PresetId::all() {
                let file = read_preset(id.source()).unwrap();
                let theme = if file.pending.is_empty() {
                    super::Theme::from_preset(id, scheme)
                } else {
                    super::Theme::from_preset_synthetic(
                        id,
                        scheme,
                        synthetic_layers(&file.pending, &file.layer).unwrap(),
                    )
                };
                query(
                    &theme
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

/// The facts gate fixtures' complete base, a preset with no listed path.
const BASE: &str = include_str!("../../tests/fixtures/facts_gate/base.toml");

/// `stack` switched to the preset `text` states (S16).
fn switch(stack: &mut Stack, text: &str) -> Result<(), LayerError> {
    let file = read_preset(text).unwrap();
    stack.set_preset(None, file.declarations, file.layer, file.pending)
}

/// The problems a refused change names, as text.
fn named(refused: Result<(), LayerError>) -> Vec<String> {
    match refused {
        Err(LayerError::Invalid(problems)) => problems.iter().map(ToString::to_string).collect(),
        other => panic!("refused with problems: {other:?}"),
    }
}

/// The base with its subtitle's presence listed (S17, ADD-A131): it
/// states the part's fields and no presence.
fn listed_presence() -> String {
    format!("[pending]\n\"widgets.button.subtitle\" = \"U3\"\n{BASE}")
}

/// One refusal rule on every route but the synthetic one (S12, S17,
/// ADD-A131): a listed path no layer states is S12's missing field, through
/// `from_preset` and `set_preset` alike; one an application layer or the
/// stand-in states is stated (a reader-backed token), and removing that
/// layer is refused again.
#[test]
fn a_listed_path_is_stated_by_a_layer_above() {
    let mut stack = preset_stack(None, BASE, Scheme::Light).unwrap();
    assert_eq!(
        named(switch(&mut stack, &listed_preset())),
        ["tokens.behavior.touch.delay: missing"]
    );
    let delay = Layer::from_toml("[tokens.behavior.touch]\ndelay_ms = 7\n").unwrap();
    let id = stack.add_layer(delay.clone()).unwrap();
    switch(&mut stack, &listed_preset()).unwrap();
    assert_eq!(
        named(stack.remove_layer(id)),
        ["tokens.behavior.touch.delay: missing"]
    );
    let mut read = preset_stack(None, BASE, Scheme::Light).unwrap();
    read.set_reader_stand_in(crate::resolve::ReaderState::new(Scheme::Light, 1.0), delay)
        .unwrap();
    switch(&mut read, &listed_preset()).unwrap();
}

/// A listed presence (S17, ADD-A131): refused through the plain route as
/// S12's missing field at the part's path unless an application layer's
/// bases state the part's presence (`absent = true`, or `absent = false`
/// drawing it), a stand-in's not counting; drawn on the synthetic route,
/// which states nothing for it.
#[test]
fn a_listed_presence_is_stated_by_an_application_layer() {
    let Err(ThemeError::Preset(LayerError::Invalid(problems))) =
        preset_stack(None, &listed_presence(), Scheme::Light)
    else {
        panic!("a listed presence is missing through the plain route");
    };
    assert_eq!(
        problems.iter().map(ToString::to_string).collect::<Vec<_>>(),
        ["widgets.button.subtitle: missing"]
    );
    let hidden = Layer::from_toml("[widgets.button.subtitle]\nabsent = true\n").unwrap();
    let mut stand_in = preset_stack(None, BASE, Scheme::Light).unwrap();
    stand_in
        .set_reader_stand_in(
            crate::resolve::ReaderState::new(Scheme::Light, 1.0),
            hidden.clone(),
        )
        .unwrap();
    assert_eq!(
        named(switch(&mut stand_in, &listed_presence())),
        ["widgets.button.subtitle: missing"]
    );
    let mut stack = preset_stack(None, BASE, Scheme::Light).unwrap();
    stack.add_layer(hidden).unwrap();
    switch(&mut stack, &listed_presence()).unwrap();
    // An application drawing the part: `absent = false` states it too.
    let drawn = Layer::from_toml("[widgets.button.subtitle]\nabsent = false\n").unwrap();
    let mut drawing = preset_stack(None, BASE, Scheme::Light).unwrap();
    drawing.add_layer(drawn).unwrap();
    switch(&mut drawing, &listed_presence()).unwrap();
    let theme = drawing
        .resolve(
            &mut WindowState::default(),
            &WindowConditions::new(InputKind::Mouse),
        )
        .theme;
    let scope = theme.root_scope(&root());
    let subtitle = PartRef::Typed(TypedPart::Button(ButtonPart::Subtitle));
    assert!(matches!(
        theme.style(&Element::new(subtitle.clone(), &scope, &NoMetrics)),
        Styled::Style(_)
    ));
    let file = read_preset(&listed_presence()).unwrap();
    assert!(
        synthetic_layers(&file.pending, &file.layer)
            .unwrap()
            .is_empty()
    );
    let synthetic = synthetic_stack(None, &listed_presence(), Scheme::Light, Vec::new()).unwrap();
    let theme = synthetic
        .resolve(
            &mut WindowState::default(),
            &WindowConditions::new(InputKind::Mouse),
        )
        .theme;
    let scope = theme.root_scope(&root());
    assert!(matches!(
        theme.style(&Element::new(subtitle, &scope, &NoMetrics)),
        Styled::Style(_)
    ));
}

/// A synthetic theme keeps the route through its mutations while its
/// preset is current (S17, ADD-A131): a stand-in, a scheme, a contrast, a
/// flavour, an accent and a layer added, replaced and removed validate
/// with the listed default size waived and the listed presence drawn; a `set_preset` to the same preset ends the route,
/// refused as its listed paths are left unstated, the theme unchanged; one
/// to a complete preset succeeds and removes the synthetic layers.
#[test]
fn a_synthetic_theme_keeps_its_route_until_set_preset() {
    let text = format!(
        "[pending]\n\"tokens.behavior.touch.delay_ms\" = \"U1\"\n\
         \"declare.widget.button.default_size\" = \"U2\"\n\"widgets.button.subtitle\" = \"U3\"\n{}\n\
         [declare.widget.button]\nsizes = [\"small\", \"regular\"]\n\
         [[rules]]\nwhen = {{ conditions = [\"flavor:b\"] }}\ntarget = \"button.body\"\n\
         set = {{ opacity = 1 }}\n",
        BASE.replacen("\ndelay_ms = \"absent\"\n", "\n", 1)
            .replacen("[declare]\n", "[declare]\nflavors = [\"a\", \"b\"]\n", 1)
    );
    let file = read_preset(&text).unwrap();
    let layers = synthetic_layers(&file.pending, &file.layer).unwrap();
    assert_eq!(layers.len(), 1);
    let mut stack = synthetic_stack(None, &text, Scheme::Light, layers).unwrap();
    stack
        .set_reader_stand_in(
            crate::resolve::ReaderState::new(Scheme::Dark, 1.0),
            Layer::default(),
        )
        .unwrap();
    stack.pick(Some(Scheme::Light)).unwrap();
    stack
        .set_contrast(Some(crate::inputs::Contrast::High))
        .unwrap();
    stack.set_flavor(crate::key!("b")).unwrap();
    let accent = crate::inputs::InputValue::new(crate::value::SchemeMap::one(
        Scheme::Light,
        crate::value::Value::Stated(crate::color::Color::parse("#445566").unwrap()),
    ));
    stack
        .set_accent(Some(crate::inputs::Accent::new(accent)))
        .unwrap();
    let id = stack.add_layer(Layer::default()).unwrap();
    stack.replace_layer(id, Layer::default()).unwrap();
    stack.remove_layer(id).unwrap();
    stack.remove_reader_stand_in().unwrap();
    assert_eq!(stack.synthetic_places(), [1]);
    let kept = stack.clone();
    let refused = named(switch(&mut stack, &text));
    assert_eq!(
        refused,
        [
            "declare.widget.button.default_size: no default size where sizes are taken",
            "tokens.behavior.touch.delay: missing",
            "widgets.button.subtitle: missing",
        ]
    );
    assert_eq!(stack, kept, "a refused set_preset changes nothing");
    switch(&mut stack, BASE).unwrap();
    assert!(stack.synthetic_places().is_empty());
    assert_eq!(
        named(switch(&mut stack, &text)),
        [
            "declare.widget.button.default_size: no default size where sizes are taken",
            "tokens.behavior.touch.delay: missing",
            "widgets.button.subtitle: missing",
        ],
        "the synthetic layers are gone with the route"
    );
}

/// A listed input's synthetic value is folded into the preset's own input
/// base at its listed scheme and marked there (S17, ADD-A132): no
/// synthetic layer holds it; the mark is in force at that scheme alone,
/// not under a contrast the preset states a base for, and not where a
/// stand-in replaces the input at that slot.
#[test]
fn a_listed_input_is_folded_into_the_preset() {
    let text = format!(
        "[pending]\n\"inputs.accent.dark\" = \"U1\"\n{}\n\
         [inputs.accent.contrast.high]\ndark = \"#000000\"\n",
        BASE.replacen(
            "[inputs.accent]\nlight = \"#3366cc\"\ndark = \"#3366cc\"\n",
            "[inputs.accent]\nlight = \"#3366cc\"\n",
            1
        )
    );
    let file = read_preset(&text).unwrap();
    let layers = synthetic_layers(&file.pending, &file.layer).unwrap();
    assert_eq!(layers.len(), 1);
    assert!(!layers[0].bases.inputs.entries.is_empty());
    let mut stack = synthetic_stack(None, &text, Scheme::Dark, layers).unwrap();
    let accent = crate::inputs::InputName::new("accent").unwrap();
    let (_, value) = stack
        .preset_layer()
        .bases
        .inputs
        .entries
        .iter()
        .find(|(name, _)| *name == accent)
        .unwrap();
    assert_eq!(
        value.per_scheme.dark,
        Some(crate::value::Value::Stated(
            crate::color::Color::parse("#010203").unwrap()
        ))
    );
    let marked = |stack: &Stack| {
        stack
            .resolve(
                &mut WindowState::default(),
                &WindowConditions::new(InputKind::Mouse),
            )
            .theme
            .synthetic_inputs
    };
    assert_eq!(marked(&stack), std::slice::from_ref(&accent));
    let theme = stack
        .resolve(
            &mut WindowState::default(),
            &WindowConditions::new(InputKind::Mouse),
        )
        .theme;
    let scope = theme.root_scope(&root());
    let applies: &dyn Fn(&crate::selector::Selector) -> bool = &|_| true;
    let reference = crate::expr::Ref::Input(accent.segments().to_vec());
    let reached = theme.evaluate_in(
        &scope,
        None,
        applies,
        crate::resolve::compose::Target::Roles,
        None,
        |cx| {
            assert!(cx.color_of(&reference).is_some());
            cx.reached().len()
        },
    );
    assert!(
        reached > 0,
        "reading the folded value reaches a synthetic value"
    );
    stack.pick(Some(Scheme::Light)).unwrap();
    assert!(marked(&stack).is_empty());
    stack.pick(Some(Scheme::Dark)).unwrap();
    stack
        .set_contrast(Some(crate::inputs::Contrast::High))
        .unwrap();
    assert!(marked(&stack).is_empty());
    stack.set_contrast(None).unwrap();
    let stand_in = Layer::from_toml("[inputs.accent]\ndark = \"#445566\"\n").unwrap();
    stack
        .set_reader_stand_in(
            crate::resolve::ReaderState::new(Scheme::Dark, 1.0),
            stand_in,
        )
        .unwrap();
    assert!(marked(&stack).is_empty());
}

/// The inputs a stand-in carries with the preset's own value, a listed
/// one's folded synthetic value staying marked (S17 *A measured
/// derivation*, ADD-A132), are that stand-in's alone: a stand-in set in its
/// place stating the input's slot with its own value leaves no mark of the
/// input.
#[test]
fn carried_inputs_end_with_their_stand_in() {
    let text = format!(
        "[pending]\n\"inputs.accent.dark\" = \"U1\"\n{}",
        BASE.replacen(
            "[inputs.accent]\nlight = \"#3366cc\"\ndark = \"#3366cc\"\n",
            "[inputs.accent]\nlight = \"#3366cc\"\n",
            1
        )
    );
    let file = read_preset(&text).unwrap();
    let layers = synthetic_layers(&file.pending, &file.layer).unwrap();
    let mut stack = synthetic_stack(None, &text, Scheme::Dark, layers).unwrap();
    let accent = crate::inputs::InputName::new("accent").unwrap();
    let marked = |stack: &Stack| {
        stack
            .resolve(
                &mut WindowState::default(),
                &WindowConditions::new(InputKind::Mouse),
            )
            .theme
            .synthetic_inputs
    };
    let dark = || crate::resolve::ReaderState::new(Scheme::Dark, 1.0);
    let carrying = Layer::from_toml("[inputs.accent]\ndark = \"#010203\"\n").unwrap();
    let own = Layer::from_toml("[inputs.accent]\ndark = \"#445566\"\n").unwrap();
    stack
        .set_reader_stand_in_carrying(dark(), carrying.clone(), vec![accent.clone()])
        .unwrap();
    assert_eq!(marked(&stack), std::slice::from_ref(&accent));
    stack.set_reader_stand_in(dark(), own).unwrap();
    assert!(marked(&stack).is_empty());
    stack.set_reader_stand_in(dark(), carrying).unwrap();
    assert!(marked(&stack).is_empty());
}

/// A declared input is missing at a slot no applying layer gives it a value
/// at (S9, S12, ADD-A132), listed or not, through `from_preset` and
/// `set_preset` alike: at `inputs.<name>.<scheme>` where the preset states
/// only another scheme, a contrast base there giving no value with no
/// contrast; a stand-in stating it there is enough, and removing that
/// stand-in is refused again.
#[test]
fn a_declared_input_missing_at_a_slot() {
    let one = BASE.replacen(
        "[inputs.accent]\nlight = \"#3366cc\"\ndark = \"#3366cc\"\n",
        "[inputs.accent]\nlight = \"#3366cc\"\n",
        1,
    );
    assert_ne!(one, BASE);
    let based = format!("{one}\n[inputs.accent.contrast.high]\ndark = \"#000000\"\n");
    for text in [&one, &based] {
        let Err(ThemeError::Preset(LayerError::Invalid(problems))) =
            preset_stack(None, text, Scheme::Light)
        else {
            panic!("an input missing at a scheme is refused");
        };
        assert_eq!(
            problems.iter().map(ToString::to_string).collect::<Vec<_>>(),
            ["inputs.accent.dark: missing"]
        );
    }
    let mut stack = preset_stack(None, BASE, Scheme::Light).unwrap();
    assert_eq!(
        named(switch(&mut stack, &one)),
        ["inputs.accent.dark: missing"]
    );
    let dark = Layer::from_toml("[inputs.accent]\ndark = \"#445566\"\n").unwrap();
    stack
        .set_reader_stand_in(crate::resolve::ReaderState::new(Scheme::Light, 1.0), dark)
        .unwrap();
    switch(&mut stack, &based).unwrap();
    assert_eq!(
        named(stack.remove_reader_stand_in()),
        ["inputs.accent.dark: missing"]
    );
}

/// `adwaita` on the synthetic route, its listed paths stated (S17).
fn synthetic_adwaita() -> super::Theme {
    synthetic_theme(PresetId::Adwaita)
}

/// The preset `id` on the synthetic route, its listed paths stated (S17).
fn synthetic_theme(id: PresetId) -> super::Theme {
    let file = read_preset(id.source()).unwrap();
    let layers = synthetic_layers(&file.pending, &file.layer).unwrap();
    super::Theme::from_preset_synthetic(id, Scheme::Light, layers).unwrap()
}

/// The reader status without a system reader: `NoReader`, `StandIn` while
/// a stand-in is set; reloading reads nothing (S16).
#[test]
fn the_reader_status_without_a_system_reader() {
    let mut theme = synthetic_adwaita();
    assert_eq!(theme.reader_status(), ReaderStatus::NoReader);
    assert_eq!(theme.reload_reader(), ReaderStatus::NoReader);
    theme
        .set_reader_stand_in(
            crate::resolve::ReaderState::new(Scheme::Dark, 1.0),
            Layer::default(),
        )
        .unwrap();
    assert_eq!(theme.reader_status(), ReaderStatus::StandIn);
    assert_eq!(theme.reload_reader(), ReaderStatus::StandIn);
    theme.remove_reader_stand_in().unwrap();
    assert_eq!(theme.reader_status(), ReaderStatus::NoReader);
}

/// A GNOME reading over `adwaita` (S9.3, S9.5): it validates as the reader
/// layer, its state is the system's, the scheme in force its scheme, and
/// the resolved theme shows its accent (libadwaita's blue, the portal's
/// colour classified), its icon theme and its `body` font, converted by
/// adwaita's 96 DPI and scaled by the text-scaling factor (S9.4).
#[cfg(all(target_os = "linux", feature = "portal"))]
#[test]
fn a_gnome_reading_over_adwaita() {
    use ashpd::desktop::settings::{ColorScheme, Contrast as PortalContrast, ReducedMotion};

    let mut theme = synthetic_adwaita();
    let reading = crate::gnome::build_reading(
        Some(ColorScheme::PreferDark),
        Some(ashpd::desktop::Color::new(0.25, 0.5, 0.75)),
        PortalContrast::High,
        Some(ReducedMotion::ReducedMotion),
        Some("wayland".to_string()),
        |_: &str, key: &str| match key {
            "icon-theme" => Some("Papirus".to_string()),
            "text-scaling-factor" => Some("1.25".to_string()),
            "enable-animations" => Some("false".to_string()),
            "overlay-scrolling" => Some("true".to_string()),
            "font-name" => Some("Adwaita Sans Bold 12".to_string()),
            _ => None,
        },
        PresetId::Adwaita,
    );
    let status = theme.take(reading, PresetId::Adwaita).unwrap();
    assert_eq!(status, ReaderStatus::Read);
    assert_eq!(theme.scheme(), Scheme::Dark);
    let system = theme.system().unwrap();
    assert_eq!(system.contrast, Some(crate::inputs::Contrast::High));
    assert!(system.reduced_motion && system.animations_off && system.overlay_scrollbars);
    assert_eq!(system.text_scaling_factor, 1.25);
    let resolved = theme
        .resolve(
            &mut WindowState::default(),
            &WindowConditions::new(InputKind::Mouse),
        )
        .theme;
    let scope = resolved.root_scope(&root());
    let tokens = resolved.tokens(&scope).unwrap();
    assert_eq!(tokens.icons.theme.as_deref(), Some("Papirus"));
    let accent = resolved.role(
        &scope,
        crate::blocks::part_style::RoleRef::Global(crate::roles::GlobalRole::Accent),
    );
    assert_eq!(
        accent.map(crate::color::Color::to_srgb8_clamped),
        Some([0x35, 0x84, 0xe4, 0xff])
    );
    let body = tokens.text.roles.body.as_ref().unwrap().font.clone();
    assert_eq!(body.family.as_deref(), Some("Adwaita Sans"));
    assert_eq!(body.weight, Some(700));
    // 12 pt at 96 DPI is 16 px, scaled by 1.25.
    assert_eq!(body.size, Some(crate::units::Px(20.0)));
}

/// A portal-only reading over `adwaita` (S16's *Which reader*): it
/// validates as the reader layer, its scheme and conditions are the
/// system's, and the resolved theme shows its classified accent.
#[cfg(all(target_os = "linux", feature = "portal"))]
#[test]
fn a_portal_only_reading_over_adwaita() {
    use ashpd::desktop::settings::Contrast as PortalContrast;

    let mut theme = synthetic_adwaita();
    let reading = crate::gnome::portal::build_reading(
        None,
        Some(ashpd::desktop::Color::new(0.9, 0.1, 0.1)),
        PortalContrast::NoPreference,
        None,
        Some("Adwaita:dark"),
        |_: &str, key: &str| (key == "enable-animations").then(|| "false".to_string()),
        || None,
    );
    let status = theme.take(reading, PresetId::Adwaita).unwrap();
    assert_eq!(status, ReaderStatus::Read);
    assert_eq!(theme.scheme(), Scheme::Dark);
    let system = theme.system().unwrap();
    assert!(system.animations_off && !system.reduced_motion);
    let resolved = theme
        .resolve(
            &mut WindowState::default(),
            &WindowConditions::new(InputKind::Mouse),
        )
        .theme;
    let scope = resolved.root_scope(&root());
    let accent = resolved.role(
        &scope,
        crate::blocks::part_style::RoleRef::Global(crate::roles::GlobalRole::Accent),
    );
    // libadwaita's red, `#e62d42`.
    assert_eq!(
        accent.map(crate::color::Color::to_srgb8_clamped),
        Some([0xe6, 0x2d, 0x42, 0xff])
    );
}

/// The colour `role` resolves to at the root of `theme`, as 8-bit sRGB.
fn root_role(theme: &super::Theme, role: crate::blocks::part_style::RoleRef) -> Option<[u8; 4]> {
    let resolved = theme
        .resolve(
            &mut WindowState::default(),
            &WindowConditions::new(InputKind::Mouse),
        )
        .theme;
    let scope = resolved.root_scope(&root());
    resolved
        .role(&scope, role)
        .map(crate::color::Color::to_srgb8_clamped)
}

/// `kde_accent` and `kde_accent_wm` resolved over `kde-breeze`'s own scheme
/// inputs at light with the accent `#e5a50a` give what Plasma wrote for
/// that pair (`docs/facts/kde/accent.toml`'s light-e5a50a facts:
/// `role.view.link` `#a77807`, `role.selection.background` `#ecbf53`,
/// `role.@wm_active_background` `#e3e5e7`), stated by an application layer
/// until the accent-derivation layer states them (S9.2, M1b). The group
/// `active` is written bare (S2.2, ADD-A143). The layer states
/// `token.kde.tint_factor` `none` itself, Breeze Light's (no `TintFactor`):
/// an operand both calls read, which S12's absent-operand check passes, as
/// `None` is a stated value, not an absent one (S4.8).
#[test]
fn kde_accent_over_kde_breeze() {
    use crate::blocks::part_style::RoleRef;
    use crate::color::Color;
    use crate::inputs::{Accent, InputValue};
    use crate::roles::{SurfaceRole, SurfaceSet};
    use crate::value::{SchemeMap, Value};

    let mut theme = synthetic_theme(PresetId::KdeBreeze);
    let accent = Color::parse("#e5a50a").unwrap();
    let accent = InputValue::new(SchemeMap::one(Scheme::Light, Value::Stated(accent)));
    theme.set_accent(Some(Accent::new(accent))).unwrap();
    let view_link = || RoleRef::Set(SurfaceSet::View, SurfaceRole::Link);
    assert_eq!(
        root_role(&theme, view_link()),
        Some([0x29, 0x80, 0xb9, 0xff]),
        "the preset's own link, no accent derived"
    );
    theme
        .add_layer(
            Layer::from_toml(
                "[colors.light.surfaces.view]\n\
                 link = \"= kde_accent(view, foreground_link, active)\"\n\
                 [colors.light.surfaces.selection]\n\
                 background = \"= kde_accent(selection, background_normal, active)\"\n\
                 [colors.light.keyed]\n\
                 wm_active_background = \"= kde_accent_wm(active_background)\"\n\
                 [tokens.kde]\n\
                 tint_factor = \"none\"\n",
            )
            .unwrap(),
        )
        .unwrap();
    assert_eq!(
        root_role(&theme, view_link()),
        Some([0xa7, 0x78, 0x07, 0xff])
    );
    assert_eq!(
        root_role(
            &theme,
            RoleRef::Set(SurfaceSet::Selection, SurfaceRole::Background)
        ),
        Some([0xec, 0xbf, 0x53, 0xff])
    );
    assert_eq!(
        root_role(&theme, RoleRef::Keyed(crate::key!("wm_active_background"))),
        Some([0xe3, 0xe5, 0xe7, 0xff])
    );
}

/// A macOS reading over `macos-tahoe` (S9.3, S9.5): it validates as the
/// reader layer, its state is the system's, and the resolved theme shows
/// what it read under the scheme in force: the accent, a role it read, a
/// keyed system colour, and `role.@selected_control`, which the preset
/// states as its input.
#[test]
fn a_macos_reading_over_macos_tahoe() {
    use crate::blocks::part_style::RoleRef;
    use crate::macos::tests::{DARK, mac_data, synthetic};
    use crate::roles::{GlobalRole, SurfaceRole, SurfaceSet};

    let mut theme = synthetic_theme(PresetId::MacosTahoe);
    let data = crate::macos::MacosData {
        scheme: Scheme::Dark,
        overlay_scrollbars: Some(true),
        ..mac_data()
    };
    let reading = crate::macos::build_reading(&data, PresetId::MacosTahoe);
    let status = theme.take(reading, PresetId::MacosTahoe).unwrap();
    assert_eq!(status, ReaderStatus::Read);
    assert_eq!(theme.scheme(), Scheme::Dark);
    assert!(theme.system().unwrap().overlay_scrollbars);
    let read = |n: u8| Some(synthetic(DARK + n).to_srgb8_clamped());
    assert_eq!(
        root_role(&theme, RoleRef::Global(GlobalRole::Accent)),
        read(1)
    );
    assert_eq!(
        root_role(
            &theme,
            RoleRef::Set(SurfaceSet::Window, SurfaceRole::Background)
        ),
        read(6)
    );
    assert_eq!(
        root_role(&theme, RoleRef::Keyed(crate::key!("system_red"))),
        read(16)
    );
    assert_eq!(
        root_role(&theme, RoleRef::Keyed(crate::key!("selected_control"))),
        read(5)
    );
}

/// Under Increase Contrast a macOS reading's rules and contrast bases show
/// under the increased contrast it reports, over the preset's own
/// increased-contrast rules (S9.3).
#[test]
fn a_macos_reading_under_increased_contrast() {
    use crate::blocks::part_style::RoleRef;
    use crate::macos::tests::{LIGHT, mac_data, synthetic};
    use crate::roles::{GlobalRole, SurfaceRole, SurfaceSet};

    let mut theme = synthetic_theme(PresetId::MacosTahoe);
    let data = crate::macos::MacosData {
        increase_contrast: true,
        ..mac_data()
    };
    let reading = crate::macos::build_reading(&data, PresetId::MacosTahoe);
    assert_eq!(
        theme.take(reading, PresetId::MacosTahoe).unwrap(),
        ReaderStatus::Read
    );
    assert_eq!(
        theme.system().unwrap().contrast,
        Some(crate::inputs::Contrast::Increased)
    );
    let read = |n: u8| Some(synthetic(LIGHT + n).to_srgb8_clamped());
    assert_eq!(
        root_role(&theme, RoleRef::Global(GlobalRole::Accent)),
        read(1)
    );
    assert_eq!(
        root_role(
            &theme,
            RoleRef::Set(SurfaceSet::Window, SurfaceRole::Foreground)
        ),
        read(7)
    );
    assert_eq!(
        root_role(&theme, RoleRef::Keyed(crate::key!("system_blue"))),
        read(19)
    );
}

/// A macOS reading over another preset (S9.5): the accent, its common
/// part, applies; its own part, macOS's roles, does not.
#[test]
fn a_macos_reading_over_adwaita_takes_its_common_part() {
    use crate::blocks::part_style::RoleRef;
    use crate::macos::tests::{LIGHT, mac_data, synthetic};
    use crate::roles::{GlobalRole, SurfaceRole, SurfaceSet};

    let mut theme = synthetic_adwaita();
    let background = RoleRef::Set(SurfaceSet::Window, SurfaceRole::Background);
    let before = root_role(&theme, background.clone());
    let reading = crate::macos::build_reading(&mac_data(), PresetId::MacosTahoe);
    assert_eq!(
        theme.take(reading, PresetId::MacosTahoe).unwrap(),
        ReaderStatus::Read
    );
    assert_eq!(
        root_role(&theme, RoleRef::Global(GlobalRole::Accent)),
        Some(synthetic(LIGHT + 1).to_srgb8_clamped())
    );
    assert_eq!(root_role(&theme, background), before);
}

/// A KDE reading over `kde-breeze` (S9.3, S9.5): with its scheme file it
/// validates as the reader layer, its state is the system's, and the
/// resolved theme shows its results at the scheme it read: a colour group's
/// role, the `[WM]` fallback's reference, its common accent; and its
/// scheme inputs where the preset's roles reference them.
#[cfg(all(target_os = "linux", feature = "kde"))]
#[test]
fn a_kde_reading_over_kde_breeze() {
    use crate::blocks::part_style::RoleRef;
    use crate::kde::tests::{KDEGLOBALS_DARK, SCHEME_FILE, data, ini, rgb};
    use crate::kde::{SchemeFile, build_reading};
    use crate::roles::{GlobalRole, SurfaceRole, SurfaceSet};

    let mut theme = synthetic_theme(PresetId::KdeBreeze);
    let reading = build_reading(
        &data(KDEGLOBALS_DARK, SchemeFile::Read(ini(SCHEME_FILE))),
        PresetId::KdeBreeze,
    );
    let status = theme.take(reading, PresetId::KdeBreeze).unwrap();
    assert_eq!(status, ReaderStatus::Read);
    assert_eq!(theme.scheme(), Scheme::Dark);
    let read = |color: crate::color::Color| Some(color.to_srgb8_clamped());
    assert_eq!(
        root_role(
            &theme,
            RoleRef::Set(SurfaceSet::View, SurfaceRole::Background)
        ),
        read(rgb(10, 11, 12))
    );
    assert_eq!(
        root_role(&theme, RoleRef::Keyed(crate::key!("wm_active_background"))),
        read(rgb(40, 41, 42))
    );
    assert_eq!(
        root_role(&theme, RoleRef::Global(GlobalRole::Accent)),
        root_role(
            &theme,
            RoleRef::Set(SurfaceSet::Selection, SurfaceRole::Background)
        )
    );
    // kdeglobals states no `[Colors:View] ForegroundLink`: the preset's role
    // references the scheme input, which the scheme file leaves unset, so
    // Breeze's dark value shows.
    assert!(root_role(&theme, RoleRef::Set(SurfaceSet::View, SurfaceRole::Link)).is_some());
}

/// A KDE reading whose scheme file is unreadable validates too, its status
/// the fallback naming the failure (S9.3).
#[cfg(all(target_os = "linux", feature = "kde"))]
#[test]
fn a_kde_reading_without_its_scheme_file() {
    use crate::kde::tests::{KDEGLOBALS_DARK, data};
    use crate::kde::{SchemeFile, build_reading};

    let mut theme = synthetic_theme(PresetId::KdeBreeze);
    let why = "the colour scheme `Synthetic` has no readable scheme file".to_string();
    let reading = build_reading(
        &data(KDEGLOBALS_DARK, SchemeFile::Unreadable(why.clone())),
        PresetId::KdeBreeze,
    );
    assert_eq!(
        theme.take(reading, PresetId::KdeBreeze).unwrap(),
        ReaderStatus::Fallback(crate::error::ReaderFailure::Platform(why))
    );
}

/// A KDE reading over another preset (S9.5): its common part applies, the
/// accent and the `body` font; its own part, KDE's results, does not.
#[cfg(all(target_os = "linux", feature = "kde"))]
#[test]
fn a_kde_reading_over_adwaita_takes_its_common_part() {
    use crate::blocks::part_style::RoleRef;
    use crate::kde::tests::{KDEGLOBALS_DARK, data, rgb};
    use crate::kde::{SchemeFile, build_reading};
    use crate::roles::{GlobalRole, SurfaceRole, SurfaceSet};

    let mut theme = synthetic_adwaita();
    theme.pick(Some(Scheme::Dark)).unwrap();
    let background = RoleRef::Set(SurfaceSet::View, SurfaceRole::Background);
    let before = root_role(&theme, background.clone());
    let text = format!("{KDEGLOBALS_DARK}\n[General]\nAccentColor=1,2,3\n");
    let reading = build_reading(&data(&text, SchemeFile::Unnamed), PresetId::KdeBreeze);
    assert_eq!(
        theme.take(reading, PresetId::KdeBreeze).unwrap(),
        ReaderStatus::Read
    );
    assert_eq!(
        root_role(&theme, RoleRef::Global(GlobalRole::Accent)),
        Some(rgb(1, 2, 3).to_srgb8_clamped())
    );
    assert_eq!(root_role(&theme, background), before);
    let resolved = theme
        .resolve(
            &mut WindowState::default(),
            &WindowConditions::new(InputKind::Mouse),
        )
        .theme;
    let scope = resolved.root_scope(&root());
    let tokens = resolved.tokens(&scope).unwrap();
    let body = tokens.text.roles.body.as_ref().unwrap().font.clone();
    assert_eq!(body.family.as_deref(), Some("Synthetic Sans"));
}

/// A Windows reading over `windows-11` (S9.3, S9.5): it validates as the
/// reader layer, its state is the system's, and the resolved theme shows
/// its accent and, through the preset's references, its ramp.
#[test]
fn a_windows_reading_over_windows_11() {
    use crate::blocks::part_style::RoleRef;
    use crate::roles::GlobalRole;
    use crate::windows::tests::{data, rgb};

    let mut theme = synthetic_theme(PresetId::Windows11);
    let reading = crate::windows::build_reading(&data(), PresetId::Windows11);
    assert_eq!(
        theme.take(reading, PresetId::Windows11).unwrap(),
        ReaderStatus::Read
    );
    assert_eq!(theme.scheme(), Scheme::Dark);
    assert_eq!(
        root_role(&theme, RoleRef::Global(GlobalRole::Accent)),
        Some(rgb(1, 2, 3).to_srgb8_clamped())
    );
    // The dark scheme's accent fill is the ramp's light2.
    assert_eq!(
        root_role(&theme, RoleRef::Keyed(crate::key!("accent_fill_default"))),
        Some(rgb(14, 15, 16).to_srgb8_clamped())
    );
}

/// Under high contrast a Windows reading's system colours show through the
/// preset's high-contrast rules (ADD-A135).
#[test]
fn a_windows_reading_under_high_contrast() {
    use crate::blocks::part_style::RoleRef;
    use crate::windows::tests::{data, rgb, system_colors};

    let mut theme = synthetic_theme(PresetId::Windows11);
    let reading = crate::windows::build_reading(
        &crate::windows::WindowsData {
            high_contrast: Some(true),
            system_colors: Some(system_colors()),
            ..data()
        },
        PresetId::Windows11,
    );
    assert_eq!(
        theme.take(reading, PresetId::Windows11).unwrap(),
        ReaderStatus::Read
    );
    assert_eq!(
        theme.system().unwrap().contrast,
        Some(crate::inputs::Contrast::High)
    );
    assert_eq!(
        root_role(&theme, RoleRef::Keyed(crate::key!("system_window_text"))),
        Some(rgb(101, 101, 101).to_srgb8_clamped())
    );
    assert_eq!(
        root_role(&theme, RoleRef::Keyed(crate::key!("text_fill_primary"))),
        Some(rgb(101, 101, 101).to_srgb8_clamped())
    );
}

/// A Windows reading over another preset (S9.5): over `adwaita`, whose one
/// accent input is the accent, the accent applies; over `macos-tahoe`, which
/// declares derived accent inputs, the reader's accent is left out.
#[test]
fn a_windows_reading_over_other_presets_takes_its_common_part() {
    use crate::blocks::part_style::RoleRef;
    use crate::roles::GlobalRole;
    use crate::windows::tests::{data, rgb};

    let accent = RoleRef::Global(GlobalRole::Accent);
    let mut theme = synthetic_adwaita();
    let reading = crate::windows::build_reading(&data(), PresetId::Windows11);
    assert_eq!(
        theme.take(reading, PresetId::Windows11).unwrap(),
        ReaderStatus::Read
    );
    assert_eq!(
        root_role(&theme, accent.clone()),
        Some(rgb(1, 2, 3).to_srgb8_clamped())
    );

    let mut theme = synthetic_theme(PresetId::MacosTahoe);
    theme.pick(Some(Scheme::Dark)).unwrap();
    let before = root_role(&theme, accent.clone());
    let reading = crate::windows::build_reading(&data(), PresetId::Windows11);
    assert_eq!(
        theme.take(reading, PresetId::Windows11).unwrap(),
        ReaderStatus::Read
    );
    assert_eq!(root_role(&theme, accent), before);
}
