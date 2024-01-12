use renet::{ConnectionConfig, DefaultChannel, RenetClient};
use renet_webtransport::prelude::*;
use std::time::Duration;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub struct ChatApp {
    renet_client: RenetClient,
    web_transport_client: WebTransportClient,
    duration: f64,
    messages: Vec<String>,
}

#[wasm_bindgen]
impl ChatApp {
    pub async fn new() -> Result<ChatApp, JsValue> {
        console_error_panic_hook::set_once();

        log(&"Creating chat app".into());

        let connection_config = ConnectionConfig::default();
        let renet_client = RenetClient::new(connection_config);

        let web_transport_client = WebTransportClient::new("https://172.20.17.234:3080", None).await?;
        log(&"Web transport client created".into());

        Ok(ChatApp {
            renet_client,
            web_transport_client,
            duration: 0.0,
            messages: Vec::with_capacity(20),
        })
    }

    pub fn update(&mut self) {
        self.duration += 0.016;
        self.renet_client.update(Duration::from_secs_f64(self.duration));
        self.web_transport_client.update(&mut self.renet_client);
        if let Some(message) = self.renet_client.receive_message(DefaultChannel::Unreliable) {
            let message = String::from_utf8(message.into()).unwrap();
            self.messages.push(message);
        };
        
    }

    pub fn is_disconnected(&self) -> bool {
        self.web_transport_client.is_disconnected()
    }

    pub fn send_packets(&mut self) {
        self.web_transport_client.send_packets(&mut self.renet_client);
    }

    pub fn send_message(&mut self, message: &str) {
        self.renet_client.send_message(DefaultChannel::Unreliable, message.as_bytes().to_vec());
    }

    pub fn disconnect(self) {
        self.web_transport_client.disconnect();
    }

    pub fn get_messages(&self) -> String {
        self.messages.join("\n")
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }
}
