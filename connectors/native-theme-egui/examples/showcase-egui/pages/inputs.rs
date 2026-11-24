//! Inputs (spec §10.4's palette table): `Role::Input` for every `TextEdit` and `DragValue`,
//! each `TextEdit` the page builds given `input_frame`; the completion popup and the date picker
//! in the base style.

use native_theme_egui::{Role, RoleVariant, next};

use super::{DemoState, Framed, caption, field_frame, input_min_size, text_area_frame};
use crate::app::OldAtlas;
use crate::demo::{self, Registry};

pub(crate) fn show(reg: &mut Registry, state: &mut DemoState, atlas: &OldAtlas, ui: &mut egui::Ui) {
    let r = atlas.next_shown(ui.ctx());
    let normal = RoleVariant::Normal;
    // Each `TextEdit` the page builds at least `input.field(NONE).min_size[1]` tall, a single
    // line's row centred in the height the minimum adds, as the platform centres a field's text
    // and the Basic page's fields are.
    let min = input_min_size(r);

    caption(reg, ui, "TextEdit with input_frame (Role::Input)");
    let id = ui.make_persistent_id("inputs/single-line");
    demo::scoped(
        reg,
        ui,
        Role::Input,
        normal,
        "TextEdit (single line)",
        |ui| {
            let frame = field_frame(ui, id, r, next::input_frame);
            ui.add(
                egui::TextEdit::singleline(&mut state.text)
                    .min_size(min)
                    .align(egui::Align2::LEFT_CENTER)
                    .id(id)
                    .framed(frame),
            )
            .accessible_name("TextEdit (single line)")
        },
    );
    let id = ui.make_persistent_id("inputs/multiline");
    let mut notes = Vec::new();
    demo::scoped(reg, ui, Role::Input, normal, "TextEdit (multiline)", |ui| {
        let frame;
        (frame, notes) = text_area_frame(ui, id, r);
        ui.add(
            egui::TextEdit::multiline(&mut state.multiline)
                .min_size(min)
                .id(id)
                .framed(frame),
        )
        .accessible_name("TextEdit (multiline)")
    });
    for note in notes {
        reg.note_last("frame", &format!("{note:?}"));
    }
    let id = ui.make_persistent_id("inputs/password");
    demo::scoped(reg, ui, Role::Input, normal, "TextEdit (password)", |ui| {
        let frame = field_frame(ui, id, r, next::input_frame);
        ui.add(
            egui::TextEdit::singleline(&mut state.password)
                .password(true)
                .min_size(min)
                .align(egui::Align2::LEFT_CENTER)
                .id(id)
                .framed(frame),
        )
        .accessible_name("TextEdit (password)")
    });
    let id = ui.make_persistent_id("inputs/hint");
    demo::scoped(reg, ui, Role::Input, normal, "TextEdit (hint)", |ui| {
        let frame = field_frame(ui, id, r, next::input_frame);
        ui.add(
            egui::TextEdit::singleline(&mut state.hint)
                .hint_text("A hint")
                .min_size(min)
                .align(egui::Align2::LEFT_CENTER)
                .id(id)
                .framed(frame),
        )
    });
    let id = ui.make_persistent_id("inputs/disabled");
    let mut disabled = "Disabled".to_string();
    demo::scoped(
        reg,
        ui,
        Role::Input,
        RoleVariant::Disabled,
        "TextEdit (disabled)",
        |ui| {
            let frame = field_frame(ui, id, r, next::input_frame);
            ui.add_enabled(
                false,
                egui::TextEdit::singleline(&mut disabled)
                    .min_size(min)
                    .align(egui::Align2::LEFT_CENTER)
                    .id(id)
                    .framed(frame),
            )
            .accessible_name("TextEdit (disabled)")
        },
    );

    // egui's completion popup under a field of the input scope: the field is the `TextEdit` the
    // popup is handed, framed by `input_frame`; the popup is a `Popup` the scope does not reach
    // (`CompletionPopup` takes no frame and no style), in the base style.
    caption(
        reg,
        ui,
        "CompletionPopup under a TextEdit (Role::Input, its popup in the base style)",
    );
    let id = ui.make_persistent_id("inputs/completion");
    demo::scoped(reg, ui, Role::Input, normal, "CompletionPopup", |ui| {
        let frame = field_frame(ui, id, r, next::input_frame);
        egui::CompletionPopup::new(id)
            .show(
                ui,
                egui::TextEdit::singleline(&mut state.completion)
                    .hint_text("A bundled preset's name")
                    .min_size(min)
                    .align(egui::Align2::LEFT_CENTER)
                    .framed(frame),
                |query| preset_suggestions(query.word),
            )
            .text_edit
            .response
            .response
            .accessible_name("CompletionPopup")
    });
    reg.note_last(
        "the popup",
        "open while the field has focus and a bundled preset's name begins with the word at the \
         cursor: a Popup under the field in the base style, which the input scope does not \
         reach (CompletionPopup sets no frame and no style, egui-0.37.0 \
         src/widgets/text_edit/completion.rs:169, :240), framed by Frame::popup \
         (src/containers/frame.rs:214: the window fill and stroke, menu_margin, \
         menu_corner_radius and popup_shadow), one Button::selectable row per suggestion, the \
         selected row in the selection fill and each row in its state's widget visuals",
    );

    caption(
        reg,
        ui,
        "The Ui's own text edits, with egui's focused stroke (Role::Input)",
    );
    // egui's own kinds, at egui's own size: `Ui::text_edit_singleline`, `text_edit_multiline` and
    // `code_editor` take no `min_size` (`egui-0.37.0/src/ui.rs:1927-1932`, `:1937-1942`,
    // `:1949-1951`; a multiline `TextEdit` at least four rows tall,
    // `src/widgets/text_edit/builder.rs:147`, `:594`).
    demo::scoped(
        reg,
        ui,
        Role::Input,
        normal,
        "ui.text_edit_singleline",
        |ui| {
            ui.text_edit_singleline(&mut state.text)
                .accessible_name("ui.text_edit_singleline")
        },
    );
    reg.note_last(
        "ui.text_edit_singleline",
        "egui's own height: one row and its margin, at least the scope's interact_size.y \
         (button.body(NONE).min_size[1]); input.field(NONE).min_size[1] is not applied",
    );
    reg.note_last(
        "padding",
        "the input scope's button_padding, one value per axis (egui-0.37.0 \
         src/theme/default_style.rs:155): each the mean of a pair of input.field(NONE).padding's \
         sides, a stated side with input.field(NONE).stroke.width added, so the mean of the two \
         where the theme states different ones (windows-11), and an unstated side the button's \
         padding, egui's own 8 × 2 (src/style.rs:1568) where button.body(NONE).padding states \
         neither side of the pair either; next::input_frame pads each side as stated",
    );
    demo::scoped(
        reg,
        ui,
        Role::Input,
        normal,
        "ui.text_edit_multiline",
        |ui| {
            ui.text_edit_multiline(&mut state.multiline)
                .accessible_name("ui.text_edit_multiline")
        },
    );
    reg.note_last(
        "ui.text_edit_multiline",
        "egui's own height, at least four rows and its margin; input.field(NONE).min_size[1] is not applied",
    );
    demo::scoped(reg, ui, Role::Input, normal, "ui.code_editor", |ui| {
        ui.code_editor(&mut state.code)
            .accessible_name("ui.code_editor")
    });
    reg.note_last(
        "ui.code_editor",
        "egui's own height, at least four rows and its margin; input.field(NONE).min_size[1] is not applied",
    );

    caption(reg, ui, "DragValue (Role::Input)");
    ui.horizontal_wrapped(|ui| {
        demo::scoped(reg, ui, Role::Input, normal, "DragValue", |ui| {
            ui.add(egui::DragValue::new(&mut state.number))
                .accessible_name("DragValue")
        });
        demo::scoped(reg, ui, Role::Input, normal, "ui.drag_angle", |ui| {
            ui.drag_angle(&mut state.angle)
        });
        demo::scoped(reg, ui, Role::Input, normal, "ui.drag_angle_tau", |ui| {
            ui.drag_angle_tau(&mut state.angle)
        });
    });

    caption(reg, ui, "DatePickerButton (the base style)");
    demo::base(reg, ui, "DatePickerButton", |ui| {
        ui.add(egui_extras::DatePickerButton::new(&mut state.date))
    });
}

/// The bundled presets whose key begins with `word`, each a suggestion that inserts it; none for
/// an empty word.
fn preset_suggestions(word: &str) -> Vec<egui::Suggestion> {
    if word.is_empty() {
        return Vec::new();
    }
    native_theme::theme::Theme::list_presets()
        .iter()
        .filter(|preset| preset.key.starts_with(word))
        .map(|preset| egui::Suggestion::new(preset.key))
        .collect()
}
