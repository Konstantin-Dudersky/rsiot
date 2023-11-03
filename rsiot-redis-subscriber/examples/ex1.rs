use serde::{Deserialize, Serialize};
use tokio::{main, spawn, sync::mpsc};
use tracing_subscriber::fmt;
use url::Url;

use rsiot_messages_core::IMessage;
use rsiot_redis_subscriber::start_redis_subscriber;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Messages {
    Message0,
}

impl IMessage for Messages {}

#[main]
async fn main() {
    fmt().init();

    let url = Url::parse("redis://127.0.0.1:6379").unwrap();

    let (tx, mut rx) = mpsc::channel::<Messages>(128);

    let task = spawn(start_redis_subscriber(
        url,
        "rsiot-redis-subscriber".to_string(),
        tx,
    ));

    let _print = spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("New message: {:?}", msg);
        }
    });

    task.await.unwrap();
}
