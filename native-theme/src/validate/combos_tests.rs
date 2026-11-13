//! S12's checks over combinations, one `combination_<item>` test per item:
//! a stack that triggers exactly that problem, and a near miss that does
//! not; S12's own examples among them. Every number and name here is
//! synthetic.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::{Context, LayerKind, Reused, Stacked, check};
use crate::declare::Declarations;
use crate::error::{LayerError, Problem};
use crate::layer::Layer;
use crate::toml::layer::read_preset;

const PRESET_HEAD: &str = "[preset]\nid = \"probe\"\ngeneration = \"probe-1\"\n";

const NONE: [&str; 0] = [];

/// The declarations a `[declare]` table's body states.
fn declare(body: &str) -> Declarations {
    read_preset(&format!("{PRESET_HEAD}[declare]\n{body}"))
        .unwrap()
        .declarations
}

fn toml(text: &str) -> Layer {
    Layer::from_toml(text).unwrap()
}

/// The problems of `top` added as an application layer over `below`, a
/// preset's layer, under `decl`.
fn over(below: &str, top: &str, decl: &Declarations) -> Vec<String> {
    let below = toml(below);
    let below = [Stacked {
        layer: &below,
        kind: LayerKind::Preset,
        left_out: None,
    }];
    let cx = Context {
        declarations: decl,
        preset: None,
        kind: LayerKind::Application,
        reader_accent_left_out: false,
        below: &below,
    };
    match check(&[&toml(top)], &cx, &Reused::default()) {
        Ok(()) => Vec::new(),
        Err(LayerError::Invalid(problems)) => problems.iter().map(Problem::to_string).collect(),
        Err(other) => panic!("{other}"),
    }
}

/// The problems of `top` alone, an application layer over nothing.
fn alone(top: &str, decl: &Declarations) -> Vec<String> {
    over("", top, decl)
}

fn bare(top: &str) -> Vec<String> {
    alone(top, &Declarations::default())
}

/// An edge's `dash` absent where its `width` and `paint` are drawn (S2.4,
/// S12); absent with its width, it is an edge not drawn.
#[test]
fn combination_edge_dash_absent() {
    let base = "[widgets.button.body.stroke.top]\nwidth_px = 1\npaint = \"#000000\"\n\
                dash = \"solid\"\nsnap = \"none\"";
    let rule = |set: &str| {
        format!(
            "[[rules]]\nwhen = {{ states = [\"hover\"] }}\ntarget = \"button.body\"\nset = {set}"
        )
    };
    assert_eq!(
        over(
            base,
            &rule("{ stroke.top.dash = \"absent\" }"),
            &Declarations::default()
        ),
        ["rules[0].set.stroke.top.dash: may not be absent"]
    );
    assert_eq!(
        over(
            base,
            &rule("{ stroke.top.dash = \"absent\", stroke.top.width_px = \"absent\" }"),
            &Declarations::default()
        ),
        NONE
    );
}

/// S12's own example: one rule stating `rendering_mode = original` with
/// `fill = "absent"` passes, as the fill is absent only where the mode is
/// `Original`; a rule leaving the mode as it was fails.
#[test]
fn combination_icon_fill_absent() {
    let base = "[widgets.button.icon.icon]\nsize_px = 16\nfill = \"#000000\"\n\
                rendering_mode = \"template\"";
    let rule = |set: &str| {
        format!(
            "[[rules]]\nwhen = {{ states = [\"hover\"] }}\ntarget = \"button.icon\"\nset = {set}"
        )
    };
    assert_eq!(
        over(
            base,
            &rule("{ icon = { rendering_mode = \"original\", fill = \"absent\" } }"),
            &Declarations::default()
        ),
        NONE
    );
    assert_eq!(
        over(
            base,
            &rule("{ icon.fill = \"absent\" }"),
            &Declarations::default()
        ),
        ["rules[0].set.icon.fill: may not be absent"]
    );
}

/// `FromIcon` recolours the paths the icon marks (S2.9), so an icon style's
/// `fill` absent under it is refused as under any mode but `Original`
/// (S12), whether the mode is stated beneath or beside it.
#[test]
fn from_icon_without_fill_refused() {
    let base = "[widgets.button.icon.icon]\nsize_px = 16\nfill = \"#000000\"\n\
                rendering_mode = \"from_icon\"";
    let rule = |set: &str| {
        format!(
            "[[rules]]\nwhen = {{ states = [\"hover\"] }}\ntarget = \"button.icon\"\nset = {set}"
        )
    };
    assert_eq!(
        over(
            base,
            &rule("{ icon.fill = \"absent\" }"),
            &Declarations::default()
        ),
        ["rules[0].set.icon.fill: may not be absent"]
    );
    assert_eq!(
        over(
            base,
            &rule("{ icon = { rendering_mode = \"from_icon\", fill = \"absent\" } }"),
            &Declarations::default()
        ),
        ["rules[0].set.icon.fill: may not be absent"]
    );
    assert_eq!(
        over(
            base,
            &rule("{ icon = { rendering_mode = \"original\", fill = \"absent\" } }"),
            &Declarations::default()
        ),
        NONE
    );
}

const SYSTEM_CONTROL: &str =
    "source = { system_control = { control = \"app_kit_checkbox\", replaces = [] } }";

const THEME_ICON: &str = "source = { theme_icon = { name = \"probe\", fallbacks = [] } }";

/// A glyph's `fill` absent where its `stroke` is absent too (S2.9, S12),
/// on a source other than `SystemControl`
/// (`glyph_fill_absent_on_system_control`).
#[test]
fn combination_glyph_fill_absent() {
    let base = format!(
        "[widgets.button.menu_indicator.glyph]\nsize_px = 8\nfill = \"#000000\"\n\
         stroke = \"absent\"\n{THEME_ICON}"
    );
    let rule = "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.menu_indicator\"\n\
                set = { glyph.fill = \"absent\" }";
    assert_eq!(
        over(&base, rule, &Declarations::default()),
        ["rules[0].set.glyph.fill: may not be absent"]
    );
    let stroked = format!(
        "[widgets.button.menu_indicator.glyph]\nsize_px = 8\nfill = \"#000000\"\n\
         stroke = {{ paint = \"#000000\", width_px = 1, cap = \"round\", join = \"round\" }}\n\
         {THEME_ICON}"
    );
    assert_eq!(over(&stroked, rule, &Declarations::default()), NONE);
    // The T7 review's M3: with no source stated, the missing source is S12's
    // missing field alone, and neither conditional mark refuses.
    let sourceless = "[widgets.button.menu_indicator.glyph]\nsize_px = 8\nfill = \"#000000\"\n\
                      stroke = \"absent\"\n";
    assert_eq!(over(sourceless, rule, &Declarations::default()), NONE);
}

/// A glyph's `size` absent where its `source` can be other than
/// `SystemControl` (S6.3, S12): a `SystemControl` source's rendering never
/// draws it; a rule changing the source under a state is refused with it.
#[test]
fn glyph_size_absent_only_on_system_control() {
    let base = |source: &str| {
        format!(
            "[widgets.button.menu_indicator.glyph]\nsize_px = 8\nfill = \"#000000\"\n\
             stroke = \"absent\"\n{source}"
        )
    };
    let rule = "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.menu_indicator\"\n\
                set = { glyph.size_px = \"absent\" }";
    assert_eq!(
        over(&base(SYSTEM_CONTROL), rule, &Declarations::default()),
        NONE
    );
    assert_eq!(
        over(&base(THEME_ICON), rule, &Declarations::default()),
        ["rules[0].set.glyph.size_px: may not be absent"]
    );
    let source = format!(
        "{rule}\n[[rules]]\nwhen = {{ states = [\"pressed\"] }}\n\
         target = \"button.menu_indicator\"\nset = {{ glyph.{THEME_ICON} }}"
    );
    assert_eq!(
        over(&base(SYSTEM_CONTROL), &source, &Declarations::default()),
        ["rules[0].set.glyph.size_px: may not be absent"]
    );
}

/// A glyph's `fill` absent beside an absent stroke is allowed on a
/// `SystemControl` source (S6.3, S12), and refused on any other
/// (`combination_glyph_fill_absent`).
#[test]
fn glyph_fill_absent_on_system_control() {
    let base = |source: &str| {
        format!(
            "[widgets.button.menu_indicator.glyph]\nsize_px = 8\nfill = \"#000000\"\n\
             stroke = \"absent\"\n{source}"
        )
    };
    let rule = "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.menu_indicator\"\n\
                set = { glyph.fill = \"absent\" }";
    assert_eq!(
        over(&base(SYSTEM_CONTROL), rule, &Declarations::default()),
        NONE
    );
    assert_eq!(
        over(&base(THEME_ICON), rule, &Declarations::default()),
        ["rules[0].set.glyph.fill: may not be absent"]
    );
}

/// A ripple's `color`, `alpha` and `padding` may be absent only where its
/// `effect` is (S2.12, S6.3, S12): with no effect stated under any
/// combination they are accepted absent; a rule stating the effect, or
/// one making them absent beside a stated effect, is refused.
#[test]
fn ripple_color_absent_only_without_effect() {
    let base = |effect: &str, rest: &str| {
        format!(
            "[widgets.button.body.interaction]\n\
             state_layer = {{ color = \"#000000\", alpha = 0.1 }}\n\
             ripple = {{ effect = {effect}, bounded = true, max_radius_px = \"absent\", \
             indication_box = \"none\", {rest} }}"
        )
    };
    let absent = "color = \"absent\", alpha = \"absent\", padding_px = \"absent\"";
    let stated = "color = \"#000000\", alpha = 0.1, padding_px = 0";
    let rule = |set: &str| {
        format!(
            "[[rules]]\nwhen = {{ states = [\"pressed\"] }}\ntarget = \"button.body\"\n\
             set = {set}"
        )
    };
    let decl = Declarations::default();
    assert_eq!(over(&base("\"absent\"", absent), "", &decl), NONE);
    assert_eq!(
        over(
            &base("\"absent\"", absent),
            &rule("{ interaction.ripple.effect = \"android_patterned\" }"),
            &decl
        ),
        [
            "widgets.button.body.interaction.ripple.color: may not be absent",
            "widgets.button.body.interaction.ripple.alpha: may not be absent",
            "widgets.button.body.interaction.ripple.padding_px: may not be absent",
        ]
    );
    assert_eq!(
        over(
            &base("\"android_patterned\"", stated),
            &rule("{ interaction.ripple.color = \"absent\" }"),
            &decl
        ),
        ["rules[0].set.interaction.ripple.color: may not be absent"]
    );
    assert_eq!(
        over(
            &base("\"android_patterned\"", stated),
            &rule(
                "{ interaction.ripple.effect = \"absent\", interaction.ripple.color = \"absent\" }"
            ),
            &decl
        ),
        NONE
    );
}

/// A `Fields` patch is never refused for what lies below it (S2.2, S6.3,
/// S12, ADD-A126): over a stroke an earlier rule makes absent under any
/// combination, or a reference that resolves `Absent`, it sets nothing; an
/// expression in it reading an absent operand is not evaluated where the
/// block it patches is absent, and is refused where it is drawn.
#[test]
fn fields_over_absent_never_refused() {
    let rules = |first: &str, second: &str| {
        format!(
            "[[rules]]\nwhen = {first}\ntarget = \"button.body\"\nset = {{ stroke = \"absent\" }}\n\
             [[rules]]\nwhen = {second}\ntarget = \"button.body\"\n\
             set = {{ stroke.top.width_px = 2 }}"
        )
    };
    // A rule names the default size alone while the sizes unit is not
    // enabled (S12; ADD-A147).
    let decl = declare(
        "[declare.widget.button]\nsizes = [\"mini\", \"regular\"]\ndefault_size = \"regular\"",
    );
    for (first, second) in [
        (
            "{ conditions = [\"scheme:dark\"] }",
            "{ conditions = [\"scheme:light\"] }",
        ),
        ("{ sizes = [\"regular\"] }", "{ not_sizes = [\"regular\"] }"),
        (
            "{ conditions = [\"scheme:dark\"] }",
            "{ conditions = [\"reduced_motion\"] }",
        ),
        ("{}", "{ states = [\"hover\"] }"),
    ] {
        assert_eq!(alone(&rules(first, second), &decl), NONE);
    }
    // Through a reference to an absent block, a leaf's expression reading
    // an absent operand is not in effect.
    let through = "[widgets.button.subtitle]\ntext = \"absent\"\n\
                   [widgets.button.label]\ntext = \"= widget.subtitle.text\"\n\
                   [widgets.button.body]\ntouch_target_px = \"absent\"";
    let rule = "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.label\"\n\
                set = { text.letter_spacing = \"= add(widget.body.touch_target, 1px)\" }";
    assert_eq!(over(through, rule, &Declarations::default()), NONE);

    // A `Fields` patch over an absent glyph leaves it absent, its leaves
    // too (the menu's check glyph outside its checked states, S6.3).
    let glyph = "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.menu_indicator\"\n\
                 set = { glyph.size_px = 4 }";
    let unglyphed = "[widgets.button.menu_indicator]\nglyph = \"absent\"";
    assert_eq!(over(unglyphed, glyph, &Declarations::default()), NONE);
    assert_eq!(bare(&format!("{unglyphed}\n{glyph}")), NONE);

    // An appended layer's expression over an absent paint is not evaluated
    // (S12): it appends to nothing.
    let layer = "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.body\"\n\
                 set = { fill = { append = [\"= alpha(part.interaction.state_layer.color, 0.5)\"] } }";
    let unfilled = "[widgets.button.body]\ninteraction = \"absent\"\nfill = \"absent\"";
    assert_eq!(over(unfilled, layer, &Declarations::default()), NONE);
    let filled = "[widgets.button.body]\ninteraction = \"absent\"\nfill = \"#000000\"";
    assert_eq!(
        over(filled, layer, &Declarations::default()),
        ["rules[0].set.fill.append[0]: `part.interaction.state_layer.color` can be `absent` here"]
    );

    let reading = "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.body\"\n\
                   set = { focus.width_px = \"= add(part.touch_target, 1px)\" }";
    let absent = "[widgets.button.body]\nfocus = \"absent\"\ntouch_target_px = \"absent\"";
    assert_eq!(over(absent, reading, &Declarations::default()), NONE);
    let drawn = "[widgets.button.body]\ntouch_target_px = \"absent\"";
    assert_eq!(
        over(drawn, reading, &Declarations::default()),
        ["rules[0].set.focus.width_px: `part.touch_target` can be `absent` here"]
    );
}

/// An `Append` over a paint stated by a reference appends to what the
/// reference resolves to, nothing where it resolves `Absent` (S2.2, S6.1):
/// validation follows the reference beneath it, so a text fill reading
/// such a paint is refused; and a paint stated below a reference above it
/// and then appended to is the stated paint, not the referent's `Absent`.
#[test]
fn append_over_a_reference_follows_it() {
    let read = "[widgets.button.label]\ntext.fill = \"= widget.body.fill\"\n\
                [widgets.button.subtitle]\ntext = \"absent\"\n\
                [widgets.button.body]\nfill = \"= widget.subtitle.text.fill\"\n";
    let append = "[[rules]]\nwhen = {}\ntarget = \"button.body\"\n\
                  set = { fill = { append = [\"#ff0000\"] } }";
    assert_eq!(
        bare(&format!("{read}{append}")),
        ["widgets.button.label.text.fill: may not be absent"]
    );

    let decl = declare("text_styles = [\"body\"]");
    let keyed = "[tokens.text.keyed.body]\nfont = { family = \"Probe Sans\", size_px = 12, weight = \
                 400, style = \"normal\", width = \"absent\", axes = {}, features = {} }\n\
                 scaling_role = \"none\"\nletter_spacing_px = 0\nword_spacing_px = 0\n\
                 line_height = \"auto\"\ndecoration = \"absent\"\nmnemonic_decoration = \"absent\"\n\
                 case = \"mixed\"\nelision = \"end\"\nwrap = \"word\"\nmax_lines = \"none\"\n\
                 fill = \"absent\"\n[widgets.button.label]\ntext = \"= token.text.keyed.body\"\n";
    let set = "[[rules]]\nwhen = {}\ntarget = \"button.label\"\nset = { text.fill = \"#000000\" }\n\
               [[rules]]\nwhen = {}\ntarget = \"button.label\"\n\
               set = { text.fill = { append = [\"#ff0000\"] } }";
    assert_eq!(alone(&format!("{keyed}{set}"), &decl), NONE);
}

/// A paint map's entry an `Append` creates has its paint unset (S6.1, S12,
/// ADD-A131): refused where no layer below holds the entry, accepted where
/// one does.
#[test]
fn combination_entry_created_by_append() {
    let decl = declare("symbolic_colors = [\"error\"]");
    let rule = "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.icon\"\n\
                set = { icon.symbolic_colors.error = { append = [\"#ff0000\"] } }";
    assert_eq!(
        alone(rule, &decl),
        ["rules[0].set.icon.symbolic_colors.error: can create the entry with `paint` unset"]
    );
    let held = "[widgets.button.icon.icon.symbolic_colors]\nerror = \"#00ff00\"";
    assert_eq!(over(held, rule, &decl), NONE);
}

/// `DrawnAs` patches forming a cycle (S6.1, S12): a button's icon drawn as
/// a button draws its own icon as a button again; drawn as a button of a
/// variant under which the icon is drawn, it does not.
#[test]
fn combination_drawn_as_cycle() {
    assert_eq!(
        bare("[widgets.button.icon]\ndrawn_as = { widget = \"button\", variants = [] }"),
        ["widgets.button.icon: drawn as itself through `drawn_as`"]
    );
    let decl = declare("[declare.widget.button]\nvariants = [\"@inner\"]");
    let rule = "[[rules]]\nwhen = { not_variants = [\"@inner\"] }\ntarget = \"button.icon\"\n\
                set = { drawn_as = { widget = \"button\", variants = [\"@inner\"] } }";
    assert_eq!(alone(rule, &decl), NONE);
}

/// `text_scale` read where `token.text.scaling` can be `DynamicType` (S2.2,
/// S9.4, S12): under a condition that sets it so, too.
#[test]
fn combination_text_scale() {
    let read = "[widgets.button.body]\nmin_height_px = \"= mul(24px, text_scale)\"";
    let scaling = |scaling: &str| format!("[tokens.text]\nscaling = \"{scaling}\"");
    assert_eq!(
        over(&scaling("dynamic_type"), read, &Declarations::default()),
        [
            "widgets.button.body.min_height_px: `text_scale` where the text scaling can be `dynamic_type`"
        ]
    );
    assert_eq!(
        over(&scaling("linear"), read, &Declarations::default()),
        NONE
    );
    let conditional = format!(
        "{}\n[[rules]]\nwhen = {{ conditions = [\"accessibility_text_size\"] }}\n\
         target = \"tokens\"\nset = {{ text.scaling = \"dynamic_type\" }}",
        scaling("linear")
    );
    assert_eq!(
        over(&conditional, read, &Declarations::default()),
        [
            "widgets.button.body.min_height_px: `text_scale` where the text scaling can be `dynamic_type`"
        ]
    );
}

/// An operand that can resolve `Absent` or `Auto` (S2.2, S12): in a call it
/// is refused, a plain reference bringing either only where its field does
/// not allow it; `line_height` is the line spacing under `auto`. S2.2's own
/// example: `add(part.max_width, 4px)` over an `auto` maximum.
#[test]
fn combination_absent_and_auto_operands() {
    let decl = Declarations::default();
    let body = |text: &str| format!("[widgets.button.body]\n{text}");
    assert_eq!(
        bare(&body(
            "max_width_px = \"auto\"\nmin_width_px = \"= add(part.max_width, 4px)\""
        )),
        ["widgets.button.body.min_width_px: `part.max_width` can be `auto` here"]
    );
    assert_eq!(
        bare(&body(
            "max_width_px = 40\nmin_width_px = \"= add(part.max_width, 4px)\""
        )),
        NONE
    );
    // A plain reference: `min_width` allows `auto`, `padding` does not.
    assert_eq!(
        bare(&body(
            "max_width_px = \"auto\"\nmin_width_px = \"= part.max_width\""
        )),
        NONE
    );
    assert_eq!(
        bare(&body(
            "max_width_px = \"auto\"\npadding.top_px = \"= part.max_width\""
        )),
        ["widgets.button.body.padding.top_px: may not be auto"]
    );
    // An absent margin.
    assert_eq!(
        bare(&body(
            "margin.top_px = \"absent\"\nmin_width_px = \"= add(part.margin.top, 4px)\""
        )),
        ["widgets.button.body.min_width_px: `part.margin.top` can be `absent` here"]
    );
    assert_eq!(
        bare(&body(
            "margin.top_px = \"absent\"\ntouch_target_px = \"= part.margin.top\""
        )),
        NONE
    );
    assert_eq!(
        bare(&body(
            "margin.top_px = \"absent\"\nmin_width_px = \"= part.margin.top\""
        )),
        ["widgets.button.body.min_width_px: may not be absent"]
    );
    // The absent value under a rule only.
    let rule = "[widgets.button.body]\nmargin.top_px = 2\nmin_width_px = \"= add(part.margin.top, \
                4px)\"\n[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.body\"\n\
                set = { margin.top_px = \"absent\" }";
    assert_eq!(
        alone(rule, &decl),
        ["widgets.button.body.min_width_px: `part.margin.top` can be `absent` here"]
    );
    // `line_height` under `auto` is the line spacing in a part's call; a
    // text style token's reading it is refused.
    assert_eq!(
        bare(
            "[widgets.button.label]\ntext.line_height = \"auto\"\n\
             padding.top_px = \"= mul(line_height, 0.5)\""
        ),
        NONE
    );
    let styles = declare("text_styles = [\"body\"]");
    let token = "[tokens.text.keyed.body]\nline_height = \"auto\"\n\
                 letter_spacing = \"= mul(line_height, 0.1)\"";
    assert_eq!(
        over(token, token, &styles),
        ["tokens.text.keyed.body.letter_spacing: `line_height` can be `auto` here"]
    );
}

/// An expression reading itself, and a chain of references longer than 32
/// (S2.2, S12).
#[test]
fn combination_cycles_and_chains() {
    assert_eq!(
        bare(
            "[widgets.button.body]\nmin_width_px = \"= part.min_height\"\n\
             min_height_px = \"= part.min_width\""
        ),
        [
            "widgets.button.body.min_width_px: the expression reads itself",
            "widgets.button.body.min_height_px: the expression reads itself",
        ]
    );
    // Under exclusive conditions the two references never meet.
    let exclusive = "[[rules]]\nwhen = { conditions = [\"scheme:dark\"] }\ntarget = \
                     \"button.body\"\nset = { min_width_px = \"= part.min_height\" }\n\
                     [[rules]]\nwhen = { conditions = [\"scheme:light\"] }\ntarget = \
                     \"button.body\"\nset = { min_height_px = \"= part.min_width\" }";
    assert_eq!(bare(exclusive), NONE);
    let keys = |count: usize| {
        let names: Vec<String> = (0..count).map(|i| format!("\"k{i}\"")).collect();
        declare(&format!(
            "params = {{ layout = {{ unit = \"px\", keys = [{}] }} }}",
            names.join(", ")
        ))
    };
    let chain = |count: usize| {
        let mut text = String::from("[tokens.params.layout]\n");
        for i in 0..count {
            text.push_str(&format!("k{i} = \"= token.params.layout.k{}\"\n", i + 1));
        }
        text.push_str(&format!("k{count} = 4\n"));
        text
    };
    // 33 references from `k0` to the number; 32 from `k1`.
    assert_eq!(
        alone(&chain(33), &keys(34)),
        ["tokens.params.layout.k0: a chain of references longer than 32"]
    );
    assert_eq!(alone(&chain(32), &keys(33)), NONE);
}

/// A `widget.<part>` reference to a part that can be absent as a whole
/// (S2.2, S6.3, S12).
#[test]
fn combination_widget_part_not_drawn() {
    let reads = "[widgets.button.label]\npadding.top_px = \"= widget.menu_indicator.min_height\"\n\
                 [widgets.button.menu_indicator]\nmin_height_px = 4";
    assert_eq!(bare(reads), NONE);
    let absent = format!(
        "{reads}\n[[rules]]\nwhen = {{ states = [\"hover\"] }}\ntarget = \"button.menu_indicator\"\n\
         set = {{ absent = true }}"
    );
    assert_eq!(
        bare(&absent),
        [
            "widgets.button.label.padding.top_px: `widget.menu_indicator` can be absent or drawn as another widget"
        ]
    );
}

/// A corner operand (S2.4, S8, S12): `Concentric` and `FollowPart` are the
/// connector's at layout, refused anywhere; `Full` only on the element's
/// own part, here the root. (`button`'s root is its one part listing
/// corners, so a `Full` corner of another part has no fixture yet.)
#[test]
fn combination_corner_operands() {
    let corners = |corner: &str| {
        format!(
            "[widgets.button.body]\ncorners.top_left = {corner}\n\
             focus.width_px = \"= part.corners.top_left\""
        )
    };
    assert_eq!(
        bare(&corners("{ concentric = { minimum_px = 2 } }")),
        [
            "widgets.button.body.focus.width_px: `part.corners.top_left` is a corner computed at layout"
        ]
    );
    assert_eq!(bare(&corners("\"follow_part\"")).len(), 2);
    let sized = declare("[declare.widget.button]\ndata = { body = [\"width\", \"height\"] }");
    assert_eq!(
        alone(&corners("{ full = { maximum_px = \"absent\" } }"), &sized),
        NONE
    );
    let px = "[widgets.button.body]\ncorners.top_left_px = 4\n\
              focus.width_px = \"= part.corners.top_left\"";
    assert_eq!(bare(px), NONE);
}

/// A focus indicator's `FollowPart` corner whose sides' offsets can
/// differ, in either direction (S2.4, S2.10, S12): the top-left corner's
/// top offset against its left one, and its right one under `rtl`.
#[test]
fn combination_follow_part_offsets() {
    let focus = |offsets: &str| {
        format!("[widgets.button.body.focus]\ncorners.top_left = \"follow_part\"\n{offsets}")
    };
    assert_eq!(
        bare(&focus(
            "offset.top_px = 2\noffset.left_px = 3\noffset.right_px = 2"
        )),
        [
            "widgets.button.body.focus.corners.top_left: `follow_part` on `top_left`, whose sides' offsets can differ"
        ]
    );
    assert_eq!(
        bare(&focus(
            "offset.top_px = 2\noffset.left_px = 2\noffset.right_px = 3"
        )),
        [
            "widgets.button.body.focus.corners.top_left: `follow_part` on `top_left`, whose sides' offsets can differ"
        ]
    );
    assert_eq!(
        bare(&focus(
            "offset.top_px = 2\noffset.left_px = 2\noffset.right_px = 2"
        )),
        NONE
    );
    // Equal at rest, different under a rule.
    let rule = format!(
        "{}\n[[rules]]\nwhen = {{ states = [\"pressed\"] }}\ntarget = \"button.body\"\n\
         set = {{ focus.offset.left_px = 1 }}",
        focus("offset.top_px = 2\noffset.left_px = 2\noffset.right_px = 2")
    );
    assert_eq!(
        bare(&rule),
        [
            "widgets.button.body.focus.corners.top_left: `follow_part` on `top_left`, whose sides' offsets can differ"
        ]
    );
}

/// Offsets whose expressions differ in their text but read, through
/// references, the one value under every combination are equal (S12:
/// "references and rules being followed as the query follows them"):
/// adwaita's ring, `token.focus`'s sides reading its top and a part's
/// sides each reading the token's own side; a reference to a value that
/// differs by side still differs.
#[test]
fn combination_follow_part_offsets_through_references() {
    let token = "[tokens.focus]\nwidth_px = 2\ncorners.top_left = \"follow_part\"\n\
                 offset.top_px = \"= sub(0px, token.focus.width)\"\n\
                 offset.right_px = \"= token.focus.offset.top\"\n\
                 offset.bottom_px = \"= token.focus.offset.top\"\n\
                 offset.left_px = \"= token.focus.offset.top\"\n";
    let part = |sides: [&str; 4]| {
        let [top, right, bottom, left] = sides.map(|side| {
            format!("\"= add(add(token.focus.offset.{side}, token.focus.width), 4px)\"")
        });
        format!(
            "{token}[widgets.button.body]\nfocus = \"= token.focus\"\n\
             [[rules]]\nwhen = {{ states = [\"pressed\"] }}\ntarget = \"button.body\"\n\
             set = {{ focus.offset = {{ top_px = {top}, right_px = {right}, bottom_px = {bottom}, \
             left_px = {left} }} }}"
        )
    };
    assert_eq!(bare(token), NONE);
    assert_eq!(bare(&part(["top", "right", "bottom", "left"])), NONE);
    let unequal = token.replace(
        "offset.left_px = \"= token.focus.offset.top\"",
        "offset.left_px = 3",
    );
    assert_eq!(
        bare(&part(["top", "right", "bottom", "left"]).replace(token, &unequal)),
        [
            "tokens.focus.corners.top_left: `follow_part` on `top_left`, whose sides' offsets can differ"
        ]
    );
}

const UNEQUAL_OFFSETS: [&str; 1] = [
    "widgets.button.body.focus.corners.top_left: `follow_part` on `top_left`, whose sides' offsets can differ",
];

/// The problems of the button's body's `FollowPart` top-left corner whose
/// top offset reads `top` and whose left and right ones read `sides`, the
/// label's and the subtitle's text as `texts` states them (each table's
/// keys below `text.`), the body's top and left padding 12px and its
/// bottom padding `= add(1px, 1px)`.
fn follow_part_reading(texts: [&str; 2], top: &str, sides: &str) -> Vec<String> {
    let decl = declare(
        "[declare.widget.button]\ndata = { body = [\"widget.label.font_size\", \
         \"widget.label.line_height\", \"widget.subtitle.font_size\", \
         \"widget.subtitle.line_height\"] }",
    );
    let [label, subtitle] = texts;
    alone(
        &format!(
            "[widgets.button.body]\npadding.top_px = 12\npadding.left_px = 12\n\
             padding.bottom_px = \"= add(1px, 1px)\"\n\
             [widgets.button.label.text]\n{label}\n\
             [widgets.button.subtitle.text]\n{subtitle}\n\
             [widgets.button.body.focus]\ncorners.top_left = \"follow_part\"\n\
             offset.top_px = \"= {top}\"\noffset.left_px = \"= {sides}\"\n\
             offset.right_px = \"= {sides}\""
        ),
        &decl,
    )
}

/// Offsets reading one stated number in two units differ (S2.4, S9.4,
/// S12): a font size of 12pt against a line height of 12px, 16px and 12px
/// at 96 DPI, read plainly or inside a call; a font size in points read on
/// every side still reads equal.
#[test]
fn combination_follow_part_offsets_through_units() {
    let texts = ["font.size_pt = 12\nline_height_px = 12", ""];
    assert_eq!(
        follow_part_reading(texts, "widget.label.font_size", "widget.label.line_height"),
        UNEQUAL_OFFSETS
    );
    assert_eq!(
        follow_part_reading(
            texts,
            "add(widget.label.font_size, 2px)",
            "add(widget.label.line_height, 2px)"
        ),
        UNEQUAL_OFFSETS
    );
    assert_eq!(
        follow_part_reading(
            texts,
            "add(widget.label.font_size, 2px)",
            "add(widget.label.font_size, 2px)"
        ),
        NONE
    );
    assert_eq!(
        follow_part_reading(texts, "widget.label.font_size", "widget.label.font_size"),
        NONE
    );
}

/// Offsets reading one stated number of pixels as a font size and as a
/// length differ (S2.4, S9.4, S12): the text scaling scales only the font
/// size; two lengths still read equal.
#[test]
fn combination_follow_part_offsets_through_text_scaling() {
    let texts = ["font.size_px = 12", ""];
    assert_eq!(
        follow_part_reading(texts, "widget.label.font_size", "part.padding.top"),
        UNEQUAL_OFFSETS
    );
    assert_eq!(
        follow_part_reading(texts, "part.padding.left", "part.padding.top"),
        NONE
    );
}

/// Offsets reading one stated number of pixels in two kinds of font size
/// differ (S2.4, S9.4, S12): `WinUi` scales a font size and leaves a line
/// height; and, read conservatively, two text styles' sizes or line heights
/// differ, as `DynamicType` scales each by its style's role and
/// `AndroidNonLinear` a line height by its style's font size. One line
/// height read on every side still reads equal.
#[test]
fn combination_follow_part_offsets_through_scaling_kinds() {
    let texts = [
        "font.size_px = 12\nline_height_px = 12",
        "font.size_px = 12\nline_height_px = 12",
    ];
    assert_eq!(
        follow_part_reading(texts, "widget.label.font_size", "widget.label.line_height"),
        UNEQUAL_OFFSETS
    );
    assert_eq!(
        follow_part_reading(
            texts,
            "widget.label.line_height",
            "widget.subtitle.line_height"
        ),
        UNEQUAL_OFFSETS
    );
    assert_eq!(
        follow_part_reading(texts, "widget.label.font_size", "widget.subtitle.font_size"),
        UNEQUAL_OFFSETS
    );
    assert_eq!(
        follow_part_reading(
            texts,
            "widget.label.line_height",
            "widget.label.line_height"
        ),
        NONE
    );
}

/// An offset reading an expression at a font size's place differs from the
/// same expression written at the offset (S2.4, S9.4, S12): its `1px`
/// literals are font sizes there, text-scaled; the place read on every
/// side, or the expression at a length's place, still reads equal.
#[test]
fn combination_follow_part_offsets_through_font_literals() {
    let texts = ["line_height = \"= add(1px, 1px)\"", ""];
    assert_eq!(
        follow_part_reading(texts, "widget.label.line_height", "add(1px, 1px)"),
        UNEQUAL_OFFSETS
    );
    assert_eq!(
        follow_part_reading(
            texts,
            "widget.label.line_height",
            "widget.label.line_height"
        ),
        NONE
    );
    assert_eq!(
        follow_part_reading(texts, "part.padding.bottom", "add(1px, 1px)"),
        NONE
    );
}

/// A `Concentric` corner whose two insets can differ, in either direction
/// (S2.4, S12, ADD-A122): the root's own margins here, its container lying
/// outside the widget.
#[test]
fn combination_concentric_insets() {
    let corner = |margins: &str| {
        format!(
            "[widgets.button.body]\ncorners.top_left = {{ concentric = {{ minimum_px = 2 }} }}\n\
             {margins}"
        )
    };
    assert_eq!(
        bare(&corner(
            "margin.top_px = 2\nmargin.left_px = 3\nmargin.right_px = 2"
        )),
        [
            "widgets.button.body.corners.top_left: `concentric` on `top_left`, whose insets can differ"
        ]
    );
    assert_eq!(
        bare(&corner(
            "margin.top_px = 2\nmargin.left_px = 2\nmargin.right_px = 2"
        )),
        NONE
    );
    // An absent margin is no inset.
    assert_eq!(
        bare(&corner(
            "margin.top_px = \"absent\"\nmargin.left_px = \"absent\"\nmargin.right_px = \"absent\""
        )),
        NONE
    );
}

/// `TextRoles` (S4.4, S12): two slots referencing one keyed style directly,
/// and a slot referencing a slot later in the owner order, are refused; a
/// slot sharing its owner's style references the owner.
#[test]
fn combination_text_roles() {
    let decl = declare("text_styles = [\"body\"]");
    let roles = |text: &str| format!("[tokens.text.roles]\n{text}");
    assert_eq!(
        alone(
            &roles("body = \"= token.text.keyed.body\"\ncaption = \"= token.text.keyed.body\""),
            &decl
        ),
        ["tokens.text.roles.caption: another slot references `text.keyed.body` directly"]
    );
    assert_eq!(
        alone(
            &roles("body = \"= token.text.keyed.body\"\ncaption = \"= token.text.roles.body\""),
            &decl
        ),
        NONE
    );
    assert_eq!(
        alone(
            &roles("caption = \"= token.text.keyed.body\"\nbody = \"= token.text.roles.caption\""),
            &decl
        ),
        ["tokens.text.roles.body: references `caption`, a slot later in the owner order"]
    );
    // Under one condition only.
    let rule = format!(
        "{}\n[[rules]]\nwhen = {{ conditions = [\"bold_text\"] }}\ntarget = \"tokens\"\n\
         set = {{ text.roles.caption = \"= token.text.keyed.body\" }}",
        roles("body = \"= token.text.keyed.body\"\ncaption = \"= token.text.roles.body\"")
    );
    assert_eq!(
        alone(&rule, &decl),
        ["rules[0].set.text.roles.caption: another slot references `text.keyed.body` directly"]
    );
}

/// A map entry's patch creating its entry with a field left unset that may
/// not be `Absent` (S6.1, S12, ADD-A122): a base's over no base stating the
/// key; a rule's where some combination before it leaves the key with no
/// entry.
#[test]
fn combination_entry_created_unset() {
    let decl = declare("text_styles = [\"caption\"]");
    let patch = "[tokens.text.keyed.caption]\nline_height = \"auto\"";
    let problems = alone(patch, &decl);
    assert!(
        problems.contains(
            &"tokens.text.keyed.caption: can create the entry with `font` unset".to_owned()
        ),
        "{problems:?}"
    );
    assert_eq!(over(patch, patch, &decl), NONE);
    // A rule creating the entry under a condition, a second patching it
    // under the same: the second meets the first's entry.
    let rule = |when: &str| {
        format!(
            "[[rules]]\nwhen = {when}\ntarget = \"tokens\"\n\
             set = {{ text.keyed.caption.line_height = \"auto\" }}"
        )
    };
    let two = format!(
        "{}\n{}",
        rule("{ conditions = [\"scheme:dark\"] }"),
        rule("{ conditions = [\"scheme:dark\"] }")
    );
    let problems = alone(&two, &decl);
    assert!(
        problems.iter().all(|p| p.starts_with("rules[0].set.")),
        "{problems:?}"
    );
    assert!(!problems.is_empty());
}

/// A layout field reading `element.width` (S8, S12), but over a fixed box;
/// a glyph drawn to its box, where the element's own part states its
/// minimum as a length.
#[test]
fn combination_element_size_in_layout() {
    let decl = declare(
        "[declare.widget.button]\ndata = { body = [\"width\"], menu_indicator = [\"width\"] }",
    );
    let padding = "[widgets.button.body]\npadding.left_px = \"= mul(element.width, 0.1)\"";
    assert_eq!(
        alone(
            &format!("{padding}\nmin_width_px = 40\nmax_width_px = \"auto\""),
            &decl
        ),
        ["widgets.button.body.padding.left_px: `element.width` in a field laying the element out"]
    );
    assert_eq!(
        alone(
            &format!("{padding}\nmin_width_px = 40\nmax_width_px = 40"),
            &decl
        ),
        NONE
    );
    // Through another field.
    let through = "[widgets.button.body]\nmin_width_px = 40\nmax_width_px = \"auto\"\n\
                   touch_target_px = \"= element.width\"\npadding.left_px = \"= part.touch_target\"";
    let problems = alone(through, &decl);
    assert!(
        problems.contains(
            &"widgets.button.body.padding.left_px: `element.width` in a field laying the element out"
                .to_owned()
        ),
        "{problems:?}"
    );
    // A glyph drawn to its box: the root's minimum stated as a length.
    let glyph = "[widgets.button.menu_indicator]\nglyph.size_px = \"= element.width\"";
    assert_eq!(
        alone(
            &format!("{glyph}\n[widgets.button.body]\nmin_width_px = 24"),
            &decl
        ),
        NONE
    );
    assert_eq!(
        alone(
            &format!("{glyph}\n[widgets.button.body]\nmin_width_px = \"auto\""),
            &decl
        ),
        [
            "widgets.button.menu_indicator.glyph.size_px: `element.width` in a field laying the element out"
        ]
    );
}

/// A size a rule selects is a later unit's where no default in force takes
/// it (S10's *Later names*, S12; ADD-A147): while the flavours unit is not
/// enabled, the default flavour's defaults alone are in force, so a size
/// the default only of a second flavour is refused; and a `sizes_by_variant`
/// entry of a `later` variant puts no default in force, as no element holds
/// that variant.
#[test]
fn combination_size_in_force_under_the_default_flavour() {
    let rule = |size: &str| {
        format!(
            "[[rules]]\nwhen = {{ sizes = [\"{size}\"] }}\ntarget = \"button.body\"\n\
             set = {{ opacity = 0.5 }}"
        )
    };
    let refused = "rules[0].when.sizes[0]: `small` is declared in `widget.button.sizes`, and a \
                   later unit states it";
    let flavoured = declare(
        "flavors = [\"base\", \"expressive\"]\n[declare.widget.button]\n\
         sizes = [\"small\", \"regular\"]\ndefault_size = \"regular\"\n\
         [declare.flavor.expressive.widget.button]\nsizes = [\"small\", \"regular\"]\n\
         default_size = \"small\"",
    );
    assert_eq!(alone(&rule("small"), &flavoured), [refused]);
    assert_eq!(alone(&rule("regular"), &flavoured), NONE);
    let by_variant = |later: &str| {
        declare(&format!(
            "[declare.widget.button]\nvariants = [\"@fab\"]\n{later}\
             sizes = [\"small\", \"regular\"]\ndefault_size = \"regular\"\n\
             sizes_by_variant = [{{ variant = \"@fab\", sizes = [\"small\", \"regular\"], \
             default_size = \"small\" }}]"
        ))
    };
    assert_eq!(
        alone(
            &rule("small"),
            &by_variant("later = { variants = [\"@fab\"] }\n")
        ),
        [refused]
    );
    assert_eq!(alone(&rule("small"), &by_variant("")), NONE);
}

/// Under each flavour the declarations in force are the widget's own with
/// that flavour's keys replacing them (S10): a rule is checked against the
/// declarations of each flavour it can apply under, a base against every
/// flavour's (S12).
#[test]
fn combination_per_flavour_declarations() {
    let decl = declare(
        "flavors = [\"widgets\", \"kirigami\"]\n[declare.flavor.kirigami.widget.button]\n\
         variants = [\"@pill\"]\ndata = { body = [\"progress\"] }",
    );
    let rule = |when: &str| {
        format!("[[rules]]\nwhen = {when}\ntarget = \"button.body\"\nset = {{ opacity = 0.5 }}")
    };
    assert_eq!(
        alone(&rule("{ variants = [\"@pill\"] }"), &decl),
        [
            "rules[0].when.variants[0]: `@pill` is not declared in `flavor.widgets.widget.button.variants`"
        ]
    );
    // A rule naming a flavour but the default names one a later unit
    // states while the flavours unit is not enabled (S12; ADD-A147).
    let later = "rules[0].when.conditions[0]: `kirigami` is declared in `flavors`, and a later \
                 unit states it";
    assert_eq!(
        alone(
            &rule("{ conditions = [\"flavor:kirigami\"], variants = [\"@pill\"] }"),
            &decl
        ),
        [later]
    );
    assert_eq!(
        alone(
            &rule("{ not_conditions = [\"flavor:widgets\"], variants = [\"@pill\"] }"),
            &decl
        ),
        NONE
    );
    // A base reads element data under every flavour.
    assert_eq!(
        alone(
            "[widgets.button.body]\nopacity = \"= element.progress\"",
            &decl
        ),
        [
            "widgets.button.body.opacity: `element.progress` is element data the part does not declare"
        ]
    );
    let flagged = "[[rules]]\nwhen = { conditions = [\"flavor:kirigami\"] }\ntarget = \
                   \"button.body\"\nset = { opacity = \"= element.progress\" }";
    assert_eq!(alone(flagged, &decl), [later]);
    // Availability under a flavour.
    let unavailable = declare(
        "flavors = [\"widgets\", \"kirigami\"]\n[declare.flavor.kirigami.widget.button]\n\
         available = false",
    );
    let drawn = |when: &str| {
        format!(
            "[[rules]]\nwhen = {when}\ntarget = \"button.icon\"\n\
             set = {{ drawn_as = {{ widget = \"button\", part = \"body\", variants = [] }} }}"
        )
    };
    assert_eq!(
        alone(&drawn("{ states = [\"hover\"] }"), &unavailable),
        ["rules[0].set.drawn_as.widget: `button` is not available on the preset"]
    );
    assert_eq!(
        alone(
            &drawn("{ conditions = [\"flavor:widgets\"] }"),
            &unavailable
        ),
        NONE
    );
}

/// On `kde-breeze`, a mapping palette leaving an accent-dependent role or
/// an inactive input role to the preset (S9.1, S9.2, S14, S12). The check
/// runs here directly over a stack whose small preset states the roles,
/// until M1b's palettes exercise it over the bundled `kde-breeze` through
/// `Theme::apply_palette`; the scheme's tint takes every key in.
#[test]
fn combination_mapping_palette() {
    use super::combos::mapping_palette;
    use super::stack::Stack;
    let decl = declare(
        "surface_sets = [\"window\"]\ninputs = [\"accent\", \"inactive.window.foreground_active\"]",
    );
    let roles = "[colors.light.surfaces.window]\nforeground_active = \"#000001\"\n\
                 link = \"#000002\"\ndecoration_focus = \"#000003\"\ndecoration_hover = \"#000004\"\n\
                 background = \"#000005\"";
    let preset = toml(roles);
    let problems = |palette: &str, preset: &Layer| {
        let palette = toml(palette);
        let layers = [
            Stacked {
                layer: preset,
                kind: LayerKind::Preset,
                left_out: None,
            },
            Stacked {
                layer: &palette,
                kind: LayerKind::Palette,
                left_out: None,
            },
        ];
        let cx = Context {
            declarations: &decl,
            preset: None,
            kind: LayerKind::Palette,
            reader_accent_left_out: false,
            below: &[],
        };
        let stack = Stack::new(&layers, cx);
        mapping_palette(&stack, 1)
            .iter()
            .map(Problem::to_string)
            .collect::<Vec<_>>()
    };
    let input = "[inputs.\"inactive.window.foreground_active\"]\nlight = \"#000006\"";
    let partial = "[colors.light.surfaces.window]\nforeground_active = \"#100001\"\n\
                   link = \"#100002\"\ndecoration_focus = \"#100003\"";
    assert_eq!(
        problems(&format!("{partial}\n{input}"), &preset),
        ["colors.light.surfaces.window.decoration_hover: \
             `colors.light.surfaces.window.decoration_hover` left to the preset's value under \
             the palette"]
    );
    let whole = format!("{partial}\ndecoration_hover = \"#100004\"\n{input}");
    assert_eq!(problems(&whole, &preset), NONE);
    // The inactive input role.
    assert_eq!(
        problems(
            &format!("{partial}\ndecoration_hover = \"#100004\""),
            &preset
        ),
        [
            "inputs.inactive.window.foreground_active: `inputs.inactive.window.foreground_active` \
             left to the preset's value under the palette"
        ]
    );
    // A port states the scheme inputs: no mapping, not checked here.
    let port =
        format!("{partial}\n[inputs.\"kde_scheme.window.background_normal\"]\nlight = \"#000000\"");
    assert_eq!(problems(&port, &preset), NONE);
    // Wherever the preset's scheme tints, every key of the set depends on
    // the accent: the background too (S9.1); not where it is `none`.
    let tinting = toml(&format!("{roles}\n[tokens.kde]\ntint_factor = 0.5"));
    assert_eq!(
        problems(&whole, &tinting),
        ["colors.light.surfaces.window.background: \
             `colors.light.surfaces.window.background` left to the preset's value under \
             the palette"]
    );
    let untinted = toml(&format!("{roles}\n[tokens.kde]\ntint_factor = \"none\""));
    assert_eq!(problems(&whole, &untinted), NONE);
    let stated =
        format!("{partial}\ndecoration_hover = \"#100004\"\nbackground = \"#100005\"\n{input}");
    assert_eq!(problems(&stated, &tinting), NONE);
}

/// A mapping palette's tinting keys follow the preset's
/// `token.kde.tint_factor` alone (S8, S12; ADD-A145): a reader whose scheme
/// tints beneath it demands no key a non-tinting preset does not, and a
/// reader that does not tint leaves a tinting preset's demand as it is.
#[test]
fn combination_mapping_palette_reads_the_presets_tint() -> Result<(), Box<dyn std::error::Error>> {
    use super::combos::mapping_palette;
    use super::stack::Stack;
    let decl = declare(
        "surface_sets = [\"window\"]\ninputs = [\"accent\", \"inactive.window.foreground_active\"]",
    );
    let roles = "[colors.light.surfaces.window]\nforeground_active = \"#000001\"\n\
                 link = \"#000002\"\ndecoration_focus = \"#000003\"\ndecoration_hover = \"#000004\"\n\
                 background = \"#000005\"";
    let problems = |preset_tint: &str, reader_tint: &str| -> Result<Vec<String>, LayerError> {
        let preset = Layer::from_toml(&format!(
            "{roles}\n[tokens.kde]\ntint_factor = {preset_tint}"
        ))?;
        let reader = Layer::from_toml(&format!("[tokens.kde]\ntint_factor = {reader_tint}"))?;
        let palette = Layer::from_toml(
            "[colors.light.surfaces.window]\nforeground_active = \"#100001\"\n\
             link = \"#100002\"\ndecoration_focus = \"#100003\"\ndecoration_hover = \"#100004\"\n\
             [inputs.\"inactive.window.foreground_active\"]\nlight = \"#000006\"",
        )?;
        let layers = [
            Stacked {
                layer: &preset,
                kind: LayerKind::Preset,
                left_out: None,
            },
            Stacked {
                layer: &reader,
                kind: LayerKind::Reader,
                left_out: None,
            },
            Stacked {
                layer: &palette,
                kind: LayerKind::Palette,
                left_out: None,
            },
        ];
        let cx = Context {
            declarations: &decl,
            preset: None,
            kind: LayerKind::Palette,
            reader_accent_left_out: false,
            below: &[],
        };
        Ok(mapping_palette(&Stack::new(&layers, cx), 2)
            .iter()
            .map(Problem::to_string)
            .collect())
    };
    assert_eq!(problems("\"none\"", "0.5")?, NONE);
    assert_eq!(
        problems("0.5", "\"none\"")?,
        ["colors.light.surfaces.window.background: \
             `colors.light.surfaces.window.background` left to the preset's value under \
             the palette"]
    );
    Ok(())
}

/// S9.1's accent-dependent roles (S9.2, S14): the one list gives, item for
/// item, the role paths `mapping_palette` built in place before it was
/// shared, over every bundled preset's declarations and over every surface
/// set with every `[WM]` keyed role declared, the scheme tinting or not.
#[test]
fn accent_dependent_roles_as_mapping_palette_listed_them() -> Result<(), Box<dyn std::error::Error>>
{
    use super::combos::{AccentRole, accent_dependent_roles};
    use crate::inputs::{KdeSchemeKey as K, KdeWmKey};
    use crate::roles::SurfaceSet as S;
    // The list as `mapping_palette` built it in place.
    let listed_before = |decl: &Declarations, tints: bool| {
        let mut roles: Vec<String> = Vec::new();
        for set in &decl.surface_sets {
            let mut keys: Vec<K> = vec![
                K::ForegroundActive,
                K::ForegroundLink,
                K::DecorationFocus,
                K::DecorationHover,
            ];
            match set {
                _ if tints => keys = K::ALL.to_vec(),
                S::Selection => keys.extend([
                    K::BackgroundNormal,
                    K::BackgroundAlternate,
                    K::ForegroundNormal,
                    K::ForegroundInactive,
                ]),
                S::Button => keys.push(K::BackgroundAlternate),
                S::Header => keys.extend([K::BackgroundNormal, K::ForegroundNormal]),
                _ => {}
            }
            for key in keys {
                let role = format!("surfaces.{}.{}", set.name(), key.role().path());
                if !roles.contains(&role) {
                    roles.push(role);
                }
            }
        }
        for key in KdeWmKey::ALL {
            let name = format!("wm_{}", key.name());
            if decl.color_roles.iter().any(|held| held.as_str() == name) {
                roles.push(format!("keyed.{name}"));
            }
        }
        roles
    };
    let every = Declarations {
        surface_sets: S::ALL.to_vec(),
        color_roles: KdeWmKey::ALL
            .iter()
            .map(|key| crate::key::Key::new(&format!("wm_{}", key.name())))
            .collect::<Result<Vec<_>, _>>()?,
        ..Declarations::default()
    };
    assert_eq!(every.color_roles.len(), KdeWmKey::ALL.len());
    let mut decls: Vec<Declarations> = crate::presets::PresetId::all()
        .iter()
        .map(|id| read_preset(id.source()).map(|file| file.declarations))
        .collect::<Result<Vec<_>, _>>()?;
    decls.push(every);
    for decl in &decls {
        for tints in [false, true] {
            let shared: Vec<String> = accent_dependent_roles(decl, tints)
                .into_iter()
                .map(AccentRole::path)
                .collect();
            assert_eq!(shared, listed_before(decl, tints));
        }
    }
    let breeze = read_preset(crate::presets::PresetId::KdeBreeze.source())?.declarations;
    let roles = accent_dependent_roles(&breeze, false);
    assert!(roles.contains(&AccentRole::Surface(S::Header, K::BackgroundNormal)));
    assert!(
        KdeWmKey::ALL
            .iter()
            .all(|key| roles.contains(&AccentRole::Wm(*key)))
    );
    Ok(())
}

/// `kde_effects` and `kde_selection_inactive` read `token.kde` themselves,
/// and the checks treat those tokens as their operands (S2.2, S12): under a
/// preset whose `token.kde` is absent each call is refused for each token
/// it reads, under one stating them it is accepted.
#[test]
fn combination_kde_functions_read_token_kde_as_operands() {
    let accent = |expr: &str| format!("[colors.light.global]\naccent = \"= {expr}\"");
    let effects =
        "kde_effects(role.window.foreground, role.window.background, inactive, foreground)";
    let selection = "kde_selection_inactive(role.window.foreground, role.window.background, \
                     role.window.background, background)";
    let decl = declare("surface_sets = [\"window\"]");
    let absent = "[tokens]\nkde = \"absent\"";
    let refused = |token: &str| {
        format!("colors.light.global.accent: `token.kde.{token}` can be `absent` here")
    };
    let fields = [
        "intensity",
        "intensity_amount",
        "color",
        "color_amount",
        "color_color",
        "contrast",
        "contrast_amount",
    ];
    let mut expected: Vec<String> = fields
        .iter()
        .map(|field| refused(&format!("inactive.{field}")))
        .collect();
    expected.push(refused("inactive_enabled"));
    let mut found = over(absent, &accent(effects), &decl);
    found.sort();
    expected.sort();
    assert_eq!(found, expected);
    assert_eq!(
        over(absent, &accent(selection), &decl),
        [refused("inactive_change_selection_color")]
    );
    let group = |name: &str| {
        format!(
            "{name} = {{ intensity = \"none\", intensity_amount = 0, color = \"none\", \
             color_amount = 0, color_color = \"#000000\", contrast = \"none\", contrast_amount = 0 }}"
        )
    };
    let stated = format!(
        "[tokens.kde]\n{}\n{}\ninactive_enabled = true\ninactive_change_selection_color = true\n\
         tint_factor = \"none\"\ntitlebar_is_accent_colored = false",
        group("disabled"),
        group("inactive")
    );
    for expr in [effects, selection] {
        assert_eq!(over(&stated, &accent(expr), &decl), NONE, "{expr}");
    }
}

/// `part.corners.<c>` names a field the part lists (S12): a part with no
/// corners has none.
#[test]
fn combination_part_corner_on_a_part_without_corners() {
    assert_eq!(
        bare("[widgets.button.label]\npadding.top_px = \"= part.corners.top_left\""),
        [
            "widgets.button.label.padding.top_px: the expression does not type: `part.corners.top_left` names nothing"
        ]
    );
    assert_eq!(
        bare("[widgets.button.label]\npadding.top_px = \"= part.focus.corners.top_left\""),
        [
            "widgets.button.label.padding.top_px: the expression does not type: `part.focus.corners.top_left` names nothing"
        ]
    );
}

/// `active(…)` reads the role's bases, before any rule (S2.2): a `Colors`
/// rule restating a role from its own active value reads no rule, so it is
/// no cycle (R5.1's inactive-window colours are written so).
#[test]
fn combination_active_reads_the_bases_not_itself() {
    let below = "[colors.light.global]\naccent = \"#3366ff\"\n\
                 [colors.dark.global]\naccent = \"#3366ff\"";
    let rule = |expr: &str| {
        format!(
            "[[rules]]\nwhen = {{ conditions = [\"reduced_motion\"] }}\ntarget = \"colors\"\n\
             set = {{ global.accent = \"{expr}\" }}"
        )
    };
    assert_eq!(
        over(
            below,
            &rule("= mix(active(role.global.accent), #000000, 0.2)"),
            &Declarations::default()
        ),
        NONE
    );
    assert_eq!(
        over(
            below,
            &rule("= mix(role.global.accent, #000000, 0.2)"),
            &Declarations::default()
        ),
        ["rules[0].set.global.accent: the expression reads itself"]
    );
}

/// A `Fields` patch kept on a reference composes lazily (S2.2): the
/// referenced value's bases, the patches stated in bases, then every rule in
/// the stack's order, the referenced value's own among them; a rule of the
/// referent after the patch decides.
#[test]
fn combination_patch_over_a_reference_meets_the_referents_rules() {
    let decl = declare("text_styles = [\"body\"]");
    let below = "[tokens.text.keyed.body]\nline_height_px = 20\n\
                 [widgets.button.label]\ntext = \"= token.text.keyed.body\"\n\
                 [[rules]]\nwhen = { conditions = [\"reduced_motion\"] }\ntarget = \"tokens\"\n\
                 set = { text.keyed.body.line_height = \"auto\" }";
    let top = "[widgets.button.label]\ntext.line_height_px = 18\n\
               text.font.size = \"= add(part.text.line_height, 2px)\"";
    assert_eq!(
        over(below, top, &decl),
        ["widgets.button.label.text.font.size: `part.text.line_height` can be `auto` here"]
    );
    // A rule patching the reference after the referent's rule decides.
    let later = format!(
        "{top}\n[[rules]]\nwhen = {{ conditions = [\"reduced_motion\"] }}\ntarget = \
         \"button.label\"\nset = {{ text.line_height_px = 18 }}"
    );
    assert_eq!(over(below, &later, &decl), NONE);
}

/// A rule's binding applies under some combinations only: under the others
/// the colour roles' rules decide the role (S3.3, S12).
#[test]
fn combination_a_rules_binding_leaves_the_roles_rules() {
    let decl = declare("surface_sets = [\"window\"]");
    let below = "[colors.light.surfaces.window]\nneutral = \"#00ff00\"\n\
                 [colors.dark.surfaces.window]\nneutral = \"#00ff00\"";
    let top = "[widgets.button.body]\nfill = \"= mix(surface.neutral, #000000, 0.5)\"\n\
               [[rules]]\nwhen = { conditions = [\"reduced_motion\"] }\ntarget = \"colors\"\n\
               set = { surfaces.window.neutral = \"absent\" }\n\
               [[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.body\"\n\
               set = { bindings = [{ role = { role = { surface = \"neutral\" }, value = \
               \"#ff0000\" } }] }";
    assert_eq!(
        over(below, top, &decl),
        ["widgets.button.body.fill: `surface.neutral` can be `absent` here"]
    );
}

/// S3.3's two steps: the root part's bindings apply to every part of the
/// widget; a `Role { Surface(x) }` binding never answers `role.S.x`; a
/// `Set(S, x)` binding answers `surface.x` only under a `Surface(S)`
/// binding.
#[test]
fn combination_bindings_follow_s3_3() {
    let decl = declare("surface_sets = [\"window\", \"header\"]");
    let below = "[colors.light.surfaces.window]\nneutral = \"absent\"\n\
                 [colors.dark.surfaces.window]\nneutral = \"absent\"\n\
                 [colors.light.surfaces.header]\nneutral = \"#000000\"\n\
                 [colors.dark.surfaces.header]\nneutral = \"#000000\"";
    let bind = |role: &str| {
        format!(
            "[widgets.button.body]\nbindings = [{{ role = {{ role = {role}, value = \
             \"#ff0000\" }} }}]"
        )
    };
    // (a) The root's binding answers the label's `surface.neutral`.
    let label = format!(
        "{}\n[widgets.button.label]\ntext.fill = \"= mix(surface.neutral, #000000, 0.5)\"",
        bind("{ surface = \"neutral\" }")
    );
    assert_eq!(over(below, &label, &decl), NONE);
    // (b) Step 2 ignores a `Role { Surface(x) }` binding.
    let role = format!(
        "{}\nfill = \"= mix(role.window.neutral, #000000, 0.5)\"",
        bind("{ surface = \"neutral\" }")
    );
    assert_eq!(
        over(below, &role, &decl),
        ["widgets.button.body.fill: `role.window.neutral` can be `absent` here"]
    );
    // (c) A `Set(header, neutral)` binding with no `Surface(header)` one.
    let set = format!(
        "{}\nfill = \"= mix(surface.neutral, #000000, 0.5)\"",
        bind("{ set = [\"header\", \"neutral\"] }")
    );
    assert_eq!(
        over(below, &set, &decl),
        ["widgets.button.body.fill: `surface.neutral` can be `absent` here"]
    );
    // With the `Surface(header)` binding, `surface.neutral` is
    // `role.header.neutral`, which the `Set` binding answers.
    let surfaced = "[widgets.button.body]\nbindings = [{ surface = \"header\" }, { role = { role = \
                    { set = [\"header\", \"neutral\"] }, value = \"#ff0000\" } }]\n\
                    fill = \"= mix(surface.neutral, #000000, 0.5)\"";
    assert_eq!(over(below, surfaced, &decl), NONE);
    // (d) A binding's value is evaluated in the scope outside it.
    let outside = bind("{ surface = \"neutral\" }").replace(
        "value = \"#ff0000\"",
        "value = \"= mix(surface.neutral, #000000, 0.5)\"",
    );
    assert_eq!(
        over(below, &outside, &decl),
        ["widgets.button.body.bindings[0].role.value: `surface.neutral` can be `absent` here"]
    );
}

/// `widget.<part>` read from the widget's values (S12): the part not drawn,
/// a corner computed at layout.
#[test]
fn combination_widget_part_from_the_values() {
    assert_eq!(
        bare(
            "[widgets.button.menu_indicator]\nabsent = true\n[widgets.button.values]\n\
             subtitle_gap_px = \"= widget.menu_indicator.min_width\""
        ),
        [
            "widgets.button.values.subtitle_gap_px: `widget.menu_indicator` can be absent or drawn as another widget"
        ]
    );
    let problems = bare(
        "[widgets.button.body]\ncorners.top_left = { concentric = { minimum_px = 2 } }\n\
         [widgets.button.values]\nicon_text_gap_px = \"= widget.body.corners.top_left\"",
    );
    assert!(
        problems.contains(
            &"widgets.button.values.icon_text_gap_px: `widget.body.corners.top_left` is a corner computed at layout"
                .to_owned()
        ),
        "{problems:?}"
    );
}

/// A corner operand that can resolve `Full` reads the element's width and
/// height, which its part declares (S8, S12).
#[test]
fn combination_full_corner_needs_the_size_data() {
    let full = "[widgets.button.body]\ncorners.top_left = { full = { maximum_px = \"absent\" } }\n\
                focus.width_px = \"= part.corners.top_left\"";
    assert_eq!(
        bare(full),
        [
            "widgets.button.body.focus.width_px: `element.width` is element data the part does not declare",
            "widgets.button.body.focus.width_px: `element.height` is element data the part does not declare",
        ]
    );
    let decl = declare("[declare.widget.button]\ndata = { body = [\"width\", \"height\"] }");
    assert_eq!(alone(full, &decl), NONE);
}

/// `widget.<part>.font_size` and `.line_height` are that part's text
/// metrics (S2.2): followed, so two parts reading each other's line height
/// form a cycle.
#[test]
fn combination_widget_part_text_metrics_followed() {
    let decl = declare(
        "[declare.widget.button]\ndata = { label = [\"widget.subtitle.line_height\"], \
         subtitle = [\"widget.label.line_height\"] }",
    );
    assert_eq!(
        alone(
            "[widgets.button.label]\ntext.line_height = \"= widget.subtitle.line_height\"\n\
             [widgets.button.subtitle]\ntext.line_height = \"= widget.label.line_height\"",
            &decl
        ),
        [
            "widgets.button.label.text.line_height: the expression reads itself",
            "widgets.button.subtitle.text.line_height: the expression reads itself",
        ]
    );
}

/// A keyed list's later item replaces the earlier whole (S6.1): the
/// replaced transition's `delay` is never evaluated.
#[test]
#[ignore = "motion unit: no part lists `transitions` until it is enabled (D6 outcome 3)"]
fn combination_keyed_item_replaced() {
    let transition = |delay: &str| {
        format!(
            "[widgets.button.body]\ntransitions = [{{ properties = [\"fill\"], kind = \
             \"interpolate\", duration_ms = 100, delay_ms = {delay}, timing = \"linear\", \
             engine = \"gtk_css\" }}]"
        )
    };
    let below = format!(
        "{}\n[widgets.button.values]\nrepeat_delay_ms = 400",
        transition("\"= add(values.repeat_delay, 10ms)\"")
    );
    let top = format!(
        "{}\n[widgets.button.values]\nrepeat_delay_ms = \"absent\"",
        transition("0")
    );
    assert_eq!(over(&below, &top, &Declarations::default()), NONE);
    // Left in place, the preset's transition meets the absent value.
    let kept = "[widgets.button.values]\nrepeat_delay_ms = \"absent\"";
    assert_eq!(
        over(&below, kept, &Declarations::default()),
        ["widgets.button.body.transitions[0].delay_ms: `values.repeat_delay` can be `absent` here"]
    );
}

/// A keyed list's transitions are keyed by their Breeze test too (S6.1,
/// ADD-A126): a later transition of the same test replaces the earlier, so
/// its `duration` is never evaluated; one of another test leaves it.
#[test]
#[ignore = "motion unit: no part lists `transitions` until it is enabled (D6 outcome 3)"]
fn combination_keyed_item_replaced_by_breeze_test() {
    let transition = |duration: &str, any: &str| {
        format!(
            "[widgets.button.body]\ntransitions = [{{ properties = [\"fill\"], kind = \
             \"interpolate\", duration_ms = {duration}, delay_ms = 0, timing = \"linear\", \
             engine = {{ breeze = {{ any = [{any}], none = [], conditions = [] }} }} }}]"
        )
    };
    let below = format!(
        "{}\n[widgets.button.values]\nrepeat_delay_ms = 400",
        transition(
            "\"= add(values.repeat_delay, 10ms)\"",
            "\"hover\", \"pressed\""
        )
    );
    let top = |any: &str| {
        format!(
            "{}\n[widgets.button.values]\nrepeat_delay_ms = \"absent\"",
            transition("100", any)
        )
    };
    let decl = Declarations::default();
    assert_eq!(over(&below, &top("\"pressed\", \"hover\""), &decl), NONE);
    assert_eq!(
        over(&below, &top("\"hover\""), &decl),
        [
            "widgets.button.body.transitions[0].duration_ms: `values.repeat_delay` can be `absent` here"
        ]
    );
}

/// An ancestor's or sibling's states are reduced by its own widget's
/// exclusive groups (S8, S10): a match needing two members of one never
/// holds, so its rule never applies (S12's exclusions).
#[test]
fn combination_context_states_reduced_by_their_groups() {
    let decl = declare("[declare.widget.button]\nexclusive = [[\"hover\", \"pressed\"]]");
    let rules = |states: &str| {
        format!(
            "[[rules]]\nwhen = {{ context = [{{ widget = \"button\", states = {states} }}] }}\n\
             target = \"button.body\"\nset = {{ touch_target_px = \"absent\" }}\n\
             [[rules]]\nwhen = {{ states = [\"hover\"] }}\ntarget = \"button.body\"\n\
             set = {{ min_width_px = \"= add(part.touch_target, 4px)\" }}"
        )
    };
    assert_eq!(alone(&rules("[\"hover\", \"pressed\"]"), &decl), NONE);
    assert_eq!(
        alone(&rules("[\"hover\"]"), &decl),
        ["rules[1].set.min_width_px: `part.touch_target` can be `absent` here"]
    );
    let siblings = "[[rules]]\nwhen = { position = { among = \"any\", next = { widget = \"button\", \
                    states = [\"hover\", \"pressed\"] } } }\ntarget = \"button.body\"\n\
                    set = { touch_target_px = \"absent\" }\n\
                    [[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.body\"\n\
                    set = { min_width_px = \"= add(part.touch_target, 4px)\" }";
    assert_eq!(alone(siblings, &decl), NONE);
}

/// A problem the combinations find at a place a layer below states is
/// kept beside the added layer's structural problem at the same path:
/// one names a place of the preset, the other of the added layer (S12's
/// "listing every problem").
#[test]
fn combination_problem_below_kept_beside_the_added_layers() {
    let below = "[widgets.button.icon.icon]\nsize_px = 16\nfill = \"#000000\"\n\
                 rendering_mode = \"original\"\n\
                 [[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.icon\"\n\
                 set = { icon.fill = \"absent\" }";
    let top = "[[rules]]\nwhen = { states = [\"pressed\"] }\ntarget = \"button.icon\"\n\
               set = { icon = \"= token.focus\" }\n\
               [[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.icon\"\n\
               set = { icon.rendering_mode = \"template\" }";
    assert_eq!(
        over(below, top, &Declarations::default()),
        [
            "rules[0].set.icon: the expression does not type: a FocusIndicator where an IconStyle is expected",
            "rules[0].set.icon.fill: may not be absent",
        ]
    );
}

/// The offset and inset checks read a side alike (S2.4, S2.10): a side no
/// layer states is missing, the preset's check, so no difference; an
/// absent margin is 0.
#[test]
fn combination_sides_read_alike() {
    let focus = |offsets: &str| {
        format!("[widgets.button.body.focus]\ncorners.top_left = \"follow_part\"\n{offsets}")
    };
    let corner = |margins: &str| {
        format!(
            "[widgets.button.body]\ncorners.top_left = {{ concentric = {{ minimum_px = 2 }} }}\n\
             {margins}"
        )
    };
    assert_eq!(
        bare(&corner("margin.top_px = 2\nmargin.right_px = 2")),
        NONE
    );
    assert_eq!(bare(&focus("offset.top_px = 2\noffset.right_px = 2")), NONE);
    assert_eq!(
        bare(&corner(
            "margin.top_px = \"absent\"\nmargin.left_px = 0\nmargin.right_px = 0"
        )),
        NONE
    );
}

/// `Ellipse` corners (S2.4, S12, ADD-A124): on all four corners with a
/// `Circular` curve, under every combination; no shape where a corner can be
/// one, and no corner operand on one, whose two radii are no one length.
#[test]
fn combination_ellipse_corners() {
    let all = "[widgets.button.body]\ncorners.top_left = \"ellipse\"\ncorners.top_right = \
               \"ellipse\"\ncorners.bottom_right = \"ellipse\"\ncorners.bottom_left = \"ellipse\"\n\
               corners.curve = \"circular\"";
    assert_eq!(bare(all), NONE);
    assert_eq!(
        bare(&all.replace(
            "corners.top_right = \"ellipse\"",
            "corners.top_right_px = 4"
        )),
        [
            "widgets.button.body.corners.top_left: `ellipse` on a corner, not on all four corners with a `circular` curve",
            "widgets.button.body.corners.bottom_right: `ellipse` on a corner, not on all four corners with a `circular` curve",
            "widgets.button.body.corners.bottom_left: `ellipse` on a corner, not on all four corners with a `circular` curve",
        ]
    );
    assert_eq!(
        bare(&all.replace("\"circular\"", "\"continuous\"")).len(),
        4
    );
    // A rule restating one corner under a state.
    let rule = format!(
        "{all}\n[[rules]]\nwhen = {{ states = [\"hover\"] }}\ntarget = \"button.body\"\n\
         set = {{ corners.top_left_px = 4 }}"
    );
    assert_eq!(bare(&rule).len(), 3);
    // Through a reference: the body's corners the token's focus corners,
    // all four ellipses, one patched to a length over the reference.
    let referenced = "[tokens.focus.corners]\ntop_left = \"ellipse\"\ntop_right = \"ellipse\"\n\
                      bottom_right = \"ellipse\"\nbottom_left = \"ellipse\"\ncurve = \"circular\"\n\
                      [widgets.button.body]\ncorners = \"= token.focus.corners\"";
    let patched = "[widgets.button.body]\ncorners.top_left_px = 4";
    assert_eq!(over(referenced, "", &Declarations::default()), NONE);
    assert!(
        !over(referenced, patched, &Declarations::default()).is_empty(),
        "an ellipse reached through a reference is checked"
    );
    // A corner operand on an ellipse.
    assert_eq!(
        bare(&format!(
            "{all}\nfocus.width_px = \"= part.corners.top_left\""
        )),
        [
            "widgets.button.body.focus.width_px: `part.corners.top_left` is a corner computed at layout"
        ]
    );
}

/// The latest flag of a recency set (S7.2, S12's exclusions): at most one
/// per set, a flag held, so two rules on two latest flags of one set never
/// apply together, and a rule on a latest flag never applies without it.
#[test]
fn combination_latest_of_a_recency_set() {
    let decl =
        declare("[declare.widget.button]\nrecency = { glow = [\"hover\", \"focus_visible\"] }");
    let rules = |second: &str| {
        format!(
            "[[rules]]\nwhen = {{ latest = {{ glow = \"hover\" }} }}\ntarget = \"button.body\"\n\
             set = {{ touch_target_px = \"absent\" }}\n\
             [[rules]]\nwhen = {second}\ntarget = \"button.body\"\n\
             set = {{ min_width_px = \"= add(part.touch_target, 4px)\" }}"
        )
    };
    assert_eq!(
        alone(&rules("{ latest = { glow = \"focus_visible\" } }"), &decl),
        NONE
    );
    assert_eq!(alone(&rules("{ not_states = [\"hover\"] }"), &decl), NONE);
    assert_eq!(
        alone(&rules("{ states = [\"focus_visible\"] }"), &decl),
        ["rules[1].set.min_width_px: `part.touch_target` can be `absent` here"]
    );
}

/// Two positions among the same widgets, listed in another order or with
/// one twice, count among one set of siblings: one parity at a time (S12),
/// so an `Absent` under odd and an expression reading it under even never
/// meet; an ancestor scope alike. Under one parity, or the same scope both
/// times, they meet and are refused, so each pass is the set comparison's.
#[test]
fn combination_sibling_lists_compared_as_sets() {
    const REFUSED: &str = "rules[1].set.min_width_px: `part.touch_target` can be `absent` here";
    let decl = declare("");
    let pair = |first: &str, second: &str| {
        format!(
            "[[rules]]\nwhen = {{ {first} }}\ntarget = \"button.body\"\n\
             set = {{ touch_target_px = \"absent\" }}\n\
             [[rules]]\nwhen = {{ {second} }}\ntarget = \"button.body\"\n\
             set = {{ min_width_px = \"= add(part.touch_target, 4px)\" }}"
        )
    };
    let odd = "position = { among = [\"button\", \"button\"], parity = \"odd\" }";
    let even = "position = { among = [\"button\"], parity = \"even\" }";
    let also_odd = "position = { among = [\"button\"], parity = \"odd\" }";
    assert_eq!(alone(&pair(odd, even), &decl), NONE);
    assert_eq!(alone(&pair(odd, also_odd), &decl), [REFUSED]);
    let within =
        "context = [{ scope = { nearest = [\"button\", \"button\"] }, widget = \"button\" }]";
    let outside = "not_context = [{ scope = { nearest = [\"button\"] }, widget = \"button\" }]";
    let also_within = "context = [{ scope = { nearest = [\"button\"] }, widget = \"button\" }]";
    assert_eq!(alone(&pair(within, outside), &decl), NONE);
    assert_eq!(alone(&pair(within, also_within), &decl), [REFUSED]);
}

/// S12's exclusions, one by one: an `Absent` under one `when` and an
/// expression reading it under another meet only where the two can hold
/// together.
/// Each pair the spec excludes passes; the same pair made independent is
/// refused, so the pass is the exclusion's.
#[test]
fn combination_exclusions_are_s12s() {
    const REFUSED: &str = "rules[1].set.min_width_px: `part.touch_target` can be `absent` here";
    let pair = |first: &str, second: &str, decl: &str| {
        let rules = format!(
            "[[rules]]\nwhen = {{ {first} }}\ntarget = \"button.body\"\nset = {{ touch_target_px = \"absent\" }}\n\
             [[rules]]\nwhen = {{ {second} }}\ntarget = \"button.body\"\n\
             set = {{ min_width_px = \"= add(part.touch_target, 4px)\" }}"
        );
        // `decl` above the widget's declarations, or after them where it
        // starts with a newline.
        let widget = "[declare.widget.button]\nsizes = [\"small\", \"regular\"]\n\
                      densities = [\"default\", \"compact\"]";
        let decl = match decl.strip_prefix('\n') {
            Some(own) => format!("{widget}\n{own}"),
            None => format!("{decl}\n{widget}"),
        };
        alone(&rules, &declare(&decl))
    };
    let excluded: &[(&str, &str, &str, &str)] = &[
        (
            "scheme",
            "conditions = [\"scheme:light\"]",
            "conditions = [\"scheme:dark\"]",
            "",
        ),
        (
            "input",
            "conditions = [\"input:mouse\"]",
            "conditions = [\"input:touch\"]",
            "",
        ),
        (
            "idiom",
            "conditions = [\"idiom:phone\"]",
            "conditions = [\"idiom:pad\"]",
            "",
        ),
        (
            "device model",
            "conditions = [\"device_model:a\"]",
            "conditions = [\"device_model:b\"]",
            "device_models = [\"a\", \"b\"]",
        ),
        (
            "posture",
            "conditions = [\"posture:flat\"]",
            "conditions = [\"posture:book\"]",
            "",
        ),
        (
            "container size",
            "conditions = [\"container_size:button:small\"]",
            "conditions = [\"container_size:button:large\"]",
            "container_sizes = { button = [\"small\", \"large\"] }",
        ),
        (
            "contrast",
            "conditions = [\"contrast_increased\"]",
            "conditions = [\"contrast_high\"]",
            "",
        ),
        (
            "exclusive flags",
            "states = [\"hover\"]",
            "states = [\"pressed\"], not_states = [\"hover\"]",
            "\nexclusive = [[\"hover\", \"pressed\"]]",
        ),
        (
            "exclusive variants",
            "variants = [\"primary\"]",
            "variants = [\"tool\"]",
            "\nvariants = [\"primary\", \"tool\"]\n\
             exclusive_variants = [[\"primary\", \"tool\"]]",
        ),
        (
            "direction",
            "direction = \"ltr\"",
            "direction = \"rtl\"",
            "",
        ),
        (
            "size class",
            "size_class = { horizontal = [\"compact\"] }",
            "size_class = { horizontal = [\"expanded\"] }",
            "",
        ),
        (
            "parity",
            "position = { parity = \"odd\" }",
            "position = { parity = \"even\" }",
            "",
        ),
        (
            "only is first and last",
            "position = { only = true }",
            "position = { first = false }",
            "",
        ),
        (
            "first is odd",
            "position = { first = true }",
            "position = { parity = \"even\" }",
            "",
        ),
        (
            "tiled within",
            "conditions = [\"tiled:top,left\"]",
            "not_conditions = [\"tiled:top\"]",
            "",
        ),
        (
            "maximized is tiled",
            "conditions = [\"maximized\"]",
            "not_conditions = [\"tiled:bottom\"]",
            "",
        ),
    ];
    for (name, first, second, decl) in excluded {
        assert_eq!(pair(first, second, decl), NONE, "{name}");
    }
    // Two flavours: a rule naming a flavour but the default names one a
    // later unit states while the flavours unit is not enabled (S12;
    // ADD-A147), so the pair is refused for that alone.
    assert_eq!(
        pair(
            "conditions = [\"flavor:one\"]",
            "conditions = [\"flavor:two\"]",
            "flavors = [\"one\", \"two\"]"
        ),
        [
            "rules[1].when.conditions[0]: `two` is declared in `flavors`, and a later unit states \
             it"
        ]
    );
    // The same kinds of atom, independent: refused.
    let independent: &[(&str, &str, &str)] = &[
        (
            "two conditions",
            "conditions = [\"reduced_motion\"]",
            "conditions = [\"bold_text\"]",
        ),
        (
            "two flags",
            "states = [\"hover\"]",
            "states = [\"pressed\"]",
        ),
        (
            "two axes",
            "size_class = { horizontal = [\"compact\"] }",
            "size_class = { vertical = [\"expanded\"] }",
        ),
        (
            "first and last",
            "position = { first = true }",
            "position = { last = true }",
        ),
        (
            "two tiled edges",
            "conditions = [\"tiled:top\"]",
            "not_conditions = [\"tiled:left\"]",
        ),
    ];
    for (name, first, second) in independent {
        assert_eq!(pair(first, second, ""), [REFUSED], "{name}");
    }
    // Sizes and densities: the widget declares no default size and two
    // densities, so each size and density a rule names is the sizes unit's,
    // refused while it is not enabled (S12; ADD-A147); their exclusions
    // come with that unit.
    let sized: &[(&str, &str, &str)] = &[
        (
            "control size",
            "sizes = [\"small\"]",
            "sizes = [\"regular\"]",
        ),
        (
            "density",
            "densities = [\"default\"]",
            "densities = [\"compact\"]",
        ),
        (
            "an ancestor's size",
            "context = [{ scope = { nearest = [\"button\"] }, widget = \"button\", sizes = [\"small\"] }]",
            "context = [{ scope = { nearest = [\"button\"] }, widget = \"button\", sizes = [\"regular\"] }]",
        ),
        (
            "size and density",
            "sizes = [\"small\"]",
            "densities = [\"compact\"]",
        ),
        (
            "any ancestors' sizes",
            "context = [{ widget = \"button\", sizes = [\"small\"] }]",
            "context = [{ widget = \"button\", sizes = [\"regular\"] }]",
        ),
    ];
    for (name, first, second) in sized {
        let problems = pair(first, second, "");
        assert!(
            problems
                .iter()
                .any(|problem| problem.ends_with("and a later unit states it")),
            "{name}: {problems:?}"
        );
    }
}

/// A binding a later binding of the same role replaces is never evaluated
/// (S6.1: the bindings merge by role), so an operand absent above it is no
/// problem; the later one's is.
#[test]
fn combination_replaced_binding_not_evaluated() {
    let decl = declare("surface_sets = [\"window\"]");
    let below = "[colors.light.surfaces.window]\nneutral = \"#00ff00\"\n\
                 [colors.dark.surfaces.window]\nneutral = \"#00ff00\"\n\
                 [widgets.button.body]\nbindings = [{ role = { role = { surface = \"neutral\" }, \
                 value = \"= mix(role.window.neutral, #000000, 0.5)\" } }]";
    let top = "[colors.light.surfaces.window]\nneutral = \"absent\"\n\
               [colors.dark.surfaces.window]\nneutral = \"absent\"\n\
               [widgets.button.body]\nbindings = [{ role = { role = { surface = \"neutral\" }, \
               value = \"#ff0000\" } }]";
    assert_eq!(over(below, top, &decl), NONE);
}

/// A cycle through two bindings of one part (S12: "through bindings
/// too"): the state layer's colour reads the role the first binding binds
/// and the role the second binds, whose value reads the state layer's
/// colour; the second binding's own check finds it.
#[test]
fn combination_cycle_through_two_bindings() {
    let decl = declare("surface_sets = [\"window\"]");
    let preset = "[colors.light.surfaces.window]\nneutral = \"#00ff00\"\n\
                  [colors.dark.surfaces.window]\nneutral = \"#00ff00\"\n\
                  [colors.light.global]\naccent = \"#0000ff\"\n\
                  [colors.dark.global]\naccent = \"#0000ff\"";
    let top = "[widgets.button.body]\n\
               interaction.state_layer.color = \"= mix(role.global.accent, surface.neutral, 0.5)\"\n\
               bindings = [\
               { role = { role = { global = \"accent\" }, value = \"= mix(role.window.neutral, #000000, 0.5)\" } }, \
               { role = { role = { surface = \"neutral\" }, value = \"= part.interaction.state_layer.color\" } }]";
    let found = over(preset, top, &decl);
    assert!(
        found.iter().any(
            |problem| problem.starts_with("widgets.button.body.bindings[1]")
                && problem.contains("itself")
        ),
        "{found:?}"
    );
}

/// An input reading itself, directly or through another input, is a cycle
/// (S12), refused at validation as evaluation refuses it.
#[test]
fn combination_input_cycle() {
    let decl = declare("inputs = [\"accent\", \"other\"]");
    assert_eq!(
        alone("[inputs.accent]\nlight = \"= input.accent\"", &decl),
        ["inputs.accent: the expression reads itself"]
    );
    assert_eq!(
        alone(
            "[inputs.accent]\nlight = \"= input.other\"\n[inputs.other]\nlight = \"= mix(input.accent, #000000, 0.5)\"",
            &decl
        ),
        [
            "inputs.accent: the expression reads itself",
            "inputs.other: the expression reads itself"
        ]
    );
    assert_eq!(
        alone(
            "[inputs.accent]\nlight = \"#ff0000\"\n[inputs.other]\nlight = \"= input.accent\"",
            &decl
        ),
        NONE
    );
    // One scheme at a time: under light accent reads other's `#000000`,
    // under dark other reads accent's `#ffffff`.
    assert_eq!(
        alone(
            "[inputs.accent]\nlight = \"= input.other\"\ndark = \"#ffffff\"\n\
             [inputs.other]\nlight = \"#000000\"\ndark = \"= input.accent\"",
            &decl
        ),
        NONE
    );
    // The values in force: the top layer's accent replaces the preset's.
    assert_eq!(
        over(
            "[inputs.accent]\nlight = \"= input.other\"\n[inputs.other]\nlight = \"#000000\"",
            "[inputs.accent]\nlight = \"#ff0000\"\n[inputs.other]\nlight = \"= input.accent\"",
            &decl
        ),
        NONE
    );
    // Through a colour role.
    assert_eq!(
        alone(
            "[inputs.other]\nlight = \"= mix(role.global.accent, #000000, 0.5)\"\n\
             [colors.light.global]\naccent = \"= input.other\"",
            &decl
        ),
        ["inputs.other: the expression reads itself"]
    );
    // Under one contrast alone.
    assert_eq!(
        alone(
            "[inputs.other]\nlight = \"#000000\"\n[inputs.other.contrast.high]\n\
             light = \"= input.other\"",
            &decl
        ),
        ["inputs.other: the expression reads itself"]
    );
}

/// An input read through `active(…)` (S12, ADD-A144): its cycle check
/// follows the role to its bases alone, so a port's inactive input
/// `= active(role.<set>.<role>)` that the role's inactive rule reads is no
/// cycle, while one the role's bases read back is. The input is validated
/// as a colour role's expression: one naming an undeclared role is
/// refused.
#[test]
fn combination_input_cycle_through_active() {
    let decl = declare("surface_sets = [\"view\"]\ninputs = [\"inactive.view.foreground_link\"]");
    // The cycles found; the view set's other roles, which these layers
    // leave unstated, are no concern here.
    let cycles = |top: &str| -> Vec<String> {
        alone(top, &decl)
            .into_iter()
            .filter(|problem| problem.contains("reads itself"))
            .collect()
    };
    let input = "[inputs.inactive.view.foreground_link]\nlight = \"= active(role.view.link)\"\n";
    let rule = "[[rules]]\nwhen = { conditions = [\"window_inactive\"] }\ntarget = \"colors\"\n\
                [rules.set]\nsurfaces.view.link = \"= input.inactive.view.foreground_link\"\n";
    assert_eq!(
        cycles(&format!(
            "{input}[colors.light.surfaces.view]\nlink = \"#102030\"\n{rule}"
        )),
        NONE
    );
    assert_eq!(
        cycles(&format!(
            "{input}[colors.light.surfaces.view]\n\
             link = \"= input.inactive.view.foreground_link\"\n"
        )),
        ["inputs.inactive.view.foreground_link: the expression reads itself"]
    );
    // A role of an undeclared set, refused as in a colour role.
    let found = alone(
        "[inputs.inactive.view.foreground_link]\nlight = \"= active(role.window.link)\"\n",
        &decl,
    );
    assert!(
        found
            .iter()
            .any(|problem| problem.starts_with("inputs.inactive.view.foreground_link")),
        "{found:?}"
    );
}

/// Task 11e's review: where the reader's common accent is left out (S8,
/// S9.5), the reader layer's `accent` is not in force, so the preset's two
/// inputs that read each other are a cycle, as evaluation finds it.
#[test]
fn combination_input_cycle_reader_accent_left_out() {
    let decl = declare("inputs = [\"accent\", \"other\"]\naccent_inputs = [\"accent\", \"other\"]");
    let preset = toml(
        "[inputs.accent]\nlight = \"= input.other\"\n[inputs.other]\nlight = \"= input.accent\"",
    );
    let below = [Stacked {
        layer: &preset,
        kind: LayerKind::Preset,
        left_out: None,
    }];
    let reader = toml("[inputs.accent]\nlight = \"#3366ff\"");
    let found = |left_out: bool| {
        let cx = Context {
            declarations: &decl,
            preset: None,
            kind: LayerKind::Reader,
            reader_accent_left_out: left_out,
            below: &below,
        };
        match check(&[&reader], &cx, &Reused::default()) {
            Ok(()) => Vec::new(),
            Err(LayerError::Invalid(problems)) => problems.iter().map(Problem::to_string).collect(),
            Err(other) => panic!("{other}"),
        }
    };
    assert_eq!(found(false), NONE);
    assert_eq!(
        found(true),
        [
            "inputs.accent: the expression reads itself",
            "inputs.other: the expression reads itself"
        ]
    );
}

/// A reference to a text style whose `decoration` is absent, directly or
/// through another reference, brings no field of the decoration: an absent
/// block has none (S2.2, S2.8, S12). A drawn decoration's field stated
/// absent is still refused.
#[test]
fn combination_reference_through_an_absent_block() {
    let decl = declare("text_styles = [\"a\"]\n");
    let base = "[tokens.text]\nkeyed.a.decoration = \"absent\"\n\
                keyed.a.mnemonic_decoration = \"absent\"\n";
    assert_eq!(
        over(
            base,
            "[tokens.text]\nroles.body = \"= token.text.keyed.a\"\n",
            &decl
        ),
        NONE
    );
    assert_eq!(
        over(
            base,
            "[tokens.text]\nroles.body = \"= token.text.keyed.a\"\n\
             roles.monospace = \"= token.text.roles.body\"\n",
            &decl
        ),
        NONE
    );
    let leaf = "[tokens.text]\nkeyed.a.decoration.thickness_px = \"absent\"\n";
    assert_eq!(
        over(
            leaf,
            "[tokens.text]\nroles.body = \"= token.text.keyed.a\"\n",
            &decl
        ),
        ["tokens.text.roles.body.decoration.thickness: may not be absent"]
    );
}

/// A field absent by one step is refused even where another step states
/// its block absent at that place: the block's absence excuses only the
/// fields its own step makes absent (S6.3, S12).
#[test]
fn combination_field_absent_by_another_step_than_its_block() {
    let decl = declare("text_styles = [\"a\"]\n");
    let base = "[tokens.text]\nkeyed.a.decoration = \"absent\"\n\
                keyed.a.mnemonic_decoration = \"absent\"\n";
    assert_eq!(
        over(
            base,
            "[tokens.text]\nroles.body = \"= token.text.keyed.a\"\n[[rules]]\n\
             when = { conditions = [\"contrast_high\"] }\ntarget = \"tokens\"\n\
             set = { text.roles.body.decoration.thickness_px = \"absent\" }\n",
            &decl
        ),
        ["rules[0].set.text.roles.body.decoration.thickness_px: may not be absent",]
    );
}

/// A block stated absent whole leaves no field to check: an absent glyph
/// is no glyph whose `fill` is absent beside an absent stroke (S2.9, S6.3,
/// S12), while a drawn glyph's absent `fill` beside an absent stroke is
/// still refused (`combination_glyph_fill_absent`).
#[test]
fn combination_absent_block_has_no_conditional_field() {
    assert_eq!(bare("[widgets.button.icon]\nglyph = \"absent\""), NONE);
}

/// Layers one check adds at once, the synthetic route's (S17, ADD-A131):
/// the checks over combinations run once over all of them, each problem
/// going to the layer stating the place it names, and each layer's list
/// holding its own and those of the layers below it, as each one's check
/// over the layers below it finds them. Two layers, constant values at
/// disjoint places, each refused where the preset draws what it makes
/// absent: the first an edge's `dash`, the second an icon's `fill`.
#[test]
fn layers_added_at_once_each_problem_its_layer() {
    let preset = toml(
        "[widgets.button.body.stroke.top]\nwidth_px = 1\npaint = \"#000000\"\ndash = \
         \"solid\"\nsnap = \"none\"\n[widgets.button.icon.icon]\nsize_px = 16\nfill = \
         \"#000000\"\nrendering_mode = \"template\"",
    );
    let dash = toml("[widgets.button.body.stroke.top]\ndash = \"absent\"");
    let fill = toml("[widgets.button.icon.icon]\nfill = \"absent\"");
    let decl = Declarations::default();
    let stacked = |layer, kind| Stacked {
        layer,
        kind,
        left_out: None,
    };
    let layers = [
        stacked(&preset, LayerKind::Preset),
        stacked(&dash, LayerKind::Application),
        stacked(&fill, LayerKind::Application),
    ];
    let cx = |below| Context {
        declarations: &decl,
        preset: None,
        kind: LayerKind::Application,
        reader_accent_left_out: false,
        below,
    };
    let strings = |result: Result<(), LayerError>| match result {
        Ok(()) => Vec::new(),
        Err(LayerError::Invalid(problems)) => {
            problems.iter().map(Problem::to_string).collect::<Vec<_>>()
        }
        Err(other) => panic!("{other}"),
    };
    let found: Vec<(usize, String)> =
        super::combinations(&layers, cx(&layers[..1]), &Reused::default())
            .into_iter()
            .map(|(layer, problem)| (layer, problem.to_string()))
            .collect();
    let dash_absent = "widgets.button.body.stroke.top.dash: may not be absent";
    let fill_absent = "widgets.button.icon.icon.fill: may not be absent";
    assert_eq!(
        found,
        [(1, dash_absent.to_owned()), (2, fill_absent.to_owned())]
    );
    let once = strings(check(
        &[&dash, &fill],
        &cx(&layers[..1]),
        &Reused::default(),
    ));
    assert_eq!(once, [dash_absent, fill_absent]);
    assert_eq!(
        strings(check(&[&dash], &cx(&layers[..1]), &Reused::default())),
        [dash_absent]
    );
    assert_eq!(
        strings(check(&[&fill], &cx(&layers[..2]), &Reused::default())),
        [dash_absent, fill_absent]
    );
}

/// A keyframe's patch is checked over its part (S2.7, S12; ADD-A125): its
/// `base` is the part's value beneath it, wherever the keyframe stands, and
/// a field whose value on the part resolves `Absent`, or every field where
/// the part is not drawn or drawn as another, is not evaluated.
#[test]
#[ignore = "motion unit: `IconStyle.animation` is staged until it is enabled (D6 outcome 3)"]
fn keyframe_checked_over_its_part() {
    let stated = |part: &str, presence: &str, patch: &str| {
        format!(
            "[widgets.button.body]\nmin_width_px = \"auto\"\nmin_height_px = 20\n\
             touch_target_px = \"absent\"\nmargin = {{ top_px = \"absent\", right_px = 0, \
             bottom_px = 0, left_px = 0 }}\n[widgets.button.menu_indicator]\n{presence}\
             min_height_px = 4\n[widgets.button.icon.icon.animation]\nperiod_ms = 100\n\
             timing = \"linear\"\nkeyframes = [{{ at = 0, part = \"{part}\", timing = \"absent\", \
             patch = {{ {patch} }} }}]"
        )
    };
    let looped = |part: &str, patch: &str| stated(part, "", patch);
    let at = "widgets.button.icon.icon.animation.keyframes[0].patch";
    assert_eq!(
        bare(&looped("body", "min_width_px = \"= add(base, 2px)\"")),
        [format!("{at}.min_width_px: `base` can be `auto` here")]
    );
    assert_eq!(
        bare(&looped("body", "margin.top_px = \"= add(base, 1px)\"")),
        NONE,
        "the part's margin's top is absent"
    );
    let touch = "min_height_px = \"= add(widget.body.touch_target, 1px)\"";
    let refused = [format!(
        "{at}.min_height_px: `widget.body.touch_target` can be `absent` here"
    )];
    assert_eq!(bare(&looped("body", touch)), refused);
    assert_eq!(bare(&looped("menu_indicator", touch)), refused);
    let hidden = stated("menu_indicator", "absent = true\n", touch);
    assert_eq!(bare(&hidden), NONE, "the part is not drawn");
    let decl = declare("[declare.widget.button]\nvariants = [\"@inner\"]");
    let drawn_as = format!(
        "{}\n[[rules]]\nwhen = {{ not_variants = [\"@inner\"] }}\ntarget = \
         \"button.menu_indicator\"\nset = {{ drawn_as = {{ widget = \"button\", variants = \
         [\"@inner\"] }} }}",
        looped("menu_indicator", touch)
    );
    assert_eq!(
        alone(&drawn_as, &decl),
        refused,
        "drawn as another without the variant, as itself only with it"
    );
    let as_label = format!(
        "{}\n[[rules]]\ntarget = \"button.menu_indicator\"\nset = {{ drawn_as = {{ widget = \
         \"button\", part = \"label\", variants = [] }} }}",
        looped("menu_indicator", touch)
    );
    assert_eq!(bare(&as_label), NONE, "drawn as another part");
    let appended = looped("body", "fill = { append = [\"#ff0000\"] }");
    assert_eq!(
        bare(&appended),
        NONE,
        "`append` in a keyframe's patch in the bases"
    );
}
