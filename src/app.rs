use serde::Deserialize;
use serde::Serialize;

use crate::editor;
use egui::{Ui, WidgetText};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct H {
    #[serde(skip)]
    tree: egui_tiles::Tree<Pane>,

    #[serde(skip)]
    behavior: TreeBehavior,
}

impl Default for H {
    fn default() -> Self {
        Self {
            behavior: TreeBehavior {},
            tree: create_tree(),
        }
    }
}

impl H {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }
        H::default()
    }
}

impl eframe::App for H {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO);
        egui::TopBottomPanel::top("top1").show(ctx, |ui| {
            if ui.button("Add new editor (does nothing yet)").clicked() {}
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.tree.ui(&mut self.behavior, ui);
        });
    }
}

fn create_tree() -> egui_tiles::Tree<Pane> {
    let mut tiles = egui_tiles::Tiles::default();

    let mut tabs = vec![];

    let first = Pane {
        title: "Editor".into(),
        module: Module::Editor(EditorState::default()),
    };

    tabs.push({
        let children = tiles.insert_pane(first);
        tiles.insert_horizontal_tile(vec![children])
    });

    let second = Pane {
        title: "Editor".into(),
        module: Module::Editor(EditorState::default()),
    };

    tabs.push({
        let children = tiles.insert_pane(second);
        tiles.insert_horizontal_tile(vec![children])
    });

    let root = tiles.insert_tab_tile(tabs);

    egui_tiles::Tree::new(root, tiles)
}

struct Pane {
    title: String,
    module: Module,
}

enum Module {
    Editor(EditorState),
}

struct TreeBehavior {}

impl egui_tiles::Behavior<Pane> for TreeBehavior {
    fn pane_ui(
        &mut self,
        ui: &mut Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        let module = &mut pane.module;

        ui.add_space(50.0);

        match module {
            Module::Editor(state) => state.render(ui),
        };

        if ui
            .add(egui::Button::new("Drag Handle").sense(egui::Sense::drag()))
            .drag_started()
        {
            egui_tiles::UiResponse::DragStarted
        } else {
            egui_tiles::UiResponse::None
        }
    }

    fn tab_title_for_pane(&mut self, pane: &Pane) -> WidgetText {
        WidgetText::from(pane.title.clone())
    }
}

#[derive(Default)]
pub struct EditorState {
    editor: editor::Editor,
}

impl EditorState {
    pub fn render(&mut self, ui: &mut Ui) {
        self.editor.render(ui);
    }
}
