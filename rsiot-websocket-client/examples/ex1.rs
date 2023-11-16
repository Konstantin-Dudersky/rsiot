use serde::{Deserialize, Serialize};
use tokio::{main, spawn, sync::mpsc};

use rsiot_channel_utils::component_logger;
use rsiot_messages_core::IMessage;
use rsiot_websocket_client::component_websocket_client;
use tracing::Level;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Message {
    Message0(f64),
    Message1(f64),
    CombineMessage(f64, f64),
}

impl IMessage for Message {}

#[main]
async fn main() {
    let (stream_begin, stream_ws_client_input) = mpsc::channel::<Message>(100);
    let (stream_ws_client_output, stream_end) = mpsc::channel::<Message>(100);

    let task_ws_client = spawn(component_websocket_client(
        stream_ws_client_input,
        stream_ws_client_output,
        |data| {
            println!("{:?}", data);
            vec![]
        },
    ));

    let _task_logger = spawn(component_logger(stream_end, None, Level::INFO));

    task_ws_client.await.unwrap();
}
