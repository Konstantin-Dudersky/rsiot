use serde::{Deserialize, Serialize};
use tokio::{
    main, spawn,
    sync::mpsc,
    time::{sleep, Duration},
};

use rsiot_channel_utils::component_mpsc_to_many_mpsc;
use rsiot_messages_core::IMessage;
use tracing::info;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Message {
    Message0(f64),
}

impl IMessage for Message {}

#[main]
async fn main() {
    tracing_subscriber::fmt().init();

    let (mpsc_send, mpsc_rcv) = mpsc::channel::<Message>(128);

    let (mpsc_send_1, mut mpsc_rcv_1) = mpsc::channel::<Message>(128);
    let (mpsc_send_2, mut mpsc_rcv_2) = mpsc::channel::<Message>(128);

    let mut counter = 0.0;
    let _source_task = spawn(async move {
        loop {
            let msg = Message::Message0(counter);
            counter += 1.0;
            mpsc_send.send(msg).await.unwrap();
            sleep(Duration::from_secs(2)).await;
        }
    });

    let main_task = spawn(component_mpsc_to_many_mpsc(
        mpsc_rcv,
        vec![mpsc_send_1, mpsc_send_2],
    ));

    let _end_task_1 = spawn(async move {
        while let Some(res) = mpsc_rcv_1.recv().await {
            info!("end_task_1: {:?}", res)
        }
    });

    let _end_task_2 = spawn(async move {
        while let Some(res) = mpsc_rcv_2.recv().await {
            info!("end_task_2: {:?}", res)
        }
    });

    main_task.await.unwrap();
}
