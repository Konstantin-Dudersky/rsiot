use serde::{Deserialize, Serialize};
use tokio::{
    main, spawn,
    sync::mpsc,
    time::{sleep, Duration},
};

use rsiot_extra_components::component_delay;
use rsiot_messages_core::IMessage;
use tracing::info;

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

    let (stream_origin, stream_delay_input) = mpsc::channel::<Message>(100);
    let (stream_delay_output, mut stream_end) = mpsc::channel::<Message>(100);

    let mut counter = 0.0;
    let _task_origin = spawn(async move {
        loop {
            let msg = Message::Message0(counter);
            stream_origin.send(msg).await.unwrap();
            counter += 1.0;
            sleep(Duration::from_millis(10)).await;
        }
    });

    let main_task = spawn(component_delay(
        stream_delay_input,
        stream_delay_output,
        Duration::from_secs(5),
    ));

    let _end_task = spawn(async move {
        while let Some(msg) = stream_end.recv().await {
            info!("Dalayed message: {:?}", msg);
        }
    });

    main_task.await.unwrap();
}
