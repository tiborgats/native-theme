//! Inputs (spec §10.4's palette table): `Role::Input` for every `TextEdit` and `DragValue`,
//! each `TextEdit` the page builds given `input_frame`; the date picker in the base style.

use native_theme_egui::{Role, RoleVariant, ThemeAtlas, input_frame, text_area_frame};

use super::{DemoState, caption};
use crate::demo::{self, Registry};

pub(crate) fn show(
    reg: &mut Registry,
    state: &mut DemoState,
    atlas: &ThemeAtlas,
    ui: &mut egui::Ui,
) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let normal = RoleVariant::Normal;

    caption(reg, ui, "TextEdit with input_frame (Role::Input)");
    let id = ui.make_persistent_id("inputs/single-line");
    let frame = input_frame(ui, id, t);
    demo::scoped(
        reg,
        ui,
        Role::Input,
        normal,
        "TextEdit (single line)",
        |ui| {
            ui.add(
                egui::TextEdit::singleline(&mut state.text)
                    .id(id)
                    .frame(frame),
            )
        },
    );
    let id = ui.make_persistent_id("inputs/multiline");
    let frame = text_area_frame(ui, id, t);
    demo::scoped(reg, ui, Role::Input, normal, "TextEdit (multiline)", |ui| {
        ui.add(
            egui::TextEdit::multiline(&mut state.multiline)
                .id(id)
                .frame(frame),
        )
    });
    let id = ui.make_persistent_id("inputs/password");
    let frame = input_frame(ui, id, t);
    demo::scoped(reg, ui, Role::Input, normal, "TextEdit (password)", |ui| {
        ui.add(
            egui::TextEdit::singleline(&mut state.password)
                .password(true)
                .id(id)
                .frame(frame),
        )
    });
    let id = ui.make_persistent_id("inputs/hint");
    let frame = input_frame(ui, id, t);
    demo::scoped(reg, ui, Role::Input, normal, "TextEdit (hint)", |ui| {
        ui.add(
            egui::TextEdit::singleline(&mut state.hint)
                .hint_text("A hint")
                .id(id)
                .frame(frame),
        )
    });
    reg.amend_last(|i| i.notes.push(("hint text", "\"A hint\"".to_string())));
    let id = ui.make_persistent_id("inputs/disabled");
    let frame = input_frame(ui, id, t);
    let mut disabled = "Disabled".to_string();
    demo::scoped(
        reg,
        ui,
        Role::Input,
        RoleVariant::Disabled,
        "TextEdit (disabled)",
        |ui| {
            ui.add_enabled(
                false,
                egui::TextEdit::singleline(&mut disabled)
                    .id(id)
                    .frame(frame),
            )
        },
    );

    caption(
        reg,
        ui,
        "The Ui's own text edits, with egui's focused stroke (Role::Input)",
    );
    demo::scoped(
        reg,
        ui,
        Role::Input,
        normal,
        "ui.text_edit_singleline",
        |ui| ui.text_edit_singleline(&mut state.text),
    );
    demo::scoped(
        reg,
        ui,
        Role::Input,
        normal,
        "ui.text_edit_multiline",
        |ui| ui.text_edit_multiline(&mut state.multiline),
    );
    demo::scoped(reg, ui, Role::Input, normal, "ui.code_editor", |ui| {
        ui.code_editor(&mut state.code)
    });

    caption(reg, ui, "DragValue (Role::Input)");
    ui.horizontal_wrapped(|ui| {
        demo::scoped(reg, ui, Role::Input, normal, "DragValue", |ui| {
            ui.add(egui::DragValue::new(&mut state.number))
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
