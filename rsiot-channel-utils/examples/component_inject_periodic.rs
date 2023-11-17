use serde::{Deserialize, Serialize};
use tokio::{main, spawn, sync::mpsc, time::Duration};

use rsiot_channel_utils::{component_inject_periodic, component_logger};
use rsiot_messages_core::IMessage;
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
    tracing_subscriber::fmt().init();

    let (stream_origin, stream_into_logger) = mpsc::channel::<Message>(10);

    let mut counter = 0.0;
    let task_inject_periodic = spawn(component_inject_periodic(
        stream_origin,
        Duration::from_secs(2),
        move || {
            let msg = Message::Message0(counter);
            counter += 1.0;
            vec![msg]
        },
    ));

    let _task_logger =
        spawn(component_logger(stream_into_logger, None, Level::INFO));

    task_inject_periodic.await.unwrap();
}
