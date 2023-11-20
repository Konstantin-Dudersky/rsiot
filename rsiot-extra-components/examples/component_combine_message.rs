use serde::{Deserialize, Serialize};
use tokio::{
    main, spawn,
    sync::mpsc,
    time::{sleep, Duration},
};

use rsiot_extra_components::component_combine_message;
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

    let (in_channel_send, in_channel_rcv) = mpsc::channel::<Message>(128);
    let (out_channel_send, mut out_channel_rcv) = mpsc::channel::<Message>(128);

    let mut counter = 0.0;
    let _task_sim = spawn(async move {
        loop {
            let msg = Message::Message0(counter);
            in_channel_send.send(msg).await.unwrap();
            counter += 1.0;
            if counter as u32 % 3 == 0 {
                let msg = Message::Message1(counter * 2.0);
                in_channel_send.send(msg).await.unwrap();
            }
            sleep(Duration::from_secs(2)).await;
        }
    });

    let main_task = spawn(component_combine_message(
        in_channel_rcv,
        out_channel_send,
        |msg| match msg {
            Message::Message0(_) | Message::Message1(_) => Some(msg),
            _ => None,
        },
        |msgs| {
            let mut value1 = None;
            let mut value2 = None;
            for msg in msgs {
                match msg {
                    Message::Message0(value) => value1 = Some(value),
                    Message::Message1(value) => value2 = Some(value),
                    _ => (),
                }
            }
            let value1 = match value1 {
                Some(val) => val,
                None => return None,
            };
            let value2 = match value2 {
                Some(val) => val,
                None => return None,
            };
            Some(Message::CombineMessage(value1, value2))
        },
    ));

    let _task_out = spawn(async move {
        while let Some(msg) = out_channel_rcv.recv().await {
            info!("Message received: {:?}", msg);
        }
    });

    main_task.await.unwrap();
}