use std::net::SocketAddr;

use futures_util::SinkExt;
use tokio::{net::TcpStream, sync::broadcast};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tracing::{error, info};

use rsiot_channel_utils::CacheType;
use rsiot_messages_core::IMessage;

use crate::Errors;

/// Создание и управление подключением websocket
pub async fn handle_ws_connection<TMessage>(
    raw_stream: TcpStream,
    addr: SocketAddr,
    rx: broadcast::Receiver<TMessage>,
    cache: CacheType<TMessage>,
) -> ()
where
    TMessage: IMessage,
{
    let result = _handle_ws_connection(raw_stream, addr, rx, cache).await;
    match result {
        Ok(_) => (),
        Err(err) => {
            error!("Websocket client from address: {}, error: {:?}", addr, err)
        }
    }
}

async fn _handle_ws_connection<TMessage>(
    raw_stream: TcpStream,
    addr: SocketAddr,
    mut rx: broadcast::Receiver<TMessage>,
    cache: CacheType<TMessage>,
) -> Result<(), Errors>
where
    TMessage: IMessage,
{
    info!("Incoming TCP connection from: {}", addr);
    let mut ws_stream = accept_async(raw_stream).await?;
    info!("WebSocket connection established: {:?}", addr);

    let local_cache: Vec<TMessage>;
    // отправляем для нового клиента все сообщение из кеша
    {
        let lock = cache.lock().await;
        local_cache = lock.values().map(|m| m.clone()).collect();
    }
    for msg in local_cache {
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

    // отправляем новые сообщения
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
