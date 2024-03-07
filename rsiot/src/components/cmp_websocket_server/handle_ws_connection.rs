//! Создание и управление подключением между сервером и клиентом

use std::net::SocketAddr;

use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use rsiot_component_core::CmpInOut;
use tokio::{net::TcpStream, sync::mpsc, task::JoinSet};
use tokio_tungstenite::{
    accept_async, tungstenite::Message as TungsteniteMessage, WebSocketStream,
};
use tracing::{debug, info, trace, warn};

use rsiot_messages_core::{system_messages::*, *};

use super::{
    config::{Config, FnOutput},
    errors::Error,
};

/// Создание и управление подключением между сервером и клиентом
pub async fn handle_ws_connection<TMessage>(
    input: CmpInOut<TMessage>,
    config: Config<TMessage>,
    stream_and_addr: (TcpStream, SocketAddr),
) where
    TMessage: MsgDataBound + 'static,
{
    let addr = stream_and_addr.1;
    let result = _handle_ws_connection(input, stream_and_addr, config).await;
    match result {
        Ok(_) => (),
        Err(err) => {
            warn!("Websocket client from address: {}, error: {}", addr, err)
        }
    }
    info!("Connection closed");
}

async fn _handle_ws_connection<TMessage>(
    in_out: CmpInOut<TMessage>,
    stream_and_addr: (TcpStream, SocketAddr),
    config: Config<TMessage>,
) -> super::Result<()>
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
    set.spawn(send_prepare_cache(in_out.clone(), prepare_tx.clone()));
    // Подготавливаем новые сообщения для отправки
    set.spawn(send_prepare_new_msgs(in_out.clone(), prepare_tx.clone()));
    // Отправляем клиенту
    set.spawn(send_to_client(prepare_rx, write, config.fn_input));
    // Получаем данные от клиента
    set.spawn(recv_from_client(read, in_out, config.fn_output));

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
async fn send_prepare_cache<TMsg>(
    mut in_out: CmpInOut<TMsg>,
    output: mpsc::Sender<Message<TMsg>>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    loop {
        debug!("Sending cache to client started");
        let local_cache = in_out.recv_cache_all().await;
        for msg in local_cache {
            output.send(msg).await?;
        }
        debug!("Sending cache to client complete");
        // При изменении доступа к системе, отправляем данные снова
        'auth_changed: while let Ok(msg) = in_out.recv_input().await {
            match msg.data {
                MsgData::System(System::AuthResponseOk(_)) => break 'auth_changed,
                _ => continue,
            }
        }
    }
}

/// При получении новых сообщений, отправляем клиенту
async fn send_prepare_new_msgs<TMessage>(
    mut input: CmpInOut<TMessage>,
    output: mpsc::Sender<Message<TMessage>>,
) -> super::Result<()>
where
    TMessage: MsgDataBound,
{
    debug!("Sending messages to client started");
    while let Ok(msg) = input.recv_input().await {
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
) -> super::Result<()> {
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
    output: CmpInOut<TMsg>,
    fn_output: FnOutput<TMsg>,
) -> super::Result<()>
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
            output.send_output(msg).await.map_err(Error::CmpOutput)?;
        }
    }
    debug!("Input stream from client closed");
    Ok(())
}
