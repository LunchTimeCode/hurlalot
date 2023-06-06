use egui::{Context, Ui};

mod buffer;
mod hurl_ext;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub(crate) struct Editor {
    view: buffer::View,
}

impl Editor {
    pub fn render(&mut self, ui: &mut Ui) {
        self.view.render(ui)
    }

    pub fn render_buffers(&mut self, ctx: &Context) {
        self.view.render_buffers(ctx)
    }

    pub fn add_buffer(&mut self, title: String, file: String) {
        self.view.add_buffer(title, file);
    }
}
