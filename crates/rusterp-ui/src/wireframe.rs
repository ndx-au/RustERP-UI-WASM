//! Wireframe stub panels for schema-aligned pages (no fake data).

use crate::shell::{tokens, WireframeMeta};

/// Draw a wireframe placeholder for a schema-backed page.
pub fn draw_wireframe_stub(ui: &mut egui::Ui, page_label: &str, meta: &WireframeMeta) {
    ui.heading(page_label);
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(meta.tier_label)
                .small()
                .color(tokens::ACCENT),
        );
        ui.label(
            egui::RichText::new("Wireframe")
                .small()
                .weak(),
        );
    });
    ui.add_space(4.0);
    ui.label(meta.description);
    ui.add_space(8.0);
    ui.label(
        egui::RichText::new(format!("Schema: {}", meta.schema_path))
            .monospace()
            .small(),
    );
    ui.add_space(12.0);

    let available = ui.available_size();
    let placeholder_height = (available.y - 40.0).max(120.0);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(available.x, placeholder_height),
        egui::Sense::hover(),
    );
    let painter = ui.painter();
    painter.rect_stroke(
        rect,
        egui::CornerRadius::ZERO,
        egui::Stroke::new(1.0, tokens::WIREFRAME_MUTED),
        egui::StrokeKind::Inside,
    );
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "List / detail views will land here",
        egui::FontId::proportional(13.0),
        tokens::WIREFRAME_MUTED,
    );
}
