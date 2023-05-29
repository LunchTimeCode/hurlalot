use std::collections::HashMap;

use egui::Ui;
use egui_extras::{Column, TableBuilder};
use log::info;
use std::sync::mpsc::{Receiver, Sender};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub(crate) struct Client {
    #[serde(skip)]
    tx: Sender<RawResponse>,
    #[serde(skip)]
    rx: Receiver<RawResponse>,
    #[serde(skip)]
    incoming: RawResponse,
    #[serde(skip)]
    outgoing: RawRequest,
}

#[derive(Debug, Default)]
struct RawResponse {
    headers: HashMap<String, String>,
    body: String,
}

#[derive(Debug, Default)]
struct RawRequest {}

impl Default for Client {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        Self {
            tx,
            rx,
            incoming: RawResponse::default(),
            outgoing: RawRequest::default(),
        }
    }
}

impl Client {
    pub fn render(&mut self, ui: &mut Ui) {
        // Update the counter with the async response.
        if let Ok(response) = self.rx.try_recv() {
            self.incoming = response
        }
        ui.label("Press the button to initiate an HTTP request.");

        if ui.button("send").clicked() {
            send_req(self.tx.clone(), &self.outgoing);
        }
        ui.collapsing("raw response", |ui| {
            ui.label("headers");
            egui::ScrollArea::vertical()
                .id_source("first")
                .max_height(400.0)
                .show(ui, |ui| {
                    TableBuilder::new(ui)
                        .striped(true)
                        .column(Column::remainder())
                        .column(Column::remainder())
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.heading("key");
                            });
                            header.col(|ui| {
                                ui.heading("value");
                            });
                        })
                        .body(|mut body| {
                            for (key, value) in &self.incoming.headers {
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(key);
                                    });
                                    row.col(|ui| {
                                        ui.label(value);
                                    });
                                });
                            }
                        });

                    ui.separator();
                    ui.label("body");

                    ui.label(&self.incoming.body);
                });
        });
    }
}

fn send_req(tx: Sender<RawResponse>, _req: &RawRequest) {
    tokio::spawn(async move {
        info!("respond sending");
        // Send a request with an increment value.
        let res = reqwest::Client::default()
            .post("https://httpbin.org/anything")
            .send()
            .await
            .expect("Unable to send request");

        info!("respond received");

        let mut headers: HashMap<String, String> = HashMap::new();

        for (key, value) in res.headers().into_iter() {
            headers.insert(
                key.to_string(),
                value.to_str().unwrap_or("nothing found").into(),
            );
        }

        let body = res.text().await.unwrap_or("nothing in body".into());

        let raw = RawResponse { headers, body };

        info!("raw: {:#?}", raw);

        // After parsing the response, notify the GUI thread of the increment value.
        let _ = tx.send(raw);
    });
}
