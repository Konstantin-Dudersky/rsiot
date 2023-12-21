//! Преобразование mpsc::Receiver в broadcast::Sender

use tokio::{
    sync::{
        broadcast::{self, error::SendError},
        mpsc,
    },
    time::{sleep, Duration},
};
use tracing::{error, info};

use rsiot_component_core::StreamInput;
use rsiot_messages_core::IMessage;

/// Компонент для перенаправления сообщений из `tokio::sync::mpsc` в `tokio::sync::broadcast`
pub async fn new<TMessage>(input: StreamInput<TMessage>, output: broadcast::Sender<TMessage>)
where
    TMessage: IMessage,
{
    info!("cmpbase_mpsc_to_broadcast started");
    let mut input = match input {
        Some(val) => val,
        None => {
            let msg = "Input stream not set";
            error!("{}", msg);
            return;
        }
    };
    loop {
        let result = loop_(&mut input, &output).await;
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
