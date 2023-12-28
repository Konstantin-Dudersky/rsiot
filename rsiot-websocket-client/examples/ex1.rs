//! Запуск:
//!
//! ```bash
//! cargo run -p rsiot-websocket-client --example ex1
//! ```

use serde::{Deserialize, Serialize};
use tokio::{main, time::Duration};
use tracing::Level;
use url::Url;

use rsiot_component_core::ComponentChain;
use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
use rsiot_messages_core::IMessage;
use rsiot_websocket_client::cmp_websocket_client;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Message {
    MessageSend(f64),
    MessageRecv(f64),
    Tick(u64),
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

    let mut chain = ComponentChain::<Message>::new(
        100,
        vec![
            cmp_inject_periodic::new(cmp_inject_periodic::Config {
                period: Duration::from_secs(2),
                fn_periodic: move || {
                    let msg = Message::MessageSend(counter);
                    counter += 1.0;
                    vec![msg]
                },
            }),
            cmp_websocket_client::new(cmp_websocket_client::Config {
                url: Url::parse("ws://localhost:9001").unwrap(),
                fn_send,
                fn_recv,
            }),
            cmp_logger::new(cmp_logger::Config {
                level: Level::INFO,
                header: "".into(),
            }),
        ],
    );

    chain.spawn().await;
}

fn fn_send(msg: Message) -> Option<String> {
    match msg {
        Message::MessageSend(_) => Some(msg.to_json().unwrap()),
        Message::MessageRecv(_) => None,
        Message::Tick(_) => None,
    }
}

fn fn_recv(data: String) -> Vec<Message> {
    // сообщение tick ...
    match parse_tick(&data) {
        Some(val) => return vec![val],
        None => (),
    }
    match Message::from_json(&data) {
        Ok(msg) => match msg {
            Message::MessageSend(val) => return vec![Message::MessageRecv(val)],
            Message::MessageRecv(_) => return vec![],
            Message::Tick(_) => return vec![],
        },
        Err(_) => (),
    };
    vec![]
}

fn parse_tick(data: &String) -> Option<Message> {
    let parts: Vec<&str> = data.split(" ").collect();
    if parts.len() != 2 {
        return None;
    }
    if parts[0] != "tick" {
        return None;
    }
    let num: Option<u64> = parts[1].parse().ok();
    let num = match num {
        Some(val) => val,
        None => return None,
    };
    Some(Message::Tick(num))
}
