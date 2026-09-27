//! Text (spec §10.4's palette table): labels in the base style's five `TextStyle` slots,
//! one label per `TextRole` in the text-role accessors, `Role::Link`, `Role::Separator`,
//! and egui_extras's code view in the base style.

use egui_extras::syntax_highlighting::{CodeTheme, code_view_ui};
use native_theme_egui::{
    Role, RoleVariant, TextRole, ThemeAtlas, text_role_font, text_role_line_height,
    text_role_weight,
};

use super::{DemoState, caption};
use crate::demo::{self, Registry};

const LONG: &str = "A label long enough to wrap onto a second line where the page is narrow, \
    and to be cut short where it is truncated instead.";
const REPOSITORY: &str = "https://github.com/tiborgats/native-theme";
const TEXT_ROLES: [(TextRole, &str); 4] = [
    (TextRole::Caption, "Caption"),
    (TextRole::SectionHeading, "Section heading"),
    (TextRole::DialogTitle, "Dialog title"),
    (TextRole::Display, "Display"),
];

pub(crate) fn show(
    reg: &mut Registry,
    state: &mut DemoState,
    atlas: &ThemeAtlas,
    ui: &mut egui::Ui,
) {
    let t = atlas.resolved_for(ui.ctx().theme());
    let prefs = atlas.accessibility();
    let normal = RoleVariant::Normal;

    caption(reg, ui, "Label (the base style)");
    demo::base(reg, ui, "Label (wrapped)", |ui| {
        ui.add(egui::Label::new(LONG).wrap())
    });
    demo::base(reg, ui, "Label (truncated)", |ui| {
        ui.add(egui::Label::new(LONG).truncate())
    });
    demo::base(reg, ui, "Label (selectable)", |ui| {
        ui.add(egui::Label::new("A selectable label").selectable(true))
    });
    ui.horizontal_wrapped(|ui| {
        demo::base(reg, ui, "ui.label", |ui| ui.label("ui.label"));
        let warn = ui.visuals().warn_fg_color;
        demo::base(reg, ui, "ui.colored_label", |ui| {
            ui.colored_label(warn, "ui.colored_label")
        });
        demo::base(reg, ui, "ui.heading", |ui| ui.heading("ui.heading"));
        demo::base(reg, ui, "ui.monospace", |ui| ui.monospace("ui.monospace"));
        demo::base(reg, ui, "ui.code", |ui| ui.code("ui.code"));
        demo::base(reg, ui, "ui.small", |ui| ui.small("ui.small"));
        demo::base(reg, ui, "ui.strong", |ui| ui.strong("ui.strong"));
        demo::base(reg, ui, "ui.weak", |ui| ui.weak("ui.weak"));
    });

    caption(
        reg,
        ui,
        "Text roles (text_role_font, text_role_line_height, text_role_weight)",
    );
    for (role, name) in TEXT_ROLES {
        let font = text_role_font(t, role, prefs);
        let line_height = text_role_line_height(t, role, prefs);
        let weight = text_role_weight(t, role);
        let text = egui::RichText::new(format!("{name}, weight {weight}"))
            .font(font.clone())
            .line_height(Some(line_height));
        demo::base(reg, ui, "text role label", |ui| ui.label(text));
        reg.amend_last(|i| {
            i.read.push(("text_role_font", format!("{font:?}")));
            i.read
                .push(("text_role_line_height", format!("{line_height}")));
            i.read.push(("text_role_weight", format!("{weight}")));
        });
    }

    caption(reg, ui, "Links (Role::Link)");
    // One scope for the enabled links, so the row wraps between them; the disabled one in its
    // own variant's scope, on a line of its own.
    demo::scoped_container(reg, ui, Role::Link, normal, "links", |ui, link, reg| {
        ui.horizontal_wrapped(|ui| {
            link.add(reg, ui, "Hyperlink", |ui| {
                ui.add(egui::Hyperlink::new(REPOSITORY))
            });
            link.add(reg, ui, "Link", |ui| ui.add(egui::Link::new("Link")));
            link.add(reg, ui, "ui.link", |ui| ui.link("ui.link"));
            link.add(reg, ui, "ui.hyperlink", |ui| ui.hyperlink(REPOSITORY));
            link.add(reg, ui, "ui.hyperlink_to", |ui| {
                ui.hyperlink_to("ui.hyperlink_to", REPOSITORY)
            });
        })
        .response
    });
    demo::scoped(
        reg,
        ui,
        Role::Link,
        RoleVariant::Disabled,
        "Link (disabled)",
        |ui| ui.add_enabled(false, egui::Link::new("Disabled link")),
    );

    caption(reg, ui, "Separators (Role::Separator)");
    demo::scoped(
        reg,
        ui,
        Role::Separator,
        normal,
        "Separator (horizontal)",
        |ui| ui.add(egui::Separator::default().horizontal()),
    );
    ui.horizontal(|ui| {
        demo::base(reg, ui, "caption", |ui| ui.label("Left"));
        demo::scoped(
            reg,
            ui,
            Role::Separator,
            normal,
            "Separator (vertical)",
            |ui| ui.add(egui::Separator::default().vertical()),
        );
        demo::base(reg, ui, "caption", |ui| ui.label("Right"));
    });
    demo::scoped(reg, ui, Role::Separator, normal, "ui.separator", |ui| {
        ui.separator()
    });

    caption(reg, ui, "Code view and its theme editor (the base style)");
    // Made again when the scheme or the `Monospace` font changes — a mode switch, an install —
    // and kept, with the editor's changes, until then.
    let style = ui.style();
    let font = style
        .override_font_id
        .clone()
        .unwrap_or_else(|| egui::TextStyle::Monospace.resolve(style));
    let key = (style.visuals.dark_mode, font);
    if state
        .code_theme
        .as_ref()
        .is_some_and(|(held, _)| *held != key)
    {
        state.code_theme = None;
    }
    let (_, theme) = state
        .code_theme
        .get_or_insert_with(|| (key, CodeTheme::from_style(style)));
    let code = state.code.clone();
    demo::base(reg, ui, "code_view_ui", |ui| {
        code_view_ui(ui, theme, &code, "rs")
    });
    demo::base(reg, ui, "CodeTheme editor", |ui| {
        ui.scope(|ui| theme.ui(ui)).response
    });
}
