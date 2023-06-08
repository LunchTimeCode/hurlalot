mod app;
mod editor;

mod web {
    use eframe::WebRunner;
    use wasm_bindgen::prelude::*;

    use crate::app;

    #[derive(Clone)]
    #[wasm_bindgen]
    pub struct WebHandle {
        runner: eframe::WebRunner,
    }

    #[wasm_bindgen]
    impl WebHandle {
        /// Installs a panic hook, then returns.
        #[allow(clippy::new_without_default)]
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            // Redirect [`log`] message to `console.log` and friends:
            eframe::WebLogger::init(log::LevelFilter::Debug).ok();

            Self {
                runner: WebRunner::new(),
            }
        }

        /// Call this once from JavaScript to start your app.
        #[wasm_bindgen]
        pub async fn start(&self, canvas_id: &str) -> Result<(), wasm_bindgen::JsValue> {
            self.runner
                .start(
                    canvas_id,
                    eframe::WebOptions::default(),
                    Box::new(|cc| Box::new(app::HApp::new(cc))),
                )
                .await
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    wasm_bindgen_futures::spawn_local(async {
        let w = web::WebHandle::new();
        w.start("hurlalot").await.expect("failed to start in web")
    });
}
