//! Создание и управление подключением между сервером и клиентом

use std::net::SocketAddr;

use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use rsiot_component_core::{Cache, ComponentInput, ComponentOutput};
use tokio::{net::TcpStream, sync::mpsc, task::JoinSet};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};
use tracing::{debug, info, trace, warn};

use rsiot_messages_core::IMessage;

use crate::{config::Config, errors::Error};

/// Создание и управление подключением между сервером и клиентом
pub async fn handle_ws_connection<TMessage>(
    input: ComponentInput<TMessage>,
    output: ComponentOutput<TMessage>,
    config: Config<TMessage>,
    stream_and_addr: (TcpStream, SocketAddr),
    cache: Cache<TMessage>,
) where
    TMessage: IMessage + 'static,
{
    let addr = stream_and_addr.1;
    let result = _handle_ws_connection(input, output, stream_and_addr, config, cache).await;
    match result {
        Ok(_) => (),
        Err(err) => {
            warn!("Websocket client from address: {}, error: {}", addr, err)
        }
    }
}

async fn _handle_ws_connection<TMessage>(
    input: ComponentInput<TMessage>,
    output: ComponentOutput<TMessage>,
    stream_and_addr: (TcpStream, SocketAddr),
    config: Config<TMessage>,
    cache: Cache<TMessage>,
) -> crate::Result<(), TMessage>
where
    TMessage: IMessage + 'static,
{
    info!("Incoming TCP connection from: {}", stream_and_addr.1);
    let ws_stream = accept_async(stream_and_addr.0).await?;
    let (write, read) = ws_stream.split();
    info!("WebSocket connection established: {:?}", stream_and_addr.1);

    let (prepare_tx, prepare_rx) = mpsc::channel::<TMessage>(100);

    let mut set = JoinSet::new();

    // Подготавливаем кеш для отправки
    set.spawn(send_prepare_cache(prepare_tx.clone(), cache.clone()));
    // Подготавливаем новые сообщения для отправки
    set.spawn(send_prepare_new_msgs(input, prepare_tx.clone()));
    // Отправляем клиенту
    set.spawn(send_to_client(prepare_rx, write, config.fn_input));
    // Получаем данные от клиента
    set.spawn(recv_from_client(read, output, config.fn_output));

    while let Some(res) = set.join_next().await {
        res??;
    }
    Ok(())
}

/// При подключении нового клиента отправляем все данные из кеша
async fn send_prepare_cache<TMessage>(
    output: mpsc::Sender<TMessage>,
    cache: Cache<TMessage>,
) -> crate::Result<(), TMessage>
where
    TMessage: IMessage,
{
    let local_cache: Vec<TMessage>;
    {
        let lock = cache.read().await;
        local_cache = lock.values().cloned().collect();
    }
    for msg in local_cache {
        output.send(msg).await?;
    }
    Ok(())
}

/// При получении новых сообщений, отправляем клиенту
async fn send_prepare_new_msgs<TMessage>(
    mut input: ComponentInput<TMessage>,
    output: mpsc::Sender<TMessage>,
) -> crate::Result<(), TMessage>
where
    TMessage: IMessage,
{
    while let Ok(msg) = input.recv().await {
        output.send(msg).await?;
    }
    Ok(())
}

/// Отправляем данные клиенту
async fn send_to_client<TMessage>(
    mut input: mpsc::Receiver<TMessage>,
    mut ws_stream_output: SplitSink<WebSocketStream<TcpStream>, Message>,
    fn_input: fn(&TMessage) -> anyhow::Result<Option<String>>,
) -> crate::Result<(), TMessage> {
    while let Some(msg) = input.recv().await {
        let msg = (fn_input)(&msg).map_err(Error::FnInput)?;
        let data = match msg {
            Some(val) => val,
            None => continue,
        };
        trace!("Send to client: {:?}", data);
        ws_stream_output.send(Message::Text(data)).await?;
    }
    debug!("Internal channel for sending to client closed");
    Ok(())
}

/// Получение данных от клиента
async fn recv_from_client<TMessage>(
    mut ws_stream_input: SplitStream<WebSocketStream<TcpStream>>,
    output: mpsc::Sender<TMessage>,
    fn_output: fn(&str) -> anyhow::Result<Option<TMessage>>,
) -> crate::Result<(), TMessage>
where
    TMessage: IMessage,
{
    while let Some(data) = ws_stream_input.next().await {
        let data = data?.into_text()?;
        if data.is_empty() {
            return Err(Error::ClientDisconnected);
        }
        let msg = (fn_output)(&data).map_err(|err| Error::FnOutput { err, data })?;
        let msg = match msg {
            Some(val) => val,
            None => continue,
        };
        output.send(msg).await?;
    }
    debug!("Input stream from client closed");
    Ok(())
}
