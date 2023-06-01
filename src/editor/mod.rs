use egui::Ui;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub(crate) struct Editor {
    some_state: String,
}

impl Editor {
    pub fn render(&mut self, ui: &mut Ui) {
        ui.text_edit_multiline(&mut self.some_state);
    }
}
