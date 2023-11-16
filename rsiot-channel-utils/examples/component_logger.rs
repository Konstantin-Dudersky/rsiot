use serde::{Deserialize, Serialize};
use tokio::{
    main, spawn,
    sync::mpsc,
    time::{sleep, Duration},
};
use tracing::{level_filters::LevelFilter, Level};

use rsiot_channel_utils::component_logger;
use rsiot_messages_core::IMessage;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Message {
    Message0(f64),
    Message1(f64),
    CombineMessage(f64, f64),
}

impl IMessage for Message {}

#[main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .init();

    let (stream_begin, stream_logger_input) = mpsc::channel::<Message>(100);

    let mut counter = 0.0;
    let _task_begin = spawn(async move {
        loop {
            let msg = Message::Message0(counter);
            counter += 1.0;
            stream_begin.send(msg).await.unwrap();
            sleep(Duration::from_secs(2)).await;
        }
    });

    let task_logger =
        spawn(component_logger(stream_logger_input, None, Level::INFO));

    task_logger.await.unwrap();
}
