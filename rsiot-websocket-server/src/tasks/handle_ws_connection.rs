use std::net::SocketAddr;

use futures_util::SinkExt;
use tokio::{net::TcpStream, sync::broadcast};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tracing::info;

use rsiot_messages_core::IMessage;

use crate::Errors;

/// Создание и управление подключением websocket
pub async fn handle_ws_connection<TMessage>(
    raw_stream: TcpStream,
    addr: SocketAddr,
    mut rx: broadcast::Receiver<TMessage>,
) -> Result<(), Errors>
where
    TMessage: IMessage,
{
    info!("Incoming TCP connection from: {}", addr);
    let mut ws_stream = accept_async(raw_stream).await?;
    info!("WebSocket connection established: {:?}", addr);

    let msgs: Vec<TMessage> =
        load_all_messages_from_hash(redis_url, redis_channel).await?;
    for msg in msgs {
        let msg = serialize(&msg).unwrap();
        let result = ws_stream.send(Message::Text(msg)).await;
        match result {
            Ok(_) => (),
            Err(error) => {
                let error = error.to_string();
                return Err(Errors::SendToWsError(error));
            }
        };
    }

    while let Ok(msg) = rx.recv().await {
        let msg = msg.to_json().unwrap();
        let result = ws_stream.send(Message::Text(msg)).await;
        match result {
            Ok(_) => (),
            Err(error) => {
                let error = error.to_string();
                return Err(Errors::SendToWsError(error));
            }
        };
    }
    Ok(())
}
