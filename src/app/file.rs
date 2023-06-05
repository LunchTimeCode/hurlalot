use std::{ffi::OsStr, fs, path::PathBuf};

use egui::{Context, Ui};
use egui_file::FileDialog;
use log::info;

use crate::editor::Editor;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct View {
    opened_file: Option<PathBuf>,
    #[serde(skip)]
    open_file_dialog: Option<FileDialog>,
    editor: Editor,
}

impl View {
    pub fn render(&mut self, ui: &mut Ui, ctx: &Context) {
        ui.menu_button("menu", |ui| {
            if (ui.button("Open")).clicked() {
                let mut dialog = FileDialog::open_file(self.opened_file.clone());
                dialog.open();
                self.open_file_dialog = Some(dialog);
            }
        });

        if let Some(dialog) = &mut self.open_file_dialog {
            if dialog.show(ctx).selected() {
                if let Some(file) = dialog.path() {
                    let name = file
                        .file_name()
                        .and_then(OsStr::to_str)
                        .unwrap_or_default()
                        .to_owned();

                    let content =
                        fs::read_to_string(file).expect("Should have been able to read the file");

                    info!("chosen file");
                    self.editor.add_buffer(name, content);
                }
            }
        }
    }
    pub fn render_editor(&mut self, ui: &mut Ui) {
        self.editor.render(ui);
    }
    pub fn render_buffers(&mut self, ctx: &Context) {
        self.editor.render_buffers(ctx);
    }
}
