//! Преобразование mpsc::Receiver в broadcast::Sender

use tokio::{
    sync::{
        broadcast::{self, error::SendError},
        mpsc,
    },
    time::{sleep, Duration},
};
use tracing::{error, info};

use rsiot_messages_core::IMessage;

pub async fn component_mpsc_to_broadcast<TMessage>(
    mut channel_mpsc_rcv: mpsc::Receiver<TMessage>,
    channel_broadcast_send: broadcast::Sender<TMessage>,
) where
    TMessage: IMessage,
{
    info!("Component component_mpsc_to_broadcast started");
    loop {
        let result =
            loop_(&mut channel_mpsc_rcv, &channel_broadcast_send).await;
        match result {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        info!("Restarting...");
        sleep(Duration::from_secs(2)).await;
    }
}

async fn loop_<TMessage>(
    channel_mpsc_rcv: &mut mpsc::Receiver<TMessage>,
    channel_broadcast_send: &broadcast::Sender<TMessage>,
) -> Result<(), SendError<TMessage>> {
    while let Some(msg) = channel_mpsc_rcv.recv().await {
        channel_broadcast_send.send(msg)?;
    }
    Ok(())
}
