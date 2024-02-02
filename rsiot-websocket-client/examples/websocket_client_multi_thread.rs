//! Запуск:
//!
//! ```bash
//! cargo run -p rsiot-websocket-client --example websocket_client_multi_thread
//! ```

use serde::{Deserialize, Serialize};
use tokio::{main, time::Duration};
use tracing::{error, Level};
use url::Url;

use rsiot_component_core::ComponentExecutor;
use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
use rsiot_messages_core::{msg_meta, IMessage, IMsgContentValue, MsgContent, MsgMeta};
use rsiot_websocket_client::cmp_websocket_client;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, MsgMeta)]
enum Message {
    Send(MsgContent<f64>),
    Recv(MsgContent<f64>),
    Tick(MsgContent<u64>),
}

impl IMessage for Message {
    fn into_eav(self) -> Vec<rsiot_messages_core::eav::EavModel> {
        vec![]
    }
}

#[main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "".into(),
    };

    let mut counter = 0.0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Message::Send(MsgContent::new(counter));
            counter += 1.0;
            vec![msg]
        },
    };

    let ws_client = cmp_websocket_client::Config {
        url: Url::parse("ws://localhost:9001")?,
        fn_input,
        fn_output,
    };

    ComponentExecutor::<Message>::new(100)
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .add_cmp(cmp_websocket_client::Cmp::new(ws_client))
        .wait_result()
        .await?;

    Ok(())
}

fn fn_input(msg: &Message) -> Option<String> {
    let text = msg.to_json();
    let text = match text {
        Ok(val) => val,
        Err(err) => {
            error!("{}", err);
            return None;
        }
    };
    match msg {
        Message::Send(_) => Some(text),
        Message::Recv(_) => None,
        Message::Tick(_) => None,
    }
}

fn fn_output(data: &str) -> Result<Vec<Message>, anyhow::Error> {
    // сообщение tick ...
    if let Some(val) = parse_tick(data) {
        return Ok(vec![val]);
    }
    if let Ok(msg) = Message::from_json(data) {
        match msg {
            Message::Send(val) => return Ok(vec![Message::Recv(val)]),
            Message::Recv(_) => return Ok(vec![]),
            Message::Tick(_) => return Ok(vec![]),
        }
    }
    Ok(vec![])
}

fn parse_tick(data: &str) -> Option<Message> {
    let parts: Vec<&str> = data.split(' ').collect();
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
    Some(Message::Tick(MsgContent::new(num)))
}
