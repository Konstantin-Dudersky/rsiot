//! Создание и управление подключением между сервером и клиентом

use std::net::SocketAddr;

use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use rsiot_component_core::{Cache, CmpInput, CmpOutput};
use tokio::{net::TcpStream, sync::mpsc, task::JoinSet};
use tokio_tungstenite::{
    accept_async, tungstenite::Message as TungsteniteMessage, WebSocketStream,
};
use tracing::{debug, info, trace, warn};

use rsiot_messages_core::message_v2::{Message, MsgDataBound};

use crate::{config::Config, errors::Error};

/// Создание и управление подключением между сервером и клиентом
pub async fn handle_ws_connection<TMessage>(
    input: CmpInput<TMessage>,
    output: CmpOutput<TMessage>,
    config: Config<TMessage>,
    stream_and_addr: (TcpStream, SocketAddr),
    cache: Cache<TMessage>,
) where
    TMessage: MsgDataBound + 'static,
{
    let addr = stream_and_addr.1;
    let result = _handle_ws_connection(input, output, stream_and_addr, config, cache).await;
    match result {
        Ok(_) => (),
        Err(err) => {
            warn!("Websocket client from address: {}, error: {}", addr, err)
        }
    }
    info!("Connection closed");
}

async fn _handle_ws_connection<TMessage>(
    input: CmpInput<TMessage>,
    output: CmpOutput<TMessage>,
    stream_and_addr: (TcpStream, SocketAddr),
    config: Config<TMessage>,
    cache: Cache<TMessage>,
) -> crate::Result<()>
where
    TMessage: MsgDataBound + 'static,
{
    info!("Incoming TCP connection from: {}", stream_and_addr.1);
    let ws_stream = accept_async(stream_and_addr.0).await?;
    let (write, read) = ws_stream.split();
    info!("WebSocket connection established: {:?}", stream_and_addr.1);

    let (prepare_tx, prepare_rx) = mpsc::channel::<Message<TMessage>>(100);

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
        let err = match res {
            Ok(val) => match val {
                Ok(_) => continue,
                Err(err) => format!("{}", err),
            },
            Err(err) => format!("{}", err),
        };
        warn!("Connection error: {}", err);
        set.shutdown().await;
    }
    Ok(())
}

/// При подключении нового клиента отправляем все данные из кеша
async fn send_prepare_cache<TMessage>(
    output: mpsc::Sender<Message<TMessage>>,
    cache: Cache<TMessage>,
) -> crate::Result<()>
where
    TMessage: MsgDataBound,
{
    debug!("Sending cache to client started");
    let local_cache: Vec<Message<TMessage>>;
    {
        let lock = cache.read().await;
        local_cache = lock.values().cloned().collect();
    }
    for msg in local_cache {
        output.send(msg).await?;
    }
    debug!("Sending cache to client complete");
    Ok(())
}

/// При получении новых сообщений, отправляем клиенту
async fn send_prepare_new_msgs<TMessage>(
    mut input: CmpInput<TMessage>,
    output: mpsc::Sender<Message<TMessage>>,
) -> crate::Result<()>
where
    TMessage: MsgDataBound,
{
    debug!("Sending messages to client started");
    while let Ok(msg) = input.recv().await {
        let msg = match msg {
            Some(val) => val,
            None => continue,
        };
        output.send(msg).await?;
    }
    warn!("Sending messages to client complete");
    Ok(())
}

/// Отправляем данные клиенту
async fn send_to_client<TMessage>(
    mut input: mpsc::Receiver<Message<TMessage>>,
    mut ws_stream_output: SplitSink<WebSocketStream<TcpStream>, TungsteniteMessage>,
    fn_input: fn(&Message<TMessage>) -> anyhow::Result<Option<String>>,
) -> crate::Result<()> {
    while let Some(msg) = input.recv().await {
        let text = (fn_input)(&msg).map_err(Error::FnInput)?;
        let text = match text {
            Some(val) => val,
            None => continue,
        };
        trace!("Send message to client: {:?}", text);
        let text = TungsteniteMessage::Text(text);
        ws_stream_output.send(text).await?;
    }
    ws_stream_output.close().await.unwrap();
    debug!("Internal channel for sending to client closed");
    Ok(())
}

/// Получение данных от клиента
async fn recv_from_client<TMsg>(
    mut ws_stream_input: SplitStream<WebSocketStream<TcpStream>>,
    output: CmpOutput<TMsg>,
    fn_output: fn(&str) -> anyhow::Result<Option<Vec<Message<TMsg>>>>,
) -> crate::Result<()>
where
    TMsg: MsgDataBound,
{
    while let Some(data) = ws_stream_input.next().await {
        let data = data?.into_text()?;
        if data.is_empty() {
            return Err(Error::ClientDisconnected);
        }
        let msgs = (fn_output)(&data).map_err(|err| Error::FnOutput { err, data })?;
        let msgs = match msgs {
            Some(val) => val,
            None => continue,
        };
        for msg in msgs {
            trace!(
                "New message from websocket client, send to internal bus: {:?}",
                msg
            );
            output.send(msg).await.map_err(Error::CmpOutput)?;
        }
    }
    debug!("Input stream from client closed");
    Ok(())
}
