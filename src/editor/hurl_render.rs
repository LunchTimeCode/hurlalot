use hurl_core::ast::{Entry, HurlFile, Template, TemplateElement};

pub trait View {
    fn render(&self, ui: &mut egui::Ui);
}

pub trait EnumeratedView {
    fn render(&self, num: usize, ui: &mut egui::Ui);
}

impl View for HurlFile {
    fn render(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            let title = if self.entries.is_empty() {
                "empty file"
            } else {
                "file"
            };
            ui.heading(format!("beta file preview {title}"));
            for (entry_num, entry) in self.entries.iter().enumerate().take(2) {
                entry.render(entry_num, ui)
            }
        });
    }
}

impl EnumeratedView for Entry {
    fn render(&self, num: usize, ui: &mut egui::Ui) {
        ui.add_space(5.0);

        ui.label(format!("Entry {num}"));
        ui.add_space(1.0);

        ui.horizontal(|ui| {
            ui.label(self.request.method.to_string());
            ui.add_space(1.0);
            self.request.url.render(ui);
        });
    }
}

impl View for Template {
    fn render(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for element in self.elements.iter() {
                ui.add_space(0.0);
                element.render(ui);
            }
        });
    }
}

impl View for TemplateElement {
    fn render(&self, ui: &mut egui::Ui) {
        match self {
            TemplateElement::String { value, .. } => ui.label(value.trim()),
            TemplateElement::Expression(e) => ui.label("{{".to_string() + &e.variable.name + "}}"),
        };
    }
}
