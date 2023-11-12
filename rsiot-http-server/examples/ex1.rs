use serde::{Deserialize, Serialize};
use tokio::{
    main, spawn,
    sync::mpsc,
    time::{sleep, Duration},
};

use rsiot_http_server::component_http_server;
use rsiot_messages_core::IMessage;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

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
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let (stream_origin, stream_http_input) = mpsc::channel::<Message>(100);
    let (stream_http_output, mut stream_end) = mpsc::channel::<Message>(100);

    let mut counter = 0.0;
    let _origin_task = spawn(async move {
        loop {
            let msg = Message::Message0(counter);
            counter += 1.0;
            stream_origin.send(msg).await.unwrap();
            sleep(Duration::from_secs(2)).await;
        }
    });

    let main_task = spawn(component_http_server(
        stream_http_input,
        stream_http_output,
        8011,
    ));

    let _end_task = spawn(async move {
        while let Some(msg) = stream_end.recv().await {
            info!("New message from HTTP: {:?}", msg);
        }
    });

    main_task.await.unwrap();
}
