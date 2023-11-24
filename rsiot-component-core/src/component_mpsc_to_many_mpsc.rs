//! Преобразование одного mpsc::Receiver в несколько mpsc::Sender

use tokio::{
    sync::mpsc::{self, error::SendError},
    time::{sleep, Duration},
};
use tracing::{error, info};

use rsiot_messages_core::IMessage;

pub async fn component_mpsc_to_many_mpsc<TMessage>(
    mut channel_rcv: mpsc::Receiver<TMessage>,
    channels_send: Vec<mpsc::Sender<TMessage>>,
) where
    TMessage: IMessage,
{
    info!("Component component_mpsc_to_many_mpsc started");
    loop {
        let result = loop_(&mut channel_rcv, &channels_send).await;
        match result {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        sleep(Duration::from_secs(2)).await;
        info!("Restarting");
    }
}

async fn loop_<TMessage>(
    channel_rcv: &mut mpsc::Receiver<TMessage>,
    channels_send: &Vec<mpsc::Sender<TMessage>>,
) -> Result<(), SendError<TMessage>>
where
    TMessage: IMessage,
{
    while let Some(msg) = channel_rcv.recv().await {
        for ch in channels_send {
            ch.send(msg.clone()).await?;
        }
    }
    Ok(())
}
