use eframe::egui;

use self::parser::Parser;

mod parser;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Editor {
    current: String,

    text: String,

    extension: String,
    text_changed: bool,
    last_text: String,

    #[serde(skip)]
    parser: Parser,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            current: Default::default(),
            extension: Default::default(),
            text_changed: Default::default(),
            last_text: Default::default(),
            text: Default::default(),
            parser: Default::default(),
        }
    }
}

impl Editor {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        self.text_changed = self.last_text.eq(&self.text);
        self.last_text = self.text.clone();

        if self.text_changed {
            self.parser.parse(&self.text)
        }

        egui::ScrollArea::vertical()
            .id_source("some inner 3")
            .max_height(500.0)
            .show(ui, |ui| {
                ui.push_id("second_some", |ui| {
                    ui.horizontal_top(|ui| {
                        let mut current: String = self
                            .text
                            .lines()
                            .take(1000)
                            .enumerate()
                            .map(|(s, _)| (s + 1).to_string() + "\n")
                            .collect();

                        egui::TextEdit::multiline(&mut current)
                            .interactive(false)
                            .desired_width(30.0)
                            .code_editor()
                            .show(ui);

                        egui::TextEdit::multiline(&mut self.text)
                            .desired_width(f32::INFINITY)
                            .code_editor()
                            .show(ui);
                    });
                });
            });

        ui.add_space(10.0);

        self.parser.render(ui);
    }
}
