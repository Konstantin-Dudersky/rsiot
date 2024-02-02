use serde::{Deserialize, Serialize};
use tokio::{
    main, spawn,
    sync::mpsc,
    time::{sleep, Duration},
};

use rsiot_extra_components::component_combine_message;
use rsiot_messages_core::{msg_meta, IMessage, IMsgContentValue, MsgContent, MsgMeta};
use tracing::info;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct CombinedMessage(pub (f64, f64));

impl IMsgContentValue for CombinedMessage {
    fn fmt_value(&self, _template: &str) -> String {
        todo!()
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, MsgMeta)]
enum Message {
    Message0(MsgContent<f64>),
    Message1(MsgContent<f64>),
    Combine(MsgContent<CombinedMessage>),
}

impl IMessage for Message {
    fn into_eav(self) -> Vec<rsiot_messages_core::eav::EavModel> {
        vec![]
    }
}

#[main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let (in_channel_send, in_channel_rcv) = mpsc::channel::<Message>(128);
    let (out_channel_send, mut out_channel_rcv) = mpsc::channel::<Message>(128);

    let mut counter = 0.0;
    #[allow(unreachable_code)]
    let _task_sim = spawn(async move {
        loop {
            let msg = Message::Message0(MsgContent::new(counter));
            in_channel_send.send(msg).await?;
            counter += 1.0;
            if counter as u32 % 3 == 0 {
                let msg = Message::Message1(MsgContent::new(counter * 2.0));
                in_channel_send.send(msg).await?;
            }
            sleep(Duration::from_secs(2)).await;
        }
        Ok(()) as anyhow::Result<()>
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
            Some(Message::Combine(MsgContent::new(CombinedMessage((
                value1.value,
                value2.value,
            )))))
        },
    ));

    let _task_out = spawn(async move {
        while let Some(msg) = out_channel_rcv.recv().await {
            info!("Message received: {:?}", msg);
        }
    });

    main_task.await?;
    Ok(())
}
