mod file;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct HurlApp {
    command_center_open: bool,
    file: file::View,
}

impl HurlApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for HurlApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        catppuccin_egui::set_theme(ctx, catppuccin_egui::FRAPPE);

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| self.file.render(ui, ctx));
        });

        egui::SidePanel::left("left").show(ctx, |ui| self.file.render_editor(ui));

        egui::CentralPanel::default().show(ctx, |_| self.file.render_buffers(ctx));
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
