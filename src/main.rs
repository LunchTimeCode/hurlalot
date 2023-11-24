#![warn(clippy::pedantic)]
mod app;
mod editor;
mod hurl_ext;

mod web {
    use crate::app;
    use eframe::WebRunner;
    use wasm_bindgen::prelude::*;

    #[derive(Clone)]
    #[wasm_bindgen]
    pub struct Handle {
        runner: eframe::WebRunner,
    }

    #[wasm_bindgen]
    impl Handle {
        /// Installs a panic hook, then returns.
        #[allow(clippy::new_without_default)]
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            eframe::WebLogger::init(log::LevelFilter::Debug).ok();

            Self {
                runner: WebRunner::new(),
            }
        }

        #[wasm_bindgen]
        pub async fn start(&self, canvas_id: &str) -> Result<(), wasm_bindgen::JsValue> {
            self.runner
                .start(
                    canvas_id,
                    eframe::WebOptions::default(),
                    Box::new(|cc| Box::new(app::H::new(cc))),
                )
                .await
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    wasm_bindgen_futures::spawn_local(async {
        let w = web::Handle::new();
        w.start("hurlalot").await.expect("failed to start in web");
    });
}
