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
    Message0(f64),
    Message1(f64),
    CombineMessage(f64, f64),
    Tick(u64),
}

impl IMessage for Message {
    fn into_eav(self) -> Vec<rsiot_messages_core::eav::EavModel> {
        todo!()
    }
}

#[main]
async fn main() {
    tracing_subscriber::fmt().init();

    let mut counter = 0.0;

    let mut chain = ComponentChain::<Message>::new(100)
        .add_cmp(cmp_inject_periodic::new(cmp_inject_periodic::Config {
            period: Duration::from_secs(2),
            fn_periodic: move || {
                let msg = Message::Message0(counter);
                counter += 1.0;
                vec![msg]
            },
        }))
        .add_cmp(cmp_websocket_client::new(cmp_websocket_client::Config {
            url: Url::parse("ws://localhost:9001").unwrap(),
            fn_send,
            fn_recv,
        }))
        .add_cmp(cmp_logger::new(cmp_logger::Config {
            level: Level::INFO,
            header: "".into(),
        }));

    chain.spawn().await;
}

fn fn_send(msg: Message) -> Option<String> {
    match msg {
        Message::Message0(_) => Some(msg.to_json().unwrap()),
        Message::Message1(_) => None,
        Message::CombineMessage(_, _) => None,
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
        Ok(val) => return vec![val],
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
