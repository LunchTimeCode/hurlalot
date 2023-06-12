use hurl_core::ast::{Entry, HurlFile};

pub trait View {
    fn render(&self, ui: &mut egui::Ui);
}

pub trait EnumeratedView {
    fn render(&self, num: usize, ui: &mut egui::Ui);
}

impl View for HurlFile {
    fn render(&self, ui: &mut egui::Ui) {
        let title = if self.entries.is_empty() {
            "empty file"
        } else {
            "file"
        };
        ui.heading(title);
        for (entry_num, entry) in self.entries.iter().enumerate() {
            entry.render(entry_num, ui)
        }
    }
}

impl EnumeratedView for Entry {
    fn render(&self, num: usize, ui: &mut egui::Ui) {
        ui.add_space(5.0);

        ui.separator();
        ui.label(format!("Entry {num}"));
        ui.add_space(1.0);
        ui.label(self.request.method.to_string());
    }
}
