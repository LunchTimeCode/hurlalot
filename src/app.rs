use crate::editor::Editor;

#[derive(Default)]
pub struct HApp {
    editor: Editor,
}

impl HApp {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }
        Default::default()
    }
}

impl eframe::App for HApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO);

        egui::CentralPanel::default().show(ctx, |ui| self.editor.render(ui));
    }

    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }
}
