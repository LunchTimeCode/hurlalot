use std::{ffi::OsStr, path::PathBuf};

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
                    info!("chosen file");
                    self.editor.add_buffer(
                        file.file_name()
                            .and_then(OsStr::to_str)
                            .unwrap_or_default()
                            .to_owned(),
                        "content".to_owned(),
                    );
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
