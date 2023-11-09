use serde::{Deserialize, Serialize};
use tokio::{
    main, spawn,
    sync::mpsc,
    time::{sleep, Duration},
};

use rsiot_channel_utils::component_filter_message;
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

    let (original_send, original_rcv) = mpsc::channel::<Message>(128);
    let (filtered_send, mut filtered_rcv) = mpsc::channel::<Message>(128);

    let mut counter = 0.0;
    let _sim_task = spawn(async move {
        loop {
            let msg = Message::Message0(counter);
            counter += 1.0;
            info!("send msg: {:?}", msg);
            original_send.send(msg).await.unwrap();

            let msg = Message::Message1(counter);
            info!("send msg: {:?}", msg);
            original_send.send(msg).await.unwrap();

            sleep(Duration::from_secs(2)).await;
        }
    });

    let main_task = spawn(component_filter_message(
        original_rcv,
        filtered_send,
        |msg| match msg {
            Message::Message0(_) => Some(msg),
            _ => None,
        },
    ));

    let _end_task = spawn(async move {
        while let Some(msg) = filtered_rcv.recv().await {
            info!("rcv msg: {:?}", msg);
        }
    });

    main_task.await.unwrap();
}
