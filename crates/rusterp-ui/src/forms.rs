//! Shared keyboard form helpers for egui create/edit panels.

/// Result of polling Enter / Escape for a form whose fields reported focus.
#[derive(Debug, Clone, Copy, Default)]
pub struct FormKeys {
    pub submit: bool,
    pub cancel: bool,
}

/// Call after drawing form fields. Pass `true` if any field had focus this frame
/// (`response.has_focus()` / `gained_focus()`).
///
/// Enter submits (without Ctrl/Cmd). Escape cancels.
pub fn form_keys(ui: &egui::Ui, field_focused: bool) -> FormKeys {
    let (enter, escape) = ui.input(|i| {
        (
            i.key_pressed(egui::Key::Enter)
                && !i.modifiers.ctrl
                && !i.modifiers.command
                && !i.modifiers.shift,
            i.key_pressed(egui::Key::Escape),
        )
    });
    FormKeys {
        submit: field_focused && enter,
        cancel: escape,
    }
}

/// Request focus on `id` once when `should` is true, then clear the flag.
pub fn focus_once(ui: &mut egui::Ui, id: egui::Id, should: &mut bool) {
    if *should {
        ui.memory_mut(|m| m.request_focus(id));
        *should = false;
    }
}

/// Draw a single-line field and accumulate focus for Enter-to-submit.
pub fn text_field(
    ui: &mut egui::Ui,
    value: &mut String,
    width: f32,
    hint: &str,
    id: egui::Id,
    focused: &mut bool,
) {
    let response = ui.add(
        egui::TextEdit::singleline(value)
            .id(id)
            .desired_width(width)
            .hint_text(hint),
    );
    *focused |= response.has_focus() || response.gained_focus();
}
