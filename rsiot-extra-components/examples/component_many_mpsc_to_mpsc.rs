use serde::{Deserialize, Serialize};
use tokio::{
    main, spawn,
    sync::mpsc,
    time::{sleep, Duration},
};

use rsiot_extra_components::cmpbase_many_mpsc_to_mpsc;
use rsiot_messages_core::IMessage;
use tracing::info;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
enum Message {
    Message0(f64),
    Message1(f64),
    Combine(f64, f64),
}

impl IMessage for Message {
    fn into_eav(self) -> Vec<rsiot_messages_core::eav::EavModel> {
        vec![]
    }
}

#[main]
async fn main() {
    tracing_subscriber::fmt().init();

    let (stream1_tx, steam1_rx) = mpsc::channel::<Message>(100);
    let (stream2_tx, steam2_rx) = mpsc::channel::<Message>(100);
    let (stream_out_tx, mut stream_out_rx) = mpsc::channel::<Message>(100);

    let mut counter1 = 0.0;
    let _task1 = spawn(async move {
        loop {
            let msg = Message::Message0(counter1);
            counter1 += 1.0;
            stream1_tx.send(msg).await.unwrap();
            sleep(Duration::from_secs(1)).await;
        }
    });

    let mut counter2 = 0.0;
    let _task2 = spawn(async move {
        loop {
            let msg = Message::Message1(counter2);
            counter2 += 1.0;
            stream2_tx.send(msg).await.unwrap();
            sleep(Duration::from_secs(2)).await;
        }
    });

    let main_task = spawn(cmpbase_many_mpsc_to_mpsc::new::<Message>(
        vec![steam1_rx, steam2_rx],
        stream_out_tx,
    ));

    let _end_task = spawn(async move {
        while let Some(msg) = stream_out_rx.recv().await {
            info!("New message: {:?}", msg);
        }
    });

    main_task.await.unwrap();
}
