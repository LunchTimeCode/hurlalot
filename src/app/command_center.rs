use super::{HurlApp, Module};

pub fn render(ui: &mut egui::Ui, app: &mut HurlApp) {
    egui::Window::new("Tabs")
        .vscroll(true)
        .default_height(300.00)
        .default_width(300.00)
        .open(&mut app.command_center_open)
        .resizable(false)
        .collapsible(false)
        .show(ui.ctx(), |ui| {
            if ui.button("Splitter").clicked()
                && app.tabs.tree.find_tab(&Module::Splitter).is_none()
            {
                app.tabs.tree.push_to_first_leaf(Module::Splitter)
            }

            if ui.button("Connector").clicked()
                && app.tabs.tree.find_tab(&Module::Connector).is_none()
            {
                app.tabs.tree.push_to_first_leaf(Module::Connector)
            }
            if ui.button("Caller").clicked() && app.tabs.tree.find_tab(&Module::Caller).is_none() {
                app.tabs.tree.push_to_first_leaf(Module::Caller)
            }

            if ui.button("Settings").clicked()
                && app.tabs.tree.find_tab(&Module::Settings).is_none()
            {
                app.tabs.tree.push_to_first_leaf(Module::Settings)
            }

            if ui.button("Editor").clicked()
                && app.tabs.tree.find_tab(&Module::Editor).is_none()
            {
                app.tabs.tree.push_to_first_leaf(Module::Editor)
            }
        });
}
