use serde::{Deserialize, Serialize};
use tokio::{
    main, spawn,
    sync::mpsc,
    time::{sleep, Duration},
};
use tracing_subscriber::fmt;
use url::Url;

use rsiot_messages_core::IMessage;
use rsiot_redis_publisher::start_redis_publisher;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Messages {
    Message0(u16),
}

impl IMessage for Messages {}

#[main]
async fn main() {
    fmt().init();

    let url = Url::parse("redis://127.0.0.1:6379").unwrap();

    let (tx, rx) = mpsc::channel::<Messages>(128);

    let mut counter = 0;
    let _ = spawn(async move {
        loop {
            let msg = Messages::Message0(counter);
            counter += 1;
            tx.send(msg).await.unwrap();
            sleep(Duration::from_secs(2)).await;
        }
    });

    let task = spawn(start_redis_publisher(
        url,
        "rsiot-redis-publisher".to_string(),
        rx,
    ));

    task.await.unwrap();
}
