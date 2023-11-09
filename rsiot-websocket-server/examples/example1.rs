use serde::{Deserialize, Serialize};
use tokio::{
    main, spawn,
    sync::mpsc,
    time::{sleep, Duration},
};
use tokio_util::sync::CancellationToken;

use rsiot_messages_core::IMessage;
use rsiot_websocket_server::component_websocket_server;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Message {
    Message0(f64),
    Message1(f64),
    CombineMessage(f64, f64),
}

impl IMessage for Message {}

#[main]
async fn main() {
    tracing_subscriber::fmt().init();

    let cancel = CancellationToken::new();

    let (msgs_origin, msgs_ws_input) = mpsc::channel::<Message>(1000);
    let (msgs_ws_output, _) = mpsc::channel::<Message>(1000);

    let mut counter = 0.0;
    let _task_origin = spawn(async move {
        loop {
            let msg = Message::Message0(counter);
            counter += 1.0;
            msgs_origin.send(msg).await.unwrap();
            sleep(Duration::from_secs(10)).await;
        }
    });

    let ws_task = spawn(component_websocket_server(
        cancel,
        msgs_ws_input,
        msgs_ws_output,
        8020,
    ));

    ws_task.await.unwrap();
}
