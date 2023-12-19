//! Простеший пример сервера websocket

use serde::{Deserialize, Serialize};
use tokio::{main, time::Duration};
use tracing::Level;

use rsiot_component_core::ComponentChain;
use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
use rsiot_messages_core::IMessage;
use rsiot_websocket_server::cmp_websocket_server;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Message {
    Message0(f64),
    Message1(f64),
    CombineMessage(f64, f64),
}

impl IMessage for Message {
    fn into_eav(self) -> Vec<rsiot_messages_core::eav::EavModel> {
        vec![]
    }
}

#[main]
async fn main() {
    tracing_subscriber::fmt().init();

    let mut counter = 0.0;

    let mut chain = ComponentChain::new(100)
        .add_cmp(cmp_inject_periodic::new(cmp_inject_periodic::Config {
            period: Duration::from_secs(10),
            fn_periodic: move || {
                let msg = Message::Message0(counter);
                counter += 1.0;
                vec![msg]
            },
        }))
        .add_cmp(cmp_websocket_server::new(cmp_websocket_server::Config {
            port: 8020,
            fn_input: |msg: &Message| msg.to_json().ok(),
            fn_output: |data: &str| Message::from_json(data).ok(),
        }))
        .add_cmp(cmp_logger::new(cmp_logger::Config {
            level: Level::INFO,
            header: "".into(),
        }));

    chain.spawn().await
}
