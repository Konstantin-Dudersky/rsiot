use std::net::SocketAddr;

use futures_util::SinkExt;
use tokio::{net::TcpStream, sync::broadcast};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};
use tracing::{info, warn};

use rsiot_extra_components::cmp_cache::CacheType;
use rsiot_messages_core::IMessage;

use crate::errors::Errors;

/// Создание и управление подключением websocket
pub async fn handle_ws_connection<TMessage>(
    stream_and_addr: (TcpStream, SocketAddr),
    input: broadcast::Receiver<TMessage>,
    cache: CacheType<TMessage>,
    fn_senf_to_client: fn(TMessage) -> Option<String>,
) where
    TMessage: IMessage + 'static,
{
    let addr = stream_and_addr.1.clone();
    let result =
        _handle_ws_connection(input, stream_and_addr, cache, fn_senf_to_client)
            .await;
    match result {
        Ok(_) => (),
        Err(err) => {
            warn!("Websocket client from address: {}, error: {:?}", addr, err)
        }
    }
}

async fn _handle_ws_connection<TMessage>(
    input: broadcast::Receiver<TMessage>,
    stream_and_addr: (TcpStream, SocketAddr),
    cache: CacheType<TMessage>,
    fn_senf_to_client: fn(TMessage) -> Option<String>,
) -> Result<(), Errors>
where
    TMessage: IMessage + 'static,
{
    info!("Incoming TCP connection from: {}", stream_and_addr.1);
    let mut ws_stream = accept_async(stream_and_addr.0).await?;
    info!("WebSocket connection established: {:?}", stream_and_addr.1);

    let _send_cache =
        send_cache(&mut ws_stream, fn_senf_to_client, cache.clone()).await?;

    let _send_new_msgs =
        send_new_msgs(input, &mut ws_stream, fn_senf_to_client).await?;

    Ok(())
}

/// При подключении нового клиента отправляем все данные из кеша
async fn send_cache<TMessage>(
    ws_stream_output: &mut WebSocketStream<TcpStream>,
    fn_senf_to_client: fn(TMessage) -> Option<String>,
    cache: CacheType<TMessage>,
) -> Result<(), Errors>
where
    TMessage: IMessage,
{
    let local_cache: Vec<TMessage>;
    {
        let lock = cache.lock().await;
        local_cache = lock.values().cloned().collect();
    }
    for msg in local_cache {
        let msg = (fn_senf_to_client)(msg);
        let msg = match msg {
            Some(val) => val,
            None => continue,
        };
        let result = ws_stream_output.send(Message::Text(msg)).await;
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

/// При получении новых сообщений, отправляем клиенту
async fn send_new_msgs<TMessage>(
    mut input: broadcast::Receiver<TMessage>,
    ws_stream_output: &mut WebSocketStream<TcpStream>,
    fn_senf_to_client: fn(TMessage) -> Option<String>,
) -> Result<(), Errors>
where
    TMessage: IMessage,
{
    while let Ok(msg) = input.recv().await {
        let msg = (fn_senf_to_client)(msg);
        let msg = match msg {
            Some(val) => val,
            None => continue,
        };
        let result = ws_stream_output.send(Message::Text(msg)).await;
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
