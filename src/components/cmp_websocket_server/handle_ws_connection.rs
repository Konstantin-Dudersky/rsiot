//! Создание и управление подключением между сервером и клиентом

use std::net::SocketAddr;

use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::{net::TcpStream, sync::mpsc, task::JoinSet};
use tokio_tungstenite::{
    accept_async, tungstenite::Message as TungsteniteMessage, WebSocketStream,
};
use tracing::{debug, info, trace, warn};

use crate::{
    executor::CmpInOut,
    message::{system_messages::*, *},
};

use super::{
    config::{Config, FnOutput},
    errors::Error,
};

/// Создание и управление подключением между сервером и клиентом
pub async fn handle_ws_connection<TMsg, TService>(
    input: CmpInOut<TMsg, TService>,
    config: Config<TMsg>,
    stream_and_addr: (TcpStream, SocketAddr),
) where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
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

async fn _handle_ws_connection<TMsg, TService>(
    in_out: CmpInOut<TMsg, TService>,
    stream_and_addr: (TcpStream, SocketAddr),
    config: Config<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    info!("Incoming TCP connection from: {}", stream_and_addr.1);
    let ws_stream = accept_async(stream_and_addr.0).await?;
    let (ws_stream_write, ws_stream_read) = ws_stream.split();
    info!("WebSocket connection established: {:?}", stream_and_addr.1);

    let (send_to_client_tx, send_to_client_rx) = mpsc::channel::<Message<TMsg>>(100);

    let mut set = JoinSet::new();

    // Подготавливаем кеш для отправки
    set.spawn(send_prepare_cache(
        in_out.clone(),
        send_to_client_tx.clone(),
    ));
    // Подготавливаем новые сообщения для отправки
    set.spawn(send_prepare_new_msgs(
        in_out.clone(),
        send_to_client_tx.clone(),
    ));
    // Отправляем клиенту
    set.spawn(send_to_client(
        send_to_client_rx,
        ws_stream_write,
        config.fn_input,
    ));
    // Получаем данные от клиента
    set.spawn(recv_from_client(
        ws_stream_read,
        in_out,
        config.fn_output,
        send_to_client_tx.clone(),
    ));

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
async fn send_prepare_cache<TMsg, TService>(
    mut in_out: CmpInOut<TMsg, TService>,
    output: mpsc::Sender<Message<TMsg>>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
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
async fn send_prepare_new_msgs<TMsg, TService>(
    mut input: CmpInOut<TMsg, TService>,
    output: mpsc::Sender<Message<TMsg>>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    debug!("Sending messages to client started");
    while let Ok(msg) = input.recv_input().await {
        output.send(msg).await?;
    }
    warn!("Sending messages to client complete");
    Ok(())
}

/// Отправляем данные клиенту
async fn send_to_client<TMsg>(
    mut input: mpsc::Receiver<Message<TMsg>>,
    mut ws_stream_output: SplitSink<WebSocketStream<TcpStream>, TungsteniteMessage>,
    fn_input: fn(&Message<TMsg>) -> anyhow::Result<Option<String>>,
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
async fn recv_from_client<TMsg, TService>(
    mut ws_stream_read: SplitStream<WebSocketStream<TcpStream>>,
    in_out: CmpInOut<TMsg, TService>,
    fn_output: FnOutput<TMsg>,
    send_to_client_tx: mpsc::Sender<Message<TMsg>>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    while let Some(data) = ws_stream_read.next().await {
        let data = data?.into_text()?;
        if data.is_empty() {
            return Err(Error::ClientDisconnected);
        }
        let msgs = (fn_output)(&data).map_err(|err| Error::FnOutput { err, data })?;
        let Some(msgs) = msgs else { continue };
        for msg in msgs {
            trace!(
                "New message from websocket client, send to internal bus: {:?}",
                msg
            );
            // Отправляем клиенту сообщение Pong для контроля связи
            if let MsgData::System(System::Ping(value)) = &msg.data {
                let value = Pong { count: value.count };
                let pong_msg = Message::new(MsgData::System(System::Pong(value)));
                send_to_client_tx.send(pong_msg).await.unwrap();
                continue;
            }
            in_out.send_output(msg).await.map_err(Error::CmpOutput)?;
        }
    }
    debug!("Input stream from client closed");
    Ok(())
}
